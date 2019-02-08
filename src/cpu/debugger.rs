#![allow(non_snake_case)]
#![allow(unused)]

extern crate sfml;
use sfml::graphics::*;
use sfml::window::*;
use sfml::window::mouse::*;

pub struct Debugger {
    dbg          : RenderWindow,
    font         : Font,
    active_state : u8,
    LIGHT_BLUE   : Color,
    DARK_BLUE    : Color,
    PURPLE       : Color,
}

impl Debugger {
    pub fn new() -> Debugger {
        Debugger {
            dbg          : RenderWindow::new (
                           (1600, 1600),
                           "C64 DBG",
                           Style::TITLEBAR | Style::CLOSE,
                           &Default::default(),
                           ),
            font         : Font::from_file("res/C64_pro.ttf").unwrap(),
            active_state : 0,
            LIGHT_BLUE   : Color::rgb(134, 122, 221),
            DARK_BLUE    : Color::rgb(72, 59, 170),
            PURPLE       : Color::rgb(147, 81, 182),
        }
    }

    pub fn clear(&mut self) {
        self.dbg.clear(&self.DARK_BLUE);
    }

    pub fn render(&mut self) {
        self.dbg.display();
    }

    pub fn poll(&mut self) -> bool {
        while let Some(event) = self.dbg.poll_event() {
            use crate::Event::*;
            match event {
                Closed => { self.dbg.close(); return false; }
                KeyPressed { code, .. } => match code {
                    Key::Escape => { self.dbg.close(); return false;},
                    _ => {}
                },
                MouseWheelScrolled { wheel, delta, .. } => match wheel {
                    Wheel::Vertical => {
                        if (delta > 0.0) {
                            self.active_state += 1;
                        }
                        if (delta < 0.0) {
                            if (self.active_state > 0) {
                                self.active_state -= 1;
                            }
                        }
                    },
                    _ => {}
                }
                _ => {}
            }
        }
        true
    }

    pub fn assemble_text(&mut self, text: Vec<String>) {
        for i in 0..text.len()-1 {
            let mut render_text = Text::new(&text[i], &self.font, 22);
            render_text.set_position((15.0, i as f32 * 32.0 + 5.0) );

            if (i == self.active_state as usize) {
                render_text.set_fill_color(&self.PURPLE);
            } else {
                render_text.set_fill_color(&self.LIGHT_BLUE);
            }

            self.dbg.draw(&render_text);
        }
    }

    pub fn memory_map(&mut self, ram: &[u8]) {

        let mut pixels: [u8; 256 * 256 * 4] = [255; 256 * 256 * 4];

        for i in 0..ram.len() {
            if (ram[i] != 0) {
                pixels[i*4] = 134;
                pixels[i*4+1] = 122;
                pixels[i*4+2] = 221;
                pixels[i*4+3] = 255;
            } else {
                pixels[i*4] = 72;
                pixels[i*4+1] = 59;
                pixels[i*4+2] = 170;
                pixels[i*4+3] = 255;
            }
        }

        let mut texture = Texture::new(256, 256).unwrap();
        texture.update_from_pixels(&pixels, 256, 256, 0, 0);
        let mut sprite = Sprite::with_texture(&texture);
        sprite.set_position((1000.0, 1000.0));
        sprite.set_scale((2.0, 2.0));
        self.dbg.draw(&sprite);
    }
}
