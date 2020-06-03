use crate::drawing::screen;

pub struct BodyPart<'a> {
    pub x: i32,
    pub y: i32,
    pub tail: Option<&'a mut BodyPart<'a>>,
    pub is_main: bool,
}

impl<'a> BodyPart<'a> {
    pub fn new() -> BodyPart<'a> {
        BodyPart {
            x: 5,
            y: 5,
            tail: None,
            is_main: true,
        }
    }

    pub fn update(&mut self) {

    }

    fn draw(&mut self, screen: &mut screen::Screen) {
    }
}