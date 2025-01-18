pub(crate) mod room;
pub(crate) mod tree;
pub(crate) mod fps;

use crate::frame::Frame;

pub trait Drawable {
    fn draw(&self, frame: &mut Frame);
}