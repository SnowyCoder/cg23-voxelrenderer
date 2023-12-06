

#[derive(Clone, Copy, Debug, Default)]
pub struct Color {
    pub r: u8,
    pub g: u8,
    pub b: u8,
}

impl Color {
    pub fn new(r: u8, g: u8, b: u8) -> Self {
        Color { r, g, b}
    }

    pub fn as_instance_data(&self) -> [f32; 3] {
        [
            self.r as f32 / 255.0,
            self.g as f32 / 255.0,
            self.b as f32 / 255.0,
        ]
    }
}

impl From<Color> for wgpu::Color {
    fn from(value: Color) -> Self {
        wgpu::Color {
            r: value.r as f64 / 255.0,
            g: value.g as f64 / 255.0,
            b: value.b as f64 / 255.0,
            a: 1.0,
        }
    }
}