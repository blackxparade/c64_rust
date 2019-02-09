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
    line_count   : u8,
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
                           &Default::default(),),
            font         : Font::from_file("res/C64_pro.ttf").unwrap(),
            active_state : 0,
            line_count   : 0,
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
                            if (self.line_count - 1 > self.active_state) {
                                self.active_state += 1;
                            }
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
        self.line_count = text.len() as u8;
        const BG_WIDTH  : u32 = 1000;
        const BG_HEIGHT : u32 = 30;
        let mut text_background: [u8; BG_WIDTH as usize * BG_HEIGHT as usize * 4] =
                                 [255; BG_WIDTH as usize * BG_HEIGHT as usize * 4];

        for i in (0..text_background.len()).step_by(4) {
            text_background[i]   = 134;
            text_background[i+1] = 122;
            text_background[i+2] = 221;
            text_background[i+3] = 255;
        }

        let mut text_background_texture = Texture::new(BG_WIDTH, BG_HEIGHT).unwrap();
        text_background_texture.update_from_pixels(&text_background, BG_WIDTH, BG_HEIGHT, 0, 0);
        let mut text_background_sprite = Sprite::with_texture(&text_background_texture);

        for i in 0..text.len() {
            let mut render_text = Text::new(&text[i], &self.font, 22);
            render_text.set_position((15.0, i as f32 * 32.0 + 5.0));

            if (i == self.active_state as usize) {
                render_text.set_fill_color(&self.DARK_BLUE);
                text_background_sprite.set_position((11.0, i as f32 * 32.0 + 2.0));
                self.dbg.draw(&text_background_sprite);
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
        sprite.set_position((1050.0, 1050.0));
        sprite.set_scale((2.0, 2.0));
        self.dbg.draw(&sprite);
    }
}
