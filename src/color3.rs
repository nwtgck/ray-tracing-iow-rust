#[derive(Debug)]
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
