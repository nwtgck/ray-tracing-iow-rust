use crate::camera::Camera;
use crate::hitable::Hitable;

pub struct Scene {
    pub camera: Camera,
    pub hitable: Box<dyn Hitable + Sync>
}
