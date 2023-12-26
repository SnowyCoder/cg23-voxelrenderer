use std::str::FromStr;

use cgmath::Vector3;
use nom::{
    IResult,
    bytes::complete::tag,
    character::complete::{multispace0, digit1, space0}, multi::{many0, count}, sequence::{preceded, pair}, combinator::{map_res, recognize, map},
};

use crate::color::Color;

use super::scene::{Voxel, Scene};


struct Header {
    grid_size: Vector3<u32>,
    voxel_num: u32,
}

fn parse_int<F: FromStr>(input: &str) -> IResult<&str, F> {
    let (input, _) = space0(input)?;// consume & ignore initial spaces
    map_res(recognize(digit1), str::parse)(input)
}

fn parse_vec3(input: &str) -> IResult<&str, Vector3<u32>> {
    // invert y and z!
    map(count(parse_int, 3), |x| Vector3::new(x[0], x[2], x[1]))(input)
}

fn parse_header(input: &str) -> IResult<&str, Header> {
    let (input, grid_size) = preceded(pair(multispace0, tag("grid_size:")), parse_vec3)(input)?;
    let (input, voxel_num) = preceded(pair(multispace0, tag("voxel_num:")), parse_int)(input)?;

    Ok((input, Header { grid_size, voxel_num }))
}

fn parse_voxel(input: &str) -> IResult<&str, Voxel> {
    let (input, _) = multispace0(input)?;
    map(count(parse_int, 4), |x| Voxel {
        // invert y and z!
        pos: Vector3::new(x[0], x[2], x[1]),
        color: x[3]
    })(input)
}

fn parse_color(input: &str) -> IResult<&str, Color> {
    let (input, _) = multispace0(input)?;
    map(pair(parse_int::<u32>, count(parse_int, 3)), |(_idx, rgb)|
        Color::new(rgb[0], rgb[1], rgb[2])
    )(input)
}

pub fn parse_scene(input: &str) -> IResult<&str, Scene> {
    let (input, header) = parse_header(input)?;

    let (input , voxels) = count(parse_voxel, header.voxel_num as _)(input)?;

    let (input, colors) = many0(parse_color)(input)?;

    // TODO: Ensure that all colors are ordered
    Ok((input, Scene {
        voxels, colors,
        grid_size: header.grid_size,
    }))
}
