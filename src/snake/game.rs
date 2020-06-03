use crate::drawing::screen;
use crate::snake::body_part;

pub struct Game<'a> {
    pub screen: screen::Screen,
    pub previous_screen: screen::Screen,
    pub snake: body_part::BodyPart<'a>,
}

impl<'a> Game<'a> {
    pub fn new() -> Game<'a> {
        Game {
            screen: screen::Screen::new(),
            previous_screen: screen::Screen::new(),
            snake: body_part::BodyPart::new(),
        }
    }

    pub fn update(&mut self) {
        snake.update();
    }
}