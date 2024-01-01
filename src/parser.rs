use std:: ffi::OsStr;

use anyhow::Context;
use nom::Finish;

mod scene;
mod vly;
mod vox;
mod ply_model;

pub use ply_model::Model;
pub use scene::Scene;


enum ExpectedFormat {
    UNKNOWN,
    VLY,
    VOX,
}

fn parse_vly(data: &[u8]) -> anyhow::Result<Scene> {
    let data = std::str::from_utf8(data)?;

    let (_rest, res) = vly::parse_scene(data)
        .map_err(|e| e.to_owned())
        .finish()
        .context("Invalid vly format")?;
    Ok(res)
}


fn parse_vox(data: &[u8]) -> anyhow::Result<Scene> {
    let (_rest, mut res) = vox::parse_scene(data)
        .map_err(|e| e.map_input(|x| format!("{:?}", x)))
        .finish()
        .context("Invalid vox format")?;
    if res.len() == 0 {
        Err(anyhow::anyhow!("VOX pack contains no scene!"))?;
    }
    res.truncate(1);
    Ok(res.pop().unwrap())
}

pub fn parse_scene(file_content: &[u8], file_name: Option<&OsStr>) -> anyhow::Result<Scene> {
    let format = match file_name {
        None => ExpectedFormat::UNKNOWN,
        Some(x) => {
            let name = x.as_encoded_bytes();
            let ext_pos = name.iter().enumerate().rev().find(|(_i, c)| **c == b'.').map(|(i, _c)| i);
            let ext = match ext_pos {
                Some(x) => name.split_at(x + 1).1,
                None => name,
            };

            match ext {
                b"vly" => ExpectedFormat::VLY,
                b"vox" => ExpectedFormat::VOX,
                _ => ExpectedFormat::UNKNOWN,
            }
        },
    };

    match format {
        ExpectedFormat::VLY => parse_vly(file_content),
        ExpectedFormat::VOX => parse_vox(file_content),
        ExpectedFormat::UNKNOWN => {
            Err(())
                .or_else(|_| parse_vly(file_content))
                .or_else(|_| parse_vox(file_content))
                .or_else(|_| Err(anyhow::anyhow!("Cannot determine format")))
        },
    }
}

pub use ply_model::parse_model;
