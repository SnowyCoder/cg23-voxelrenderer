use std:: ffi::OsStr;

use anyhow::Context;
use nom::Finish;

mod scene;
mod vly;
mod ply_model;

pub use ply_model::Model;
pub use scene::{Scene, Voxel};

pub fn parse_scene<'a>(file_content: &'a [u8], _file_name: Option<&OsStr>) -> anyhow::Result<Scene> {
    let file_content = std::str::from_utf8(file_content)?;
    let (_rest, res) = vly::parse_scene(file_content)
        .map_err(|e| e.to_owned())
        .finish()
        .context("Invalid vly format")?;
    return Ok(res)
}

pub use ply_model::parse_model;
