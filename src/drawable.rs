pub(crate) mod fps;
pub(crate) mod room;
pub(crate) mod tree;

use crate::bounding_box::BoundingBox;
use crate::frame::Frame;
use crate::tile::{Coord, Tile};
use std::collections::HashMap;

pub trait Drawable {
    fn draw(&self, frame: &mut Frame);

    fn static_map(&self, collision_map: &mut HashMap<Coord, Tile>);
    fn bound_box(&self) -> BoundingBox;
    fn as_any(&self) -> &dyn std::any::Any;
    fn as_any_mut(&mut self) -> &mut dyn std::any::Any;
}

impl dyn Drawable {
    pub fn downcast_ref<T: Drawable + 'static>(&self) -> Option<&T> {
        self.as_any().downcast_ref::<T>()
    }

    pub fn downcast_mut<T: Drawable + 'static>(&mut self) -> Option<&mut T> {
        self.as_any_mut().downcast_mut::<T>()
    }
}
