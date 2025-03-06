#![warn(clippy::all, clippy::pedantic, clippy::nursery)]

#[cfg(feature = "device_query")]
use device_query::{DeviceQuery, DeviceState, MouseState};
use std::io::{stdout, IsTerminal, Write};
use termcolor::{Color, ColorChoice, ColorSpec, StandardStream, WriteColor};
use xcap::Monitor;
#[cfg(feature = "mouce")]
use {
    mouce::{
        common::{MouseButton, MouseEvent},
        Mouse, MouseActions,
    },
    std::sync::mpsc,
};

fn main() {
    let Some(coordinates) = request_pixel_position() else {
        println!("Ending program");
        return;
    };

    let (r, g, b) = get_pixel_colour(coordinates);

    let rgb_hex = format!("#{r:02X}{g:02X}{b:02X}");

    if stdout().is_terminal() {
        print_color_result((r, g, b), &rgb_hex);
    } else {
        println!("{rgb_hex}");
    }

    send_to_clibpoard(rgb_hex);
}

#[cfg(feature = "device_query")]
fn request_pixel_position() -> Option<(i32, i32)> {
    let device_state = DeviceState::new();

    loop {
        let mouse: MouseState = device_state.get_mouse();

        if mouse.button_pressed[1] {
            return Some(mouse.coords);
        }

        if mouse.button_pressed[3] {
            return None;
        }
    }
}

#[cfg(feature = "mouce")]
fn request_pixel_position() -> Option<(i32, i32)> {
    let mut mouse_manager = Mouse::new();

    let (sender, receiver) = mpsc::channel();

    let callback_id = mouse_manager
        .hook(Box::new(move |x| match x {
            MouseEvent::Press(MouseButton::Left) => sender.send(true).unwrap(),
            MouseEvent::Press(MouseButton::Right) => sender.send(false).unwrap(),
            _ => (),
        }))
        .unwrap();

    let accepted = receiver.recv().unwrap();
    mouse_manager.unhook(callback_id).unwrap();

    if accepted {
        Some(mouse_manager.get_position().unwrap())
    } else {
        None
    }
}

fn get_pixel_colour((x, y): (i32, i32)) -> (u8, u8, u8) {
    let screen = Monitor::from_point(x, y).unwrap();

    let screenshot = screen.capture_image().unwrap();

    let pixel = screenshot.get_pixel(
        (x - screen.x().unwrap()).try_into().unwrap(),
        (y - screen.y().unwrap()).try_into().unwrap(),
    );

    (pixel[0], pixel[1], pixel[2])
}

fn print_color_result((r, g, b): (u8, u8, u8), rgb_hex: &str) {
    let mut stdout = StandardStream::stdout(ColorChoice::Always);
    stdout
        .set_color(
            ColorSpec::new()
                .set_bg(Some(Color::Rgb(r, g, b)))
                .set_fg(Some(Color::Rgb(255 - r, 255 - g, 255 - b))),
        )
        .unwrap();

    stdout.write_all(rgb_hex.as_bytes()).unwrap();
    stdout.write_all(b"\n").unwrap();

    stdout.reset().unwrap();

    stdout.flush().unwrap();
}

fn send_to_clibpoard(rgb_hex: String) {
    #[cfg(unix)]
    {
        use copypasta_ext::{prelude::ClipboardProvider, x11_bin, x11_fork};

        let bin_result =
            x11_bin::ClipboardContext::new().and_then(|mut x| x.set_contents(rgb_hex.to_string()));

        if let Err(err) = bin_result {
            eprintln!("{err}\nUsing fork");
        } else {
            return;
        }

        let fork_result =
            x11_fork::ClipboardContext::new().and_then(|mut x| x.set_contents(rgb_hex));

        if let Err(err) = fork_result {
            eprintln!("Fork clipboard method failed: {err}");
        }
    }

    #[cfg(not(unix))]
    copypasta_ext::display::DisplayServer::select()
        .try_context()
        .map(|mut x| x.set_contents(rgb_hex))
        .expect("Could not find display server")
        .expect("Failed to set clipboard content");
}
