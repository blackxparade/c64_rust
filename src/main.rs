#![allow(unused)]

extern crate rand;
extern crate sfml;

mod cpu;
use self::cpu::MOS6510;
use sfml::graphics::*;
use sfml::window::*;
use rand::Rng;


fn main() {

    let c64 : MOS6510 = MOS6510::new();

    let mut window = RenderWindow::new (
        (800, 600),
        "this is a title.",
        Style::TITLEBAR | Style::CLOSE,
        &Default::default(),
    );
    window.set_vertical_sync_enabled(true);

    let mut rng = rand::thread_rng();
    let mut pixels: [u8; 80 * 60 * 4] = [150; 80 * 60 * 4];

    let font = Font::from_file("res/C64_pro.ttf").unwrap();
    let msg = "YAY this shit is working";
    let mut instructions = Text::new(msg, &font, 30);
    instructions.set_position((100.0, 280.0));
    instructions.set_fill_color(&Color::WHITE);

    while window.is_open() {
        while let Some(event) = window.poll_event() {
            use crate::Event::*;
            match event {
                Closed => window.close(),
                KeyPressed { code, .. } => match code {
                    Key::Escape => window.close(),
                    _ => {}
                },
                _ => {}
            }
        }

        for i in 0..(80 * 60 * 4) {
            pixels[i] = rng.gen_range(0, 255);
        }

        let mut texture = Texture::new(800, 600).unwrap();
        texture.update_from_pixels(&pixels, 80, 60, 0, 0);
        let mut sprite = Sprite::with_texture(&texture);
        sprite.set_position((0., 0.));
        sprite.set_scale((10.0, 10.0));

        window.clear(&Color::BLACK);
        window.draw(&sprite);
        window.draw(&instructions);
        window.display();
    }
}
