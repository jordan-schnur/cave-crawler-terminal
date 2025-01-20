use crate::camera::Camera;
use crate::frame::Frame;

pub trait Moveable {
    fn attempt_move(&mut self, dx: i32, dy: i32, frame: &Frame, camera: &Camera) -> bool;
}