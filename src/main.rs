use copypasta::{ClipboardContext, ClipboardProvider};
#[cfg(feature = "device_query")]
use device_query::{DeviceQuery, DeviceState, MouseState};
use image::{io::Reader, ImageFormat};
use screenshots::Screen;
use std::io::{Cursor, Write};
use termcolor::{Color, ColorChoice, ColorSpec, StandardStream, WriteColor};
#[cfg(feature = "mouce")]
use {
    mouce::{
        common::{MouseButton, MouseEvent},
        Mouse,
    },
    std::sync::mpsc,
};

fn main() {
    let coordinates = match request_pixel_position() {
        Some(x) => x,
        None => {
            println!("Ending program");
            return;
        }
    };

    let (r, g, b) = get_pixel_colour(coordinates);

    let rgb_hex = format!("#{r:X}{g:X}{b:X}");

    print_result((r, g, b), &rgb_hex);

    ClipboardContext::new()
        .unwrap()
        .set_contents(rgb_hex)
        .unwrap();
}

#[cfg(feature = "device_query")]
fn request_pixel_position() -> Option<(i32, i32)> {
    let device_state = DeviceState::new();

    loop {
        let mouse: MouseState = device_state.get_mouse();

        if mouse.button_pressed[1] == true {
            return Some(mouse.coords);
        }

        if mouse.button_pressed[3] == true {
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
    let screen = Screen::from_point(x, y).unwrap();

    let (x, y) = (x - screen.display_info.x, y - screen.display_info.y);

    let screenshot = screen.capture_area(x, y, 1, 1).unwrap();

    let stream = Cursor::new(screenshot.buffer());

    let image = Reader::with_format(stream, ImageFormat::Png)
        .decode()
        .unwrap();

    let pixel = image.as_rgba8().unwrap().pixels().next().unwrap();

    (pixel.0[0], pixel.0[1], pixel.0[2])
}

fn print_result((r, g, b): (u8, u8, u8), rgb_hex: &str) {
    let mut stdout = StandardStream::stdout(ColorChoice::Always);
    stdout
        .set_color(
            ColorSpec::new()
                .set_bg(Some(Color::Rgb(r, g, b)))
                .set_fg(Some(Color::Rgb(255 - r, 255 - g, 255 - b))),
        )
        .unwrap();

    stdout.write(rgb_hex.as_bytes()).unwrap();

    stdout.reset().unwrap();
}
