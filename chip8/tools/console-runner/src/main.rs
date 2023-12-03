use std::{env::args, fs, time::Instant};

use ruscii::{
    app::{App, State},
    drawing::{Pencil, RectCharset},
    keyboard::{Key, KeyEvent},
    spatial::Vec2,
    terminal::{Color, Style, Window},
};

use chip8::{Ch8Image, ManagedInterpreter, SCREEN_HEIGHT, SCREEN_WIDTH};

////////////////////////////////////////////////////////////////////////////////

fn map_key(ruscii_key: Key) -> Option<chip8::Key> {
    let value = match ruscii_key {
        Key::Num1 => 0x1,
        Key::Num2 => 0x2,
        Key::Num3 => 0x3,
        Key::Num4 => 0xC,
        Key::Q => 0x4,
        Key::W => 0x5,
        Key::E => 0x6,
        Key::R => 0xD,
        Key::A => 0x7,
        Key::S => 0x8,
        Key::D => 0x9,
        Key::F => 0xE,
        Key::Z => 0xA,
        Key::X => 0x0,
        Key::C => 0xB,
        Key::V => 0xF,
        _ => return None,
    };
    Some(chip8::Key::try_from(value).unwrap())
}

////////////////////////////////////////////////////////////////////////////////

fn main() {
    let image_path = args().collect::<Vec<_>>()[1].clone();
    let image_data = fs::read(image_path).unwrap();
    let image = Ch8Image::new(image_data).expect("failed to load image");

    let mut interpreter = ManagedInterpreter::new(image, rand::random);

    let mut app = App::default();
    let mut last_instant = Instant::now();
    let mut crashed_error = None;

    app.run(|state: &mut State, window: &mut Window| {
        for key_event in state.keyboard().last_key_events() {
            if let KeyEvent::Pressed(Key::Esc) = key_event {
                state.stop();
            }
            let (is_pressed, key) = match key_event {
                KeyEvent::Pressed(key) => (true, key),
                KeyEvent::Released(key) => (false, key),
            };
            if let Some(chip8_key) = map_key(*key) {
                interpreter.set_key_down(chip8_key, is_pressed);
            }
        }

        let window_size = window.size();
        let mut pencil = Pencil::new(window.canvas_mut());

        pencil.set_origin(Vec2::xy(
            (window_size.x - 2 * SCREEN_WIDTH as i32) / 2,
            (window_size.y - SCREEN_HEIGHT as i32) / 2,
        ));

        let border_color = if crashed_error.is_some() {
            Color::Red
        } else {
            Color::White
        };
        pencil.set_foreground(border_color).draw_rect(
            &RectCharset::simple_round_lines(),
            Vec2::xy(-1, -1),
            Vec2::xy(SCREEN_WIDTH * 2 + 2, SCREEN_HEIGHT + 2),
        );

        let now = Instant::now();
        let duration = now.duration_since(last_instant);
        last_instant = now;

        if let Some(ref err) = crashed_error {
            pencil
                .set_foreground(Color::Red)
                .set_style(Style::Bold)
                .draw_center_text(
                    &format!("CRASHED: {}", err),
                    Vec2::xy(SCREEN_WIDTH, SCREEN_HEIGHT + 1),
                );
        } else {
            crashed_error = interpreter.simulate_duration(duration).err();
        }

        pencil.set_foreground(Color::Yellow).set_style(Style::Bold);
        for (y, row) in interpreter.frame_buffer().iter_rows().enumerate() {
            for (x, pixel) in row.iter().enumerate() {
                if *pixel {
                    pencil.draw_char('█', Vec2::xy(2 * x, y));
                    pencil.draw_char('█', Vec2::xy(2 * x + 1, y));
                }
            }
        }
    });
}
