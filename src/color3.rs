use std::ops::{Add, Mul};

#[derive(Debug, Copy, Clone)]
pub struct Color3 {
    pub r: f32,
    pub g: f32,
    pub b: f32
}

fn color_elem_to_int(f: f32) -> i32 {
    (255.99f32 * f) as i32
}

impl Color3 {
    pub fn ir(&self) -> i32 {
        color_elem_to_int(self.r)
    }

    pub fn ig(&self) -> i32 {
        color_elem_to_int(self.g)
    }

    pub fn ib(&self) -> i32 {
        color_elem_to_int(self.b)
    }
}

impl Add for &Color3 {
    type Output = Color3;

    fn add(self, rhs: Self) -> Self::Output {
        Color3{r: self.r + rhs.r, g: self.g + rhs.g, b: self.b + rhs.b}
    }
}

impl Mul<f32> for &Color3 {
    type Output = Color3;

    fn mul(self, rhs: f32) -> Self::Output {
        Color3{r: self.r * rhs, g: self.g * rhs, b: self.b * rhs}
    }
}
