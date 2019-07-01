use crate::camera::Camera;
use crate::hitable::Hitable;

pub struct Scene<H: Hitable> {
    pub camera: Camera,
    pub hitable: H
}
