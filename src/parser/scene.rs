use cgmath::Vector3;

use crate::color::Color;


#[derive(Debug, Clone)]
pub struct Voxel {
    pub pos: Vector3<u32>,
    pub color: u32,
}


#[derive(Debug, Clone)]
pub struct Scene {
    pub voxels: Vec<Voxel>,
    pub colors: Vec<Color>,
    pub grid_size: Vector3<u32>,
}