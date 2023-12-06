use std::io::BufRead;

use ply_rs::{ply, parser};

use crate::model::ModelVertex;

#[derive(Debug)]
struct Face {
    vertex_index: Vec<u32>,
}

pub struct Model {
    pub vertices: Vec<ModelVertex>,
    pub indices: Vec<u32>,
}


// The structs need to implement the PropertyAccess trait, otherwise the parser doesn't know how to write to them.
// Most functions have default, hence you only need to implement, what you expect to need.

impl ply::PropertyAccess for ModelVertex {
    fn new() -> Self {
        ModelVertex {
            position: Default::default(),
            normal: Default::default(),
        }
    }
    fn set_property(&mut self, key: String, property: ply::Property) {
        use ply_rs::ply::Property::Float;
        match (key.as_ref(), property) {
            ("x", Float(v)) => self.position[0] = v,
            ("y", Float(v)) => self.position[1] = v,
            ("z", Float(v)) => self.position[2] = v,
            ("nx", Float(v)) => self.normal[0] = v,
            ("ny", Float(v)) => self.normal[1] = v,
            ("nz", Float(v)) => self.normal[2] = v,
            (k, _) => panic!("Vertex: Unexpected key/value combination: key: {}", k),
        }
    }
}

// same thing for Face
impl ply::PropertyAccess for Face {
    fn new() -> Self {
        Face {
            vertex_index: Vec::new(),
        }
    }
    fn set_property(&mut self, key: String, property: ply::Property) {
        match (key.as_ref(), property) {
            ("vertex_indices", ply::Property::ListUInt(vec)) => self.vertex_index = vec,
            (k, _) => panic!("Face: Unexpected key/value combination: key: {}", k),
        }
    }
}

/// Demonstrates simplest use case for reading from a file.
pub fn parse_model(mut f: impl BufRead) -> Model {
    let vertex_parser = parser::Parser::<ModelVertex>::new();
    let face_parser = parser::Parser::<Face>::new();

    let header = vertex_parser.read_header(&mut f).unwrap();

    let mut vertex_list = Vec::new();
    let mut face_list = Vec::new();
    for (_ignore_key, element) in &header.elements {
        match element.name.as_ref() {
            "vertex" => { vertex_list = vertex_parser.read_payload_for_element(&mut f, &element, &header).unwrap(); },
            "face" => { face_list = face_parser.read_payload_for_element(&mut f, &element, &header).unwrap(); },
            _ => panic!("Enexpeced element!"),
        }
    }
    let index_count = face_list.iter().map(|x| x.vertex_index.len()).sum();
    let mut indices = Vec::with_capacity(index_count);
    for mut face in face_list.into_iter() {
        indices.extend(face.vertex_index.drain(..))
    }

    return Model {
        vertices: vertex_list,
        indices: indices,
    }
}