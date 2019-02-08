#![allow(non_snake_case)]
#![allow(unused)]

extern crate sfml;
use sfml::graphics::*;
use sfml::window::*;
use sfml::window::mouse::*;

pub struct Debugger {
    dbg : RenderWindow,
    font : Font,
    active_state : u8,
}

impl Debugger {
    pub fn new() -> Debugger {
        Debugger {
            dbg : RenderWindow::new (
                (800, 600),
                "C64 DBG",
                Style::TITLEBAR | Style::CLOSE,
                &Default::default(),
            ),
            font : Font::from_file("res/C64_pro.ttf").unwrap(),
            active_state : 0,
        }
    }

    pub fn clear(&mut self) {
        self.dbg.clear(&Color::rgb(72, 59, 170));
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
                            println!("down");
                            self.active_state += 1;
                        }
                        if (delta < 0.0) {
                            println!("up");
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
            render_text.set_position((15.0, i as f32 * 35.0 + 10.0) );

            if (i == self.active_state as usize) {
                render_text.set_fill_color(&Color::rgb(147, 81, 182));
            } else {
                render_text.set_fill_color(&Color::rgb(134, 122, 221));
            }

            self.dbg.draw(&render_text);
        }
    }
}
