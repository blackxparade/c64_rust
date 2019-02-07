#![allow(non_snake_case)]
#![allow(unused)]

extern crate sfml;
use sfml::graphics::*;
use sfml::window::*;

pub struct Debugger {
    dbg : RenderWindow,
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
        }
    }

    pub fn render(&mut self) {
        self.dbg.clear(&Color::BLACK);
        self.dbg.display();
    }
}
