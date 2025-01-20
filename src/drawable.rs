pub(crate) mod fps;
pub(crate) mod room;
pub(crate) mod tree;

use crate::bounding_box::BoundingBox;
use crate::frame::Frame;

pub trait Drawable {
    fn draw(&self, frame: &mut Frame);

    fn bound_box(&self) -> BoundingBox;
}
