use std::error::Error;

use nom::Finish;

mod scene;
mod vly;
mod ply_model;

pub use ply_model::Model;
pub use scene::{Scene, Voxel};

pub fn parse_scene<'a>(file_content: &'a str, _file_name: Option<&str>) -> Result<Scene, Box<dyn Error + 'a>> {
    let (_rest, res) = vly::parse_scene(file_content).finish()?;
    return Ok(res)
}

pub use ply_model::parse_model;