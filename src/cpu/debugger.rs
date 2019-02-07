#![allow(non_snake_case)]
#![allow(unused)]

extern crate sfml;
use sfml::graphics::*;
use sfml::window::*;
use sfml::window::mouse::*;

pub struct Debugger {
    dbg : RenderWindow,
    font : Font,
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
        }
    }

    pub fn render(&mut self) {
        self.dbg.clear(&Color::rgb(72, 59, 170));
        self.dbg.display();
    }

    pub fn poll(&mut self) -> bool {
        while let Some(event) = self.dbg.poll_event() {
            use crate::Event::*;
            match event {
                Closed => self.dbg.close(),
                KeyPressed { code, .. } => match code {
                    Key::Escape => { self.dbg.close(); return false;},
                    _ => {}
                },
                MouseWheelScrolled { wheel, delta, .. } => match wheel {
                    Wheel::Vertical => {
                        if (delta > 0.0) { println!("down"); }
                        if (delta < 0.0) { println!("up"); }
                    },
                    _ => {}
                }
                _ => {}
            }
        }
        true
    }

    pub fn assemble_text(&mut self, text: String, x: f32, y: f32, color: Color) -> Text {
        let mut drawable_text = Text::new(&text, &self.font, 30);
        drawable_text.set_position((x, y));
        drawable_text.set_fill_color(&color);
        drawable_text
    }
}
