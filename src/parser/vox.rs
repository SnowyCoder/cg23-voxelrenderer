
use cgmath::{Vector3, num_traits::ToBytes};
use nom::{
    IResult,
    bytes::complete::{tag, take},
    multi::{count, fill},
    sequence::{preceded, tuple},
    combinator::map, error::{ParseError, ErrorKind, FromExternalError}, Err,
};

use crate::color::Color;

use super::scene::{Voxel, Scene};

const MAGIC_BYTES: &'static [u8] = b"VOX ";
const DEFAULT_PALETTE: &[u32] = &[
    0x00000000, 0xffffffff, 0xffccffff, 0xff99ffff, 0xff66ffff, 0xff33ffff, 0xff00ffff, 0xffffccff, 0xffccccff, 0xff99ccff, 0xff66ccff, 0xff33ccff, 0xff00ccff, 0xffff99ff, 0xffcc99ff, 0xff9999ff,
    0xff6699ff, 0xff3399ff, 0xff0099ff, 0xffff66ff, 0xffcc66ff, 0xff9966ff, 0xff6666ff, 0xff3366ff, 0xff0066ff, 0xffff33ff, 0xffcc33ff, 0xff9933ff, 0xff6633ff, 0xff3333ff, 0xff0033ff, 0xffff00ff,
    0xffcc00ff, 0xff9900ff, 0xff6600ff, 0xff3300ff, 0xff0000ff, 0xffffffcc, 0xffccffcc, 0xff99ffcc, 0xff66ffcc, 0xff33ffcc, 0xff00ffcc, 0xffffcccc, 0xffcccccc, 0xff99cccc, 0xff66cccc, 0xff33cccc,
    0xff00cccc, 0xffff99cc, 0xffcc99cc, 0xff9999cc, 0xff6699cc, 0xff3399cc, 0xff0099cc, 0xffff66cc, 0xffcc66cc, 0xff9966cc, 0xff6666cc, 0xff3366cc, 0xff0066cc, 0xffff33cc, 0xffcc33cc, 0xff9933cc,
    0xff6633cc, 0xff3333cc, 0xff0033cc, 0xffff00cc, 0xffcc00cc, 0xff9900cc, 0xff6600cc, 0xff3300cc, 0xff0000cc, 0xffffff99, 0xffccff99, 0xff99ff99, 0xff66ff99, 0xff33ff99, 0xff00ff99, 0xffffcc99,
    0xffcccc99, 0xff99cc99, 0xff66cc99, 0xff33cc99, 0xff00cc99, 0xffff9999, 0xffcc9999, 0xff999999, 0xff669999, 0xff339999, 0xff009999, 0xffff6699, 0xffcc6699, 0xff996699, 0xff666699, 0xff336699,
    0xff006699, 0xffff3399, 0xffcc3399, 0xff993399, 0xff663399, 0xff333399, 0xff003399, 0xffff0099, 0xffcc0099, 0xff990099, 0xff660099, 0xff330099, 0xff000099, 0xffffff66, 0xffccff66, 0xff99ff66,
    0xff66ff66, 0xff33ff66, 0xff00ff66, 0xffffcc66, 0xffcccc66, 0xff99cc66, 0xff66cc66, 0xff33cc66, 0xff00cc66, 0xffff9966, 0xffcc9966, 0xff999966, 0xff669966, 0xff339966, 0xff009966, 0xffff6666,
    0xffcc6666, 0xff996666, 0xff666666, 0xff336666, 0xff006666, 0xffff3366, 0xffcc3366, 0xff993366, 0xff663366, 0xff333366, 0xff003366, 0xffff0066, 0xffcc0066, 0xff990066, 0xff660066, 0xff330066,
    0xff000066, 0xffffff33, 0xffccff33, 0xff99ff33, 0xff66ff33, 0xff33ff33, 0xff00ff33, 0xffffcc33, 0xffcccc33, 0xff99cc33, 0xff66cc33, 0xff33cc33, 0xff00cc33, 0xffff9933, 0xffcc9933, 0xff999933,
    0xff669933, 0xff339933, 0xff009933, 0xffff6633, 0xffcc6633, 0xff996633, 0xff666633, 0xff336633, 0xff006633, 0xffff3333, 0xffcc3333, 0xff993333, 0xff663333, 0xff333333, 0xff003333, 0xffff0033,
    0xffcc0033, 0xff990033, 0xff660033, 0xff330033, 0xff000033, 0xffffff00, 0xffccff00, 0xff99ff00, 0xff66ff00, 0xff33ff00, 0xff00ff00, 0xffffcc00, 0xffcccc00, 0xff99cc00, 0xff66cc00, 0xff33cc00,
    0xff00cc00, 0xffff9900, 0xffcc9900, 0xff999900, 0xff669900, 0xff339900, 0xff009900, 0xffff6600, 0xffcc6600, 0xff996600, 0xff666600, 0xff336600, 0xff006600, 0xffff3300, 0xffcc3300, 0xff993300,
    0xff663300, 0xff333300, 0xff003300, 0xffff0000, 0xffcc0000, 0xff990000, 0xff660000, 0xff330000, 0xff0000ee, 0xff0000dd, 0xff0000bb, 0xff0000aa, 0xff000088, 0xff000077, 0xff000055, 0xff000044,
    0xff000022, 0xff000011, 0xff00ee00, 0xff00dd00, 0xff00bb00, 0xff00aa00, 0xff008800, 0xff007700, 0xff005500, 0xff004400, 0xff002200, 0xff001100, 0xffee0000, 0xffdd0000, 0xffbb0000, 0xffaa0000,
    0xff880000, 0xff770000, 0xff550000, 0xff440000, 0xff220000, 0xff110000, 0xffeeeeee, 0xffdddddd, 0xffbbbbbb, 0xffaaaaaa, 0xff888888, 0xff777777, 0xff555555, 0xff444444, 0xff222222, 0xff111111
];


struct Header {
    version: u32,
}

fn parse_int4(input: &[u8]) -> IResult<&[u8], u32> {
    let (input, version) = take(4u32)(input)?;

    let n = u32::from_le_bytes(version.try_into().unwrap());
    Ok((input, n))
}

fn parse_header(input: &[u8]) -> IResult<&[u8], Header> {
    let (input, _magic) = tag(MAGIC_BYTES)(input)?;
    let (input, version) = parse_int4(input)?;

    Ok((input, Header { version }))
}

fn parse_chunk<'a>(name: &'static [u8]) ->  impl FnMut(&'a [u8]) -> IResult<&'a [u8], (&'a [u8], u32)> {
    move |i: &[u8]| {
        let (input, (data_len, ch_count)) = preceded(tag(name), tuple((parse_int4, parse_int4)))(i)?;
        let (input, chunk_data) = take(data_len)(input)?;

        Ok((input, (chunk_data, ch_count)))
    }
}

fn check_zero(chunk: &[u8], children: u32) -> Result<(), nom::Err<nom::error::Error<&[u8]>>> {
    if children != 0 {
        return Err(nom::Err::Error(nom::error::Error::from_error_kind(chunk, ErrorKind::NonEmpty)));
    }
    Ok(())
}

fn parse_vec3(input: &[u8]) -> IResult<&[u8], Vector3<u32>> {
    map(count(parse_int4, 3), |x| Vector3::new(x[0], x[2], x[1]))(input)
}

fn parse_size(input: &[u8]) -> IResult<&[u8], Vector3<u32>> {
    let (input, (data, children)) = parse_chunk(b"SIZE")(input)?;
    check_zero(data, children)?;
    let (data, vec3) = parse_vec3(data)?;
    check_zero(data, data.len() as _)?;

    Ok((input, vec3))
}

fn parse_voxel(input: &[u8]) -> IResult<&[u8], Voxel> {
    map(parse_int4, |x| {
        let x = x.to_le_bytes();
        Voxel {
            // invert y and z!
            pos: Vector3::new(x[1] as u32, x[2] as _, x[0] as _),
            color: x[3] as u32
        }
})(input)
}

fn parse_model(input: &[u8]) -> IResult<&[u8], (Vector3<u32>, Vec<Voxel>)> {
    let (input, size) = parse_size(input)?;
    let (input, (chunk, children)) = parse_chunk(b"XYZI")(input)?;
    check_zero(input, children)?;
    let (chunk, num_voxels) = parse_int4(chunk)?;
    let (_chunk, voxels) = count(parse_voxel, num_voxels as _)(chunk)?;

    Ok((input, (size, voxels)))
}

fn parse_color(input: &[u8]) -> IResult<&[u8], Color> {
    let (input, rgba) = take(4u32)(input)?;
    let color = Color {
        r: rgba[0],
        g: rgba[1],
        b: rgba[2],
        // a? no >:(
    };
    Ok((input, color))
}

fn parse_palette(input: &[u8]) -> IResult<&[u8], Vec<Color>> {
    let (input, (data, children)) = parse_chunk(b"RGBA")(input)?;
    check_zero(data, children)?;
    // https://github.com/ephtracy/voxel-model/blob/8044f9eb086216f3485cdaa525a52120d72274e9/MagicaVoxel-file-format-vox.txt#L81
    // this is the line that we should use if we were to respect what the specifics says
    // but the pseudocode shown after reads only 255 colors, putting a zero (i guess) at the beginning
    // in all the files I've seen 0 is always used as thelast element, so I hope nothing is wrong?
    //let (_data, mut palette) = count(parse_color, 256)(data)?;
    let mut palette = vec![Color::new(0, 0, 0); 256];
    let (_data, ()) = fill(parse_color, &mut palette.as_mut_slice()[1..])(data)?;
    Ok((input, palette))
}

fn parse_pack(input: &[u8]) -> IResult<&[u8], u32> {
    let (input, (data, children)) = parse_chunk(b"PACK")(input)?;
    check_zero(data, children)?;
    let (_data, len_models) = parse_int4(data)?;
    Ok((input, len_models))
}

fn default_palette() -> Vec<Color> {
    DEFAULT_PALETTE.iter().map(|c| parse_color(&c.to_le_bytes()).unwrap().1).collect()
}

fn parse_main(input: &[u8]) -> IResult<&[u8], Vec<Scene>> {
    let (input, (data, children)) = parse_chunk(b"MAIN")(input)?;
    check_zero(data, data.len() as _)?;

    if children == 0 {
        return Err(Err::Error(nom::error::Error::from_external_error(input, ErrorKind::NonEmpty, "No children for pack")));
    }

    let (input, num_models) = parse_pack(input).unwrap_or((input, 1));
    let (input, partial_models) = count(parse_model, num_models as _)(input)?;
    let (input, palette) =  parse_palette(input).unwrap_or_else(|_| (input, default_palette()));

    let models = partial_models.into_iter().map(|(grid_size, voxels)| Scene {
        voxels,
        colors: palette.clone(),
        grid_size,
    }).collect();

    Ok((input, models))
}

pub fn parse_scene(input: &[u8]) -> IResult<&[u8], Vec<Scene>> {
    let (input, header) = parse_header(input)?;
    if header.version != 150 {
        return Err(Err::Error(nom::error::Error::from_external_error(input, ErrorKind::Verify, "Invalid version")));
    }
    parse_main(input)
}
