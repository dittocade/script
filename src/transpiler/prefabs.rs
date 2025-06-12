use std::collections::HashMap;

use ndarray::{array, Array3};

use crate::game::{OptKind, RawKind, ValueKind};

#[derive(Debug, Default)]
#[allow(unused)]
pub struct Prefab {
    pub name: String,
    pub parts: Array3<u16>,
    pub inputs: Vec<ValuePort>,
    pub outputs: Vec<ValuePort>,
    pub callable: bool,
    pub callbacks: Vec<ExecutePort>,
    pub options: Vec<Opt>,
}

#[derive(Debug)]
#[allow(unused)]
pub struct ValuePort {
    pub name: String,
    pub kind: ValueKind,
}

#[derive(Debug)]
#[allow(unused)]
pub struct ExecutePort {
    pub name: String,
}

#[derive(Debug)]
#[allow(unused)]
pub struct Opt {
    pub name: String,
    pub kind: OptKind,
}

pub fn get_prefabs<'a>() -> HashMap<String, Prefab> {
    let prefabs = vec![
        Prefab {
            name: "stone_block".to_string(),
            parts: array![[[0x01]]],
            outputs: vec![ValuePort {
                name: "object".to_string(),
                kind: ValueKind::Raw(RawKind::Object),
            }],
            ..Default::default()
        },
        Prefab {
            name: "bricks".to_string(),
            parts: array![[[0x02]]],
            outputs: vec![ValuePort {
                name: "object".to_string(),
                kind: ValueKind::Raw(RawKind::Object),
            }],
            ..Default::default()
        },
        Prefab {
            name: "grass".to_string(),
            parts: array![[[0x03]]],
            outputs: vec![ValuePort {
                name: "object".to_string(),
                kind: ValueKind::Raw(RawKind::Object),
            }],
            ..Default::default()
        },
        Prefab {
            name: "spotted_grass".to_string(),
            parts: array![[[0x04]]],
            outputs: vec![ValuePort {
                name: "object".to_string(),
                kind: ValueKind::Raw(RawKind::Object),
            }],
            ..Default::default()
        },
        Prefab {
            name: "dirt".to_string(),
            parts: array![[[0x05]]],
            outputs: vec![ValuePort {
                name: "object".to_string(),
                kind: ValueKind::Raw(RawKind::Object),
            }],
            ..Default::default()
        },
        Prefab {
            name: "wood_x".to_string(),
            parts: array![[[0x06]]],
            outputs: vec![ValuePort {
                name: "object".to_string(),
                kind: ValueKind::Raw(RawKind::Object),
            }],
            ..Default::default()
        },
        Prefab {
            name: "wood_z".to_string(),
            parts: array![[[0x07]]],
            outputs: vec![ValuePort {
                name: "object".to_string(),
                kind: ValueKind::Raw(RawKind::Object),
            }],
            ..Default::default()
        },
        Prefab {
            name: "wood_y".to_string(),
            parts: array![[[0x08]]],
            outputs: vec![ValuePort {
                name: "object".to_string(),
                kind: ValueKind::Raw(RawKind::Object),
            }],
            ..Default::default()
        },
        Prefab {
            name: "comment".to_string(),
            parts: array![[[0x0F]]],
            options: vec![Opt {
                name: "value".to_string(),
                kind: OptKind::Name,
            }],
            ..Default::default()
        },
        Prefab {
            name: "inspect_number".to_string(),
            parts: array![[[0x10, 0x11]], [[0x12, 0x13]]],
            inputs: vec![ValuePort {
                name: "number".to_string(),
                kind: ValueKind::Raw(RawKind::Number),
            }],
            callable: true,
            ..Default::default()
        },
        Prefab {
            name: "inspect_vector".to_string(),
            parts: array![[[0x14, 0x15]], [[0x16, 0x17]]],
            inputs: vec![ValuePort {
                name: "vector".to_string(),
                kind: ValueKind::Raw(RawKind::Vector),
            }],
            callable: true,
            ..Default::default()
        },
        Prefab {
            name: "inspect_rotation".to_string(),
            parts: array![[[0x18, 0x19]], [[0x1A, 0x1B]]],
            inputs: vec![ValuePort {
                name: "rotation".to_string(),
                kind: ValueKind::Raw(RawKind::Rotation),
            }],
            callable: true,
            ..Default::default()
        },
        Prefab {
            name: "inspect_truth".to_string(),
            parts: array![[[0x1C, 0x1D]], [[0x1E, 0x1F]]],
            inputs: vec![ValuePort {
                name: "truth".to_string(),
                kind: ValueKind::Raw(RawKind::Truth),
            }],
            callable: true,
            ..Default::default()
        },
        Prefab {
            name: "inspect_object".to_string(),
            parts: array![[[0x20, 0x21]], [[0x22, 0x23]]],
            inputs: vec![ValuePort {
                name: "truth".to_string(),
                kind: ValueKind::Raw(RawKind::Object),
            }],
            callable: true,
            ..Default::default()
        },
        Prefab {
            name: "inspect_object".to_string(),
            parts: array![[[0x20, 0x21]], [[0x22, 0x23]]],
            inputs: vec![ValuePort {
                name: "truth".to_string(),
                kind: ValueKind::Raw(RawKind::Object),
            }],
            callable: true,
            ..Default::default()
        },
        Prefab {
            name: "number".to_string(),
            parts: array![[[0x24, 0x25]]],
            options: vec![Opt {
                name: "value".to_string(),
                kind: OptKind::Float32,
            }],
            outputs: vec![ValuePort {
                name: "number".to_string(),
                kind: ValueKind::Raw(RawKind::Number),
            }],
            ..Default::default()
        },
        Prefab {
            name: "vector".to_string(),
            parts: array![[[0x26, 0x27]], [[0x28, 0x29]]],
            options: vec![Opt {
                name: "value".to_string(),
                kind: OptKind::Vec,
            }],
            outputs: vec![ValuePort {
                name: "vector".to_string(),
                kind: ValueKind::Raw(RawKind::Vector),
            }],
            ..Default::default()
        },
        Prefab {
            name: "rotation".to_string(),
            parts: array![[[0x2A, 0x2B]], [[0x2C, 0x2D]]],
            options: vec![Opt {
                name: "value".to_string(),
                kind: OptKind::Vec,
            }],
            outputs: vec![ValuePort {
                name: "vector".to_string(),
                kind: ValueKind::Raw(RawKind::Vector),
            }],
            ..Default::default()
        },
        Prefab {
            name: "rotation".to_string(),
            parts: array![[[0x2A, 0x2B]], [[0x2C, 0x2D]]],
            options: vec![Opt {
                name: "value".to_string(),
                kind: OptKind::Vec,
            }],
            outputs: vec![ValuePort {
                name: "vector".to_string(),
                kind: ValueKind::Raw(RawKind::Rotation),
            }],
            ..Default::default()
        },
        Prefab {
            name: "get_number".to_string(),
            parts: array![[[0x2E, 0x2F]]],
            options: vec![Opt {
                name: "name".to_string(),
                kind: OptKind::Name,
            }],
            outputs: vec![ValuePort {
                name: "number".to_string(),
                kind: ValueKind::Reference(RawKind::Number),
            }],
            ..Default::default()
        },
        Prefab {
            name: "get_vector".to_string(),
            parts: array![[[0x30, 0x31]]],
            options: vec![Opt {
                name: "name".to_string(),
                kind: OptKind::Name,
            }],
            outputs: vec![ValuePort {
                name: "vector".to_string(),
                kind: ValueKind::Reference(RawKind::Vector),
            }],
            ..Default::default()
        },
        Prefab {
            name: "get_rotation".to_string(),
            parts: array![[[0x32, 0x33]]],
            options: vec![Opt {
                name: "name".to_string(),
                kind: OptKind::Name,
            }],
            outputs: vec![ValuePort {
                name: "rotation".to_string(),
                kind: ValueKind::Reference(RawKind::Rotation),
            }],
            ..Default::default()
        },
        Prefab {
            name: "get_truth".to_string(),
            parts: array![[[0x34, 0x35]]],
            options: vec![Opt {
                name: "name".to_string(),
                kind: OptKind::Name,
            }],
            outputs: vec![ValuePort {
                name: "truth".to_string(),
                kind: ValueKind::Reference(RawKind::Truth),
            }],
            ..Default::default()
        },
        Prefab {
            name: "get_object".to_string(),
            parts: array![[[0x36, 0x37]]],
            options: vec![Opt {
                name: "name".to_string(),
                kind: OptKind::Name,
            }],
            outputs: vec![ValuePort {
                name: "object".to_string(),
                kind: ValueKind::Reference(RawKind::Object),
            }],
            ..Default::default()
        },
        Prefab {
            name: "get_constraint".to_string(),
            parts: array![[[0x38, 0x39]]],
            options: vec![Opt {
                name: "name".to_string(),
                kind: OptKind::Name,
            }],
            outputs: vec![ValuePort {
                name: "constraint".to_string(),
                kind: ValueKind::Reference(RawKind::Constraint),
            }],
            ..Default::default()
        },
        Prefab {
            name: "set_number_list".to_string(),
            parts: array![[[0x3A, 0x3B]], [[0x3C, 0x3D]]],
            inputs: vec![
                ValuePort {
                    name: "variable".to_string(),
                    kind: ValueKind::Reference(RawKind::Number),
                },
                ValuePort {
                    name: "value".to_string(),
                    kind: ValueKind::Raw(RawKind::Number),
                },
            ],
            callable: true,
            ..Default::default()
        },
        Prefab {
            name: "set_vector_list".to_string(),
            parts: array![[[0x3E, 0x3F]], [[0x40, 0x41]]],
            inputs: vec![
                ValuePort {
                    name: "variable".to_string(),
                    kind: ValueKind::Reference(RawKind::Vector),
                },
                ValuePort {
                    name: "value".to_string(),
                    kind: ValueKind::Raw(RawKind::Vector),
                },
            ],
            callable: true,
            ..Default::default()
        },
        Prefab {
            name: "set_rotation_list".to_string(),
            parts: array![[[0x42, 0x43]], [[0x44, 0x45]]],
            inputs: vec![
                ValuePort {
                    name: "variable".to_string(),
                    kind: ValueKind::Reference(RawKind::Rotation),
                },
                ValuePort {
                    name: "value".to_string(),
                    kind: ValueKind::Raw(RawKind::Rotation),
                },
            ],
            callable: true,
            ..Default::default()
        },
        Prefab {
            name: "set_truth_list".to_string(),
            parts: array![[[0x46, 0x47]], [[0x48, 0x49]]],
            inputs: vec![
                ValuePort {
                    name: "variable".to_string(),
                    kind: ValueKind::Reference(RawKind::Truth),
                },
                ValuePort {
                    name: "value".to_string(),
                    kind: ValueKind::Raw(RawKind::Truth),
                },
            ],
            callable: true,
            ..Default::default()
        },
        Prefab {
            name: "set_object_list".to_string(),
            parts: array![[[0x4A, 0x4B]], [[0x4C, 0x4D]]],
            inputs: vec![
                ValuePort {
                    name: "variable".to_string(),
                    kind: ValueKind::Reference(RawKind::Object),
                },
                ValuePort {
                    name: "value".to_string(),
                    kind: ValueKind::Raw(RawKind::Object),
                },
            ],
            callable: true,
            ..Default::default()
        },
        Prefab {
            name: "set_constraint_list".to_string(),
            parts: array![[[0x4E, 0x4F]], [[0x50, 0x51]]],
            inputs: vec![
                ValuePort {
                    name: "variable".to_string(),
                    kind: ValueKind::Reference(RawKind::Constraint),
                },
                ValuePort {
                    name: "value".to_string(),
                    kind: ValueKind::Raw(RawKind::Constraint),
                },
            ],
            callable: true,
            ..Default::default()
        },
        Prefab {
            name: "list_number".to_string(),
            parts: array![[[0x52, 0x53]], [[0x54, 0x55]]],
            inputs: vec![
                ValuePort {
                    name: "variable".to_string(),
                    kind: ValueKind::Reference(RawKind::Number),
                },
                ValuePort {
                    name: "index".to_string(),
                    kind: ValueKind::Raw(RawKind::Number),
                },
            ],
            outputs: vec![ValuePort {
                name: "element".to_string(),
                kind: ValueKind::Reference(RawKind::Number),
            }],
            ..Default::default()
        },
        Prefab {
            name: "list_object".to_string(),
            parts: array![[[0x56, 0x57]], [[0x58, 0x59]]],
            inputs: vec![
                ValuePort {
                    name: "variable".to_string(),
                    kind: ValueKind::Reference(RawKind::Object),
                },
                ValuePort {
                    name: "index".to_string(),
                    kind: ValueKind::Raw(RawKind::Number),
                },
            ],
            outputs: vec![ValuePort {
                name: "element".to_string(),
                kind: ValueKind::Reference(RawKind::Object),
            }],
            ..Default::default()
        },
        Prefab {
            name: "negate".to_string(),
            parts: array![[[0x5A, 0x5B]]],
            inputs: vec![ValuePort {
                name: "num".to_string(),
                kind: ValueKind::Raw(RawKind::Number),
            }],
            outputs: vec![ValuePort {
                name: "negative".to_string(),
                kind: ValueKind::Raw(RawKind::Number),
            }],
            ..Default::default()
        },
        Prefab {
            name: "add_numbers".to_string(),
            parts: array![[[0x5C, 0x5D]], [[0x5E, 0x5F]]],
            inputs: vec![
                ValuePort {
                    name: "num1".to_string(),
                    kind: ValueKind::Raw(RawKind::Number),
                },
                ValuePort {
                    name: "num2".to_string(),
                    kind: ValueKind::Raw(RawKind::Number),
                },
            ],
            outputs: vec![ValuePort {
                name: "sum".to_string(),
                kind: ValueKind::Raw(RawKind::Number),
            }],
            ..Default::default()
        },
        Prefab {
            name: "add_vectors".to_string(),
            parts: array![[[0x60, 0x61]], [[0x62, 0x63]]],
            inputs: vec![
                ValuePort {
                    name: "vec1".to_string(),
                    kind: ValueKind::Raw(RawKind::Vector),
                },
                ValuePort {
                    name: "vec2".to_string(),
                    kind: ValueKind::Raw(RawKind::Vector),
                },
            ],
            outputs: vec![ValuePort {
                name: "sum".to_string(),
                kind: ValueKind::Raw(RawKind::Vector),
            }],
            ..Default::default()
        },
        Prefab {
            name: "subtract_numbers".to_string(),
            parts: array![[[0x64, 0x65]], [[0x66, 0x67]]],
            inputs: vec![
                ValuePort {
                    name: "num1".to_string(),
                    kind: ValueKind::Raw(RawKind::Number),
                },
                ValuePort {
                    name: "num2".to_string(),
                    kind: ValueKind::Raw(RawKind::Number),
                },
            ],
            outputs: vec![ValuePort {
                name: "difference".to_string(),
                kind: ValueKind::Raw(RawKind::Number),
            }],
            ..Default::default()
        },
        Prefab {
            name: "subtract_vectors".to_string(),
            parts: array![[[0x68, 0x69]], [[0x6A, 0x6B]]],
            inputs: vec![
                ValuePort {
                    name: "vec1".to_string(),
                    kind: ValueKind::Raw(RawKind::Vector),
                },
                ValuePort {
                    name: "vec2".to_string(),
                    kind: ValueKind::Raw(RawKind::Vector),
                },
            ],
            outputs: vec![ValuePort {
                name: "difference".to_string(),
                kind: ValueKind::Raw(RawKind::Vector),
            }],
            ..Default::default()
        },
        Prefab {
            name: "multiply".to_string(),
            parts: array![[[0x6C, 0x6D]], [[0x6E, 0x6F]]],
            inputs: vec![
                ValuePort {
                    name: "num1".to_string(),
                    kind: ValueKind::Raw(RawKind::Number),
                },
                ValuePort {
                    name: "num2".to_string(),
                    kind: ValueKind::Raw(RawKind::Number),
                },
            ],
            outputs: vec![ValuePort {
                name: "product".to_string(),
                kind: ValueKind::Raw(RawKind::Number),
            }],
            ..Default::default()
        },
        Prefab {
            name: "scale".to_string(),
            parts: array![[[0x70, 0x71]], [[0x72, 0x73]]],
            inputs: vec![
                ValuePort {
                    name: "vec".to_string(),
                    kind: ValueKind::Raw(RawKind::Vector),
                },
                ValuePort {
                    name: "factor".to_string(),
                    kind: ValueKind::Raw(RawKind::Number),
                },
            ],
            outputs: vec![ValuePort {
                name: "scaled".to_string(),
                kind: ValueKind::Raw(RawKind::Vector),
            }],
            ..Default::default()
        },
        Prefab {
            name: "rotate".to_string(),
            parts: array![[[0x74, 0x75]], [[0x76, 0x77]]],
            inputs: vec![
                ValuePort {
                    name: "vec".to_string(),
                    kind: ValueKind::Raw(RawKind::Vector),
                },
                ValuePort {
                    name: "rot".to_string(),
                    kind: ValueKind::Raw(RawKind::Rotation),
                },
            ],
            outputs: vec![ValuePort {
                name: "rotated".to_string(),
                kind: ValueKind::Raw(RawKind::Vector),
            }],
            ..Default::default()
        },
        Prefab {
            name: "combine".to_string(),
            parts: array![[[0x78, 0x79]], [[0x7A, 0x7B]]],
            inputs: vec![
                ValuePort {
                    name: "rot1".to_string(),
                    kind: ValueKind::Raw(RawKind::Rotation),
                },
                ValuePort {
                    name: "rot2".to_string(),
                    kind: ValueKind::Raw(RawKind::Rotation),
                },
            ],
            outputs: vec![ValuePort {
                name: "combination".to_string(),
                kind: ValueKind::Raw(RawKind::Rotation),
            }],
            ..Default::default()
        },
        Prefab {
            name: "divide".to_string(),
            parts: array![[[0x7C, 0x7D]], [[0x7E, 0x7F]]],
            inputs: vec![
                ValuePort {
                    name: "num1".to_string(),
                    kind: ValueKind::Raw(RawKind::Number),
                },
                ValuePort {
                    name: "num2".to_string(),
                    kind: ValueKind::Raw(RawKind::Number),
                },
            ],
            outputs: vec![ValuePort {
                name: "quotient".to_string(),
                kind: ValueKind::Raw(RawKind::Number),
            }],
            ..Default::default()
        },
        Prefab {
            name: "less_than".to_string(),
            parts: array![[[0x80, 0x81]], [[0x82, 0x83]]],
            inputs: vec![
                ValuePort {
                    name: "num1".to_string(),
                    kind: ValueKind::Raw(RawKind::Number),
                },
                ValuePort {
                    name: "num2".to_string(),
                    kind: ValueKind::Raw(RawKind::Number),
                },
            ],
            outputs: vec![ValuePort {
                name: "less_than".to_string(),
                kind: ValueKind::Raw(RawKind::Truth),
            }],
            ..Default::default()
        },
        Prefab {
            name: "equal_numbers".to_string(),
            parts: array![[[0x84, 0x85]], [[0x86, 0x87]]],
            inputs: vec![
                ValuePort {
                    name: "num1".to_string(),
                    kind: ValueKind::Raw(RawKind::Number),
                },
                ValuePort {
                    name: "num2".to_string(),
                    kind: ValueKind::Raw(RawKind::Number),
                },
            ],
            outputs: vec![ValuePort {
                name: "equality".to_string(),
                kind: ValueKind::Raw(RawKind::Truth),
            }],
            ..Default::default()
        },
        Prefab {
            name: "equal_vectors".to_string(),
            parts: array![[[0x88, 0x89]], [[0x8A, 0x8B]]],
            inputs: vec![
                ValuePort {
                    name: "vec1".to_string(),
                    kind: ValueKind::Raw(RawKind::Vector),
                },
                ValuePort {
                    name: "vec2".to_string(),
                    kind: ValueKind::Raw(RawKind::Vector),
                },
            ],
            outputs: vec![ValuePort {
                name: "equality".to_string(),
                kind: ValueKind::Raw(RawKind::Truth),
            }],
            ..Default::default()
        },
        Prefab {
            name: "equal_objects".to_string(),
            parts: array![[[0x8C, 0x8D]], [[0x8E, 0x8F]]],
            inputs: vec![
                ValuePort {
                    name: "obj1".to_string(),
                    kind: ValueKind::Raw(RawKind::Object),
                },
                ValuePort {
                    name: "obj2".to_string(),
                    kind: ValueKind::Raw(RawKind::Object),
                },
            ],
            outputs: vec![ValuePort {
                name: "equality".to_string(),
                kind: ValueKind::Raw(RawKind::Truth),
            }],
            ..Default::default()
        },
        Prefab {
            name: "not".to_string(),
            parts: array![[[0x90, 0x91]]],
            inputs: vec![ValuePort {
                name: "tru".to_string(),
                kind: ValueKind::Raw(RawKind::Truth),
            }],
            outputs: vec![ValuePort {
                name: "opposite".to_string(),
                kind: ValueKind::Raw(RawKind::Truth),
            }],
            ..Default::default()
        },
        Prefab {
            name: "and".to_string(),
            parts: array![[[0x92, 0x93]], [[0x94, 0x95]]],
            inputs: vec![
                ValuePort {
                    name: "tru1".to_string(),
                    kind: ValueKind::Raw(RawKind::Truth),
                },
                ValuePort {
                    name: "tru2".to_string(),
                    kind: ValueKind::Raw(RawKind::Truth),
                },
            ],
            outputs: vec![ValuePort {
                name: "conjunction".to_string(),
                kind: ValueKind::Raw(RawKind::Truth),
            }],
            ..Default::default()
        },
        Prefab {
            name: "make_vector".to_string(),
            parts: array![[[0x96, 0x97]], [[0x98, 0x99]], [[0x9A, 0x9B]]],
            inputs: vec![
                ValuePort {
                    name: "x".to_string(),
                    kind: ValueKind::Raw(RawKind::Number),
                },
                ValuePort {
                    name: "y".to_string(),
                    kind: ValueKind::Raw(RawKind::Number),
                },
                ValuePort {
                    name: "z".to_string(),
                    kind: ValueKind::Raw(RawKind::Number),
                },
            ],
            outputs: vec![ValuePort {
                name: "vector".to_string(),
                kind: ValueKind::Raw(RawKind::Vector),
            }],
            ..Default::default()
        },
        Prefab {
            name: "break_vector".to_string(),
            parts: array![[[0x9C, 0x9D]], [[0x9E, 0x9F]], [[0xA0, 0xA1]]],
            inputs: vec![ValuePort {
                name: "vector".to_string(),
                kind: ValueKind::Raw(RawKind::Vector),
            }],
            outputs: vec![
                ValuePort {
                    name: "x".to_string(),
                    kind: ValueKind::Raw(RawKind::Number),
                },
                ValuePort {
                    name: "y".to_string(),
                    kind: ValueKind::Raw(RawKind::Number),
                },
                ValuePort {
                    name: "z".to_string(),
                    kind: ValueKind::Raw(RawKind::Number),
                },
            ],
            ..Default::default()
        },
        Prefab {
            name: "make_rotation".to_string(),
            parts: array![[[0xA2, 0xA3]], [[0xA4, 0xA5]], [[0xA6, 0xA7]]],
            inputs: vec![
                ValuePort {
                    name: "x".to_string(),
                    kind: ValueKind::Raw(RawKind::Number),
                },
                ValuePort {
                    name: "y".to_string(),
                    kind: ValueKind::Raw(RawKind::Number),
                },
                ValuePort {
                    name: "z".to_string(),
                    kind: ValueKind::Raw(RawKind::Number),
                },
            ],
            outputs: vec![ValuePort {
                name: "rotation".to_string(),
                kind: ValueKind::Raw(RawKind::Rotation),
            }],
            ..Default::default()
        },
        Prefab {
            name: "random".to_string(),
            parts: array![[[0xA8, 0xA9]], [[0xAA, 0xAB]]],
            inputs: vec![
                ValuePort {
                    name: "min".to_string(),
                    kind: ValueKind::Raw(RawKind::Number),
                },
                ValuePort {
                    name: "max".to_string(),
                    kind: ValueKind::Raw(RawKind::Number),
                },
            ],
            outputs: vec![ValuePort {
                name: "random".to_string(),
                kind: ValueKind::Raw(RawKind::Number),
            }],
            ..Default::default()
        },
        Prefab {
            name: "modulo".to_string(),
            parts: array![[[0xAC, 0xAD]], [[0xAE, 0xAF]]],
            inputs: vec![
                ValuePort {
                    name: "a".to_string(),
                    kind: ValueKind::Raw(RawKind::Number),
                },
                ValuePort {
                    name: "b".to_string(),
                    kind: ValueKind::Raw(RawKind::Number),
                },
            ],
            outputs: vec![ValuePort {
                name: "remainder".to_string(),
                kind: ValueKind::Raw(RawKind::Number),
            }],
            ..Default::default()
        },
        Prefab {
            name: "min".to_string(),
            parts: array![[[0xB0, 0xB1]], [[0xB2, 0xB3]]],
            inputs: vec![
                ValuePort {
                    name: "num1".to_string(),
                    kind: ValueKind::Raw(RawKind::Number),
                },
                ValuePort {
                    name: "num2".to_string(),
                    kind: ValueKind::Raw(RawKind::Number),
                },
            ],
            outputs: vec![ValuePort {
                name: "minimum".to_string(),
                kind: ValueKind::Raw(RawKind::Number),
            }],
            ..Default::default()
        },
        Prefab {
            name: "max".to_string(),
            parts: array![[[0xB4, 0xB5]], [[0xB6, 0xB7]]],
            inputs: vec![
                ValuePort {
                    name: "num1".to_string(),
                    kind: ValueKind::Raw(RawKind::Number),
                },
                ValuePort {
                    name: "num2".to_string(),
                    kind: ValueKind::Raw(RawKind::Number),
                },
            ],
            outputs: vec![ValuePort {
                name: "maximum".to_string(),
                kind: ValueKind::Raw(RawKind::Number),
            }],
            ..Default::default()
        },
        Prefab {
            name: "round".to_string(),
            parts: array![[[0xB8, 0xB9]]],
            inputs: vec![ValuePort {
                name: "number".to_string(),
                kind: ValueKind::Raw(RawKind::Number),
            }],
            outputs: vec![ValuePort {
                name: "rounded".to_string(),
                kind: ValueKind::Raw(RawKind::Number),
            }],
            ..Default::default()
        },
        Prefab {
            name: "floor".to_string(),
            parts: array![[[0xBA, 0xBB]]],
            inputs: vec![ValuePort {
                name: "number".to_string(),
                kind: ValueKind::Raw(RawKind::Number),
            }],
            outputs: vec![ValuePort {
                name: "floor".to_string(),
                kind: ValueKind::Raw(RawKind::Number),
            }],
            ..Default::default()
        },
        Prefab {
            name: "ceiling".to_string(),
            parts: array![[[0xBC, 0xBD]]],
            inputs: vec![ValuePort {
                name: "number".to_string(),
                kind: ValueKind::Raw(RawKind::Number),
            }],
            outputs: vec![ValuePort {
                name: "ceiling".to_string(),
                kind: ValueKind::Raw(RawKind::Number),
            }],
            ..Default::default()
        },
        Prefab {
            name: "ceiling".to_string(),
            parts: array![[[0xBC, 0xBD]]],
            inputs: vec![ValuePort {
                name: "number".to_string(),
                kind: ValueKind::Raw(RawKind::Number),
            }],
            outputs: vec![ValuePort {
                name: "ceiling".to_string(),
                kind: ValueKind::Raw(RawKind::Number),
            }],
            ..Default::default()
        },
        Prefab {
            name: "distance".to_string(),
            parts: array![[[0xBE, 0xBF]], [[0xC0, 0xC1]]],
            inputs: vec![
                ValuePort {
                    name: "vec1".to_string(),
                    kind: ValueKind::Raw(RawKind::Vector),
                },
                ValuePort {
                    name: "vec2".to_string(),
                    kind: ValueKind::Raw(RawKind::Vector),
                },
            ],
            outputs: vec![ValuePort {
                name: "distance".to_string(),
                kind: ValueKind::Raw(RawKind::Number),
            }],
            ..Default::default()
        },
        Prefab {
            name: "lerp".to_string(),
            parts: array![[[0xC2, 0xC3]], [[0xC4, 0xC5]], [[0xC6, 0xC7]]],
            inputs: vec![
                ValuePort {
                    name: "from".to_string(),
                    kind: ValueKind::Raw(RawKind::Rotation),
                },
                ValuePort {
                    name: "to".to_string(),
                    kind: ValueKind::Raw(RawKind::Rotation),
                },
                ValuePort {
                    name: "amount".to_string(),
                    kind: ValueKind::Raw(RawKind::Number),
                },
            ],
            outputs: vec![ValuePort {
                name: "rotation".to_string(),
                kind: ValueKind::Raw(RawKind::Rotation),
            }],
            ..Default::default()
        },
        Prefab {
            name: "axis_angle".to_string(),
            parts: array![[[0xC8, 0xC9]], [[0xCA, 0xCB]]],
            inputs: vec![
                ValuePort {
                    name: "axis".to_string(),
                    kind: ValueKind::Raw(RawKind::Vector),
                },
                ValuePort {
                    name: "angle".to_string(),
                    kind: ValueKind::Raw(RawKind::Number),
                },
            ],
            outputs: vec![ValuePort {
                name: "rotation".to_string(),
                kind: ValueKind::Raw(RawKind::Rotation),
            }],
            ..Default::default()
        },
        Prefab {
            name: "look_rotation".to_string(),
            parts: array![[[0xCC, 0xCD]], [[0xCE, 0xCF]]],
            inputs: vec![
                ValuePort {
                    name: "direction".to_string(),
                    kind: ValueKind::Raw(RawKind::Vector),
                },
                ValuePort {
                    name: "up".to_string(),
                    kind: ValueKind::Raw(RawKind::Vector),
                },
            ],
            outputs: vec![ValuePort {
                name: "rotation".to_string(),
                kind: ValueKind::Raw(RawKind::Rotation),
            }],
            ..Default::default()
        },
        Prefab {
            name: "line_vs_plane".to_string(),
            parts: array![
                [[0xD0, 0xD1]],
                [[0xD2, 0xD3]],
                [[0xD4, 0xD5]],
                [[0xD6, 0xD7]]
            ],
            inputs: vec![
                ValuePort {
                    name: "line_from".to_string(),
                    kind: ValueKind::Raw(RawKind::Vector),
                },
                ValuePort {
                    name: "line_to".to_string(),
                    kind: ValueKind::Raw(RawKind::Vector),
                },
                ValuePort {
                    name: "plane_point".to_string(),
                    kind: ValueKind::Raw(RawKind::Vector),
                },
                ValuePort {
                    name: "plane_normal".to_string(),
                    kind: ValueKind::Raw(RawKind::Vector),
                },
            ],
            outputs: vec![ValuePort {
                name: "intersection".to_string(),
                kind: ValueKind::Raw(RawKind::Vector),
            }],
            ..Default::default()
        },
        Prefab {
            name: "screen_to_world".to_string(),
            parts: array![[[0xD8, 0xD9]], [[0xDA, 0xDB]]],
            inputs: vec![
                ValuePort {
                    name: "screen_x".to_string(),
                    kind: ValueKind::Raw(RawKind::Number),
                },
                ValuePort {
                    name: "screen_y".to_string(),
                    kind: ValueKind::Raw(RawKind::Number),
                },
            ],
            outputs: vec![
                ValuePort {
                    name: "world_near".to_string(),
                    kind: ValueKind::Raw(RawKind::Vector),
                },
                ValuePort {
                    name: "world_far".to_string(),
                    kind: ValueKind::Raw(RawKind::Vector),
                },
            ],
            ..Default::default()
        },
        Prefab {
            name: "screen_size".to_string(),
            parts: array![[[0xDC, 0xDD]], [[0xDE, 0xDF]]],
            outputs: vec![
                ValuePort {
                    name: "width".to_string(),
                    kind: ValueKind::Raw(RawKind::Number),
                },
                ValuePort {
                    name: "height".to_string(),
                    kind: ValueKind::Raw(RawKind::Number),
                },
            ],
            ..Default::default()
        },
        Prefab {
            name: "accelerometer".to_string(),
            parts: array![[[0xE0, 0xE1]], [[0xE2, 0xE3]]],
            outputs: vec![ValuePort {
                name: "direction".to_string(),
                kind: ValueKind::Raw(RawKind::Vector),
            }],
            ..Default::default()
        },
        Prefab {
            name: "raycast".to_string(),
            parts: array![[[0xE4, 0xE5]], [[0xE6, 0xE7]], [[0xE8, 0xE9]]],
            inputs: vec![
                ValuePort {
                    name: "from".to_string(),
                    kind: ValueKind::Raw(RawKind::Vector),
                },
                ValuePort {
                    name: "to".to_string(),
                    kind: ValueKind::Raw(RawKind::Vector),
                },
            ],
            outputs: vec![
                ValuePort {
                    name: "hit".to_string(),
                    kind: ValueKind::Raw(RawKind::Truth),
                },
                ValuePort {
                    name: "pos".to_string(),
                    kind: ValueKind::Raw(RawKind::Vector),
                },
                ValuePort {
                    name: "obj".to_string(),
                    kind: ValueKind::Raw(RawKind::Object),
                },
            ],
            ..Default::default()
        },
        Prefab {
            name: "if".to_string(),
            parts: array![[[0xEA, 0xEB]], [[0xEC, 0xED]]],
            inputs: vec![ValuePort {
                name: "condition".to_string(),
                kind: ValueKind::Raw(RawKind::Truth),
            }],
            callable: true,
            callbacks: vec![
                ExecutePort {
                    name: "true".to_string(),
                },
                ExecutePort {
                    name: "false".to_string(),
                },
            ],
            ..Default::default()
        },
        Prefab {
            name: "play_sensor".to_string(),
            parts: array![[[0xEE, 0xEF]], [[0xF0, 0xF1]]],
            callable: true,
            callbacks: vec![ExecutePort {
                name: "on_play".to_string(),
            }],
            ..Default::default()
        },
        Prefab {
            name: "touch_sensor".to_string(),
            parts: array![[[0xF2, 0xF3]], [[0xF4, 0xF5]], [[0xF6, 0xF7]]],
            outputs: vec![
                ValuePort {
                    name: "screen_x".to_string(),
                    kind: ValueKind::Raw(RawKind::Number),
                },
                ValuePort {
                    name: "screen_y".to_string(),
                    kind: ValueKind::Raw(RawKind::Number),
                },
            ],
            options: vec![
                Opt {
                    name: "state".to_string(),
                    kind: OptKind::Int8,
                },
                Opt {
                    name: "index".to_string(),
                    kind: OptKind::Int8,
                },
            ],
            callable: true,
            callbacks: vec![ExecutePort {
                name: "touched".to_string(),
            }],
            ..Default::default()
        },
        Prefab {
            name: "swipe_sensor".to_string(),
            parts: array![[[0xF8, 0xF9]], [[0xFA, 0xFB]]],
            outputs: vec![ValuePort {
                name: "direction".to_string(),
                kind: ValueKind::Raw(RawKind::Vector),
            }],
            callable: true,
            callbacks: vec![ExecutePort {
                name: "swiped".to_string(),
            }],
            ..Default::default()
        },
        Prefab {
            name: "win".to_string(),
            parts: array![[[0xFC, 0xFD]], [[0xFE, 0xFF]]],
            options: vec![Opt {
                name: "delay".to_string(),
                kind: OptKind::Int8,
            }],
            callable: true,
            ..Default::default()
        },
        Prefab {
            name: "lose".to_string(),
            parts: array![[[0x100, 0x101]], [[0x102, 0x103]]],
            options: vec![Opt {
                name: "delay".to_string(),
                kind: OptKind::Int8,
            }],
            callable: true,
            ..Default::default()
        },
        Prefab {
            name: "set_score".to_string(),
            parts: array![[[0x104, 0x105]], [[0x106, 0x107]]],
            inputs: vec![
                ValuePort {
                    name: "score".to_string(),
                    kind: ValueKind::Raw(RawKind::Number),
                },
                ValuePort {
                    name: "coins".to_string(),
                    kind: ValueKind::Raw(RawKind::Number),
                },
            ],
            options: vec![Opt {
                name: "order".to_string(),
                kind: OptKind::Int8,
            }],
            callable: true,
            ..Default::default()
        },
        Prefab {
            name: "play_sound".to_string(),
            parts: array![[[0x108, 0x109]], [[0x10A, 0x10B]]],
            inputs: vec![
                ValuePort {
                    name: "volume".to_string(),
                    kind: ValueKind::Raw(RawKind::Number),
                },
                ValuePort {
                    name: "pitch".to_string(),
                    kind: ValueKind::Raw(RawKind::Number),
                },
            ],
            outputs: vec![ValuePort {
                name: "channel".to_string(),
                kind: ValueKind::Raw(RawKind::Number),
            }],
            options: vec![
                Opt {
                    name: "loop".to_string(),
                    kind: OptKind::Int8,
                },
                Opt {
                    name: "sound".to_string(),
                    kind: OptKind::Int8,
                },
            ],
            callable: true,
            ..Default::default()
        },
        Prefab {
            name: "set_camera".to_string(),
            parts: array![[[0x10C, 0x10D]], [[0x10E, 0x10F]], [[0x110, 0x111]]],
            inputs: vec![
                ValuePort {
                    name: "position".to_string(),
                    kind: ValueKind::Raw(RawKind::Vector),
                },
                ValuePort {
                    name: "rotation".to_string(),
                    kind: ValueKind::Raw(RawKind::Rotation),
                },
                ValuePort {
                    name: "range".to_string(),
                    kind: ValueKind::Raw(RawKind::Number),
                },
            ],
            options: vec![Opt {
                name: "perspective".to_string(),
                kind: OptKind::Int8,
            }],
            callable: true,
            ..Default::default()
        },
        Prefab {
            name: "set_light".to_string(),
            parts: array![[[0x112, 0x113]], [[0x114, 0x115]]],
            inputs: vec![
                ValuePort {
                    name: "position".to_string(),
                    kind: ValueKind::Raw(RawKind::Vector),
                },
                ValuePort {
                    name: "rotation".to_string(),
                    kind: ValueKind::Raw(RawKind::Rotation),
                },
            ],
            callable: true,
            ..Default::default()
        },
        Prefab {
            name: "get_position".to_string(),
            parts: array![[[0x116, 0x117]], [[0x118, 0x119]]],
            inputs: vec![ValuePort {
                name: "object".to_string(),
                kind: ValueKind::Raw(RawKind::Object),
            }],
            outputs: vec![
                ValuePort {
                    name: "position".to_string(),
                    kind: ValueKind::Raw(RawKind::Vector),
                },
                ValuePort {
                    name: "rotation".to_string(),
                    kind: ValueKind::Raw(RawKind::Rotation),
                },
            ],
            ..Default::default()
        },
        Prefab {
            name: "set_position".to_string(),
            parts: array![[[0x11A, 0x11B]], [[0x11C, 0x11D]], [[0x11E, 0x11F]]],
            inputs: vec![
                ValuePort {
                    name: "object".to_string(),
                    kind: ValueKind::Raw(RawKind::Object),
                },
                ValuePort {
                    name: "position".to_string(),
                    kind: ValueKind::Raw(RawKind::Vector),
                },
                ValuePort {
                    name: "rotation".to_string(),
                    kind: ValueKind::Raw(RawKind::Rotation),
                },
            ],
            callable: true,
            ..Default::default()
        },
        Prefab {
            name: "get_velocity".to_string(),
            parts: array![[[0x120, 0x121]], [[0x122, 0x123]]],
            inputs: vec![ValuePort {
                name: "object".to_string(),
                kind: ValueKind::Raw(RawKind::Object),
            }],
            outputs: vec![
                ValuePort {
                    name: "velocity".to_string(),
                    kind: ValueKind::Raw(RawKind::Vector),
                },
                ValuePort {
                    name: "spin".to_string(),
                    kind: ValueKind::Raw(RawKind::Vector),
                },
            ],
            ..Default::default()
        },
        Prefab {
            name: "set_velocity".to_string(),
            parts: array![[[0x124, 0x125]], [[0x126, 0x127]], [[0x128, 0x129]]],
            inputs: vec![
                ValuePort {
                    name: "object".to_string(),
                    kind: ValueKind::Raw(RawKind::Object),
                },
                ValuePort {
                    name: "velocity".to_string(),
                    kind: ValueKind::Raw(RawKind::Vector),
                },
                ValuePort {
                    name: "spin".to_string(),
                    kind: ValueKind::Raw(RawKind::Vector),
                },
            ],
            callable: true,
            ..Default::default()
        },
        Prefab {
            name: "add_force".to_string(),
            parts: array![
                [[0x12A, 0x12B]],
                [[0x12C, 0x12D]],
                [[0x12E, 0x12F]],
                [[0x130, 0x131]]
            ],
            inputs: vec![
                ValuePort {
                    name: "object".to_string(),
                    kind: ValueKind::Raw(RawKind::Object),
                },
                ValuePort {
                    name: "force".to_string(),
                    kind: ValueKind::Raw(RawKind::Vector),
                },
                ValuePort {
                    name: "apply_at".to_string(),
                    kind: ValueKind::Raw(RawKind::Vector),
                },
                ValuePort {
                    name: "torque".to_string(),
                    kind: ValueKind::Raw(RawKind::Vector),
                },
            ],
            callable: true,
            ..Default::default()
        },
        Prefab {
            name: "set_visible".to_string(),
            parts: array![[[0x132, 0x133]], [[0x134, 0x135]]],
            inputs: vec![
                ValuePort {
                    name: "object".to_string(),
                    kind: ValueKind::Raw(RawKind::Object),
                },
                ValuePort {
                    name: "visible".to_string(),
                    kind: ValueKind::Raw(RawKind::Truth),
                },
            ],
            callable: true,
            ..Default::default()
        },
        Prefab {
            name: "set_locked".to_string(),
            parts: array![[[0x136, 0x137]], [[0x138, 0x139]], [[0x13A, 0x13B]]],
            inputs: vec![
                ValuePort {
                    name: "object".to_string(),
                    kind: ValueKind::Raw(RawKind::Object),
                },
                ValuePort {
                    name: "position".to_string(),
                    kind: ValueKind::Raw(RawKind::Vector),
                },
                ValuePort {
                    name: "rotation".to_string(),
                    kind: ValueKind::Raw(RawKind::Vector),
                },
            ],
            callable: true,
            ..Default::default()
        },
        Prefab {
            name: "create_object".to_string(),
            parts: array![[[0x13C, 0x13D]], [[0x13E, 0x13F]]],
            inputs: vec![ValuePort {
                name: "original".to_string(),
                kind: ValueKind::Raw(RawKind::Object),
            }],
            outputs: vec![ValuePort {
                name: "copy".to_string(),
                kind: ValueKind::Raw(RawKind::Object),
            }],
            callable: true,
            ..Default::default()
        },
        Prefab {
            name: "destroy_object".to_string(),
            parts: array![[[0x140, 0x141]], [[0x142, 0x143]]],
            inputs: vec![ValuePort {
                name: "object".to_string(),
                kind: ValueKind::Raw(RawKind::Object),
            }],
            callable: true,
            ..Default::default()
        },
        Prefab {
            name: "set_gravity".to_string(),
            parts: array![[[0x144, 0x145]], [[0x146, 0x147]]],
            inputs: vec![ValuePort {
                name: "gravity".to_string(),
                kind: ValueKind::Raw(RawKind::Vector),
            }],
            callable: true,
            ..Default::default()
        },
        Prefab {
            name: "set_mass".to_string(),
            parts: array![[[0x148, 0x149]], [[0x14A, 0x14B]]],
            inputs: vec![
                ValuePort {
                    name: "object".to_string(),
                    kind: ValueKind::Raw(RawKind::Object),
                },
                ValuePort {
                    name: "mass".to_string(),
                    kind: ValueKind::Raw(RawKind::Number),
                },
            ],
            callable: true,
            ..Default::default()
        },
        Prefab {
            name: "set_friction".to_string(),
            parts: array![[[0x14C, 0x14D]], [[0x14E, 0x14F]]],
            inputs: vec![
                ValuePort {
                    name: "object".to_string(),
                    kind: ValueKind::Raw(RawKind::Object),
                },
                ValuePort {
                    name: "friction".to_string(),
                    kind: ValueKind::Raw(RawKind::Number),
                },
            ],
            callable: true,
            ..Default::default()
        },
        Prefab {
            name: "set_bounciness".to_string(),
            parts: array![[[0x150, 0x151]], [[0x152, 0x153]]],
            inputs: vec![
                ValuePort {
                    name: "object".to_string(),
                    kind: ValueKind::Raw(RawKind::Object),
                },
                ValuePort {
                    name: "bounciness".to_string(),
                    kind: ValueKind::Raw(RawKind::Number),
                },
            ],
            callable: true,
            ..Default::default()
        },
        Prefab {
            name: "add_constraint".to_string(),
            parts: array![[[0x154, 0x155]], [[0x156, 0x157]], [[0x158, 0x159]]],
            inputs: vec![
                ValuePort {
                    name: "base".to_string(),
                    kind: ValueKind::Raw(RawKind::Object),
                },
                ValuePort {
                    name: "part".to_string(),
                    kind: ValueKind::Raw(RawKind::Object),
                },
                ValuePort {
                    name: "pivot".to_string(),
                    kind: ValueKind::Raw(RawKind::Vector),
                },
            ],
            callable: true,
            ..Default::default()
        },
        Prefab {
            name: "linear_limits".to_string(),
            parts: array![[[0x15A, 0x15B]], [[0x15C, 0x15D]], [[0x15E, 0x15F]]],
            inputs: vec![
                ValuePort {
                    name: "constraint".to_string(),
                    kind: ValueKind::Raw(RawKind::Constraint),
                },
                ValuePort {
                    name: "lower".to_string(),
                    kind: ValueKind::Raw(RawKind::Vector),
                },
                ValuePort {
                    name: "upper".to_string(),
                    kind: ValueKind::Raw(RawKind::Vector),
                },
            ],
            callable: true,
            ..Default::default()
        },
        Prefab {
            name: "angular_limits".to_string(),
            parts: array![[[0x60, 0x161]], [[0x162, 0x163]], [[0x164, 0x165]]],
            inputs: vec![
                ValuePort {
                    name: "constraint".to_string(),
                    kind: ValueKind::Raw(RawKind::Constraint),
                },
                ValuePort {
                    name: "lower".to_string(),
                    kind: ValueKind::Raw(RawKind::Vector),
                },
                ValuePort {
                    name: "upper".to_string(),
                    kind: ValueKind::Raw(RawKind::Vector),
                },
            ],
            callable: true,
            ..Default::default()
        },
        Prefab {
            name: "linear_spring".to_string(),
            parts: array![[[0x166, 0x167]], [[0x168, 0x169]], [[0x16A, 0x16B]]],
            inputs: vec![
                ValuePort {
                    name: "constraint".to_string(),
                    kind: ValueKind::Raw(RawKind::Constraint),
                },
                ValuePort {
                    name: "stiffness".to_string(),
                    kind: ValueKind::Raw(RawKind::Vector),
                },
                ValuePort {
                    name: "damping".to_string(),
                    kind: ValueKind::Raw(RawKind::Vector),
                },
            ],
            callable: true,
            ..Default::default()
        },
        Prefab {
            name: "angular_spring".to_string(),
            parts: array![[[0x16C, 0x16D]], [[0x16E, 0x16F]], [[0x170, 0x171]]],
            inputs: vec![
                ValuePort {
                    name: "constraint".to_string(),
                    kind: ValueKind::Raw(RawKind::Constraint),
                },
                ValuePort {
                    name: "stiffness".to_string(),
                    kind: ValueKind::Raw(RawKind::Vector),
                },
                ValuePort {
                    name: "damping".to_string(),
                    kind: ValueKind::Raw(RawKind::Vector),
                },
            ],
            callable: true,
            ..Default::default()
        },
        Prefab {
            name: "linear_motor".to_string(),
            parts: array![[[0x172, 0x173]], [[0x174, 0x175]], [[0x176, 0x177]]],
            inputs: vec![
                ValuePort {
                    name: "constraint".to_string(),
                    kind: ValueKind::Raw(RawKind::Constraint),
                },
                ValuePort {
                    name: "speed".to_string(),
                    kind: ValueKind::Raw(RawKind::Vector),
                },
                ValuePort {
                    name: "force".to_string(),
                    kind: ValueKind::Raw(RawKind::Vector),
                },
            ],
            callable: true,
            ..Default::default()
        },
        Prefab {
            name: "angular_motor".to_string(),
            parts: array![[[0x178, 0x179]], [[0x17A, 0x17B]], [[0x17C, 0x17D]]],
            inputs: vec![
                ValuePort {
                    name: "constraint".to_string(),
                    kind: ValueKind::Raw(RawKind::Constraint),
                },
                ValuePort {
                    name: "speed".to_string(),
                    kind: ValueKind::Raw(RawKind::Vector),
                },
                ValuePort {
                    name: "force".to_string(),
                    kind: ValueKind::Raw(RawKind::Vector),
                },
            ],
            callable: true,
            ..Default::default()
        },
        Prefab {
            name: "folder_unknown".to_string(),
            parts: array![[[0x17E]]],
            outputs: vec![ValuePort {
                name: "object".to_string(),
                kind: ValueKind::Raw(RawKind::Object),
            }],
            ..Default::default()
        },
        Prefab {
            name: "swipe_chick".to_string(),
            parts: array![[[0x17F]]],
            outputs: vec![ValuePort {
                name: "object".to_string(),
                kind: ValueKind::Raw(RawKind::Object),
            }],
            ..Default::default()
        },
        Prefab {
            name: "drirt_slab".to_string(),
            parts: array![[[0x180]]],
            outputs: vec![ValuePort {
                name: "object".to_string(),
                kind: ValueKind::Raw(RawKind::Object),
            }],
            ..Default::default()
        },
        Prefab {
            name: "steel".to_string(),
            parts: array![[[0x181]]],
            outputs: vec![ValuePort {
                name: "object".to_string(),
                kind: ValueKind::Raw(RawKind::Object),
            }],
            ..Default::default()
        },
        Prefab {
            name: "arch".to_string(),
            parts: array![[[0x182], [0x183]]],
            outputs: vec![ValuePort {
                name: "object".to_string(),
                kind: ValueKind::Raw(RawKind::Object),
            }],
            ..Default::default()
        },
        Prefab {
            name: "box".to_string(),
            parts: array![[[0x184]]],
            outputs: vec![ValuePort {
                name: "object".to_string(),
                kind: ValueKind::Raw(RawKind::Object),
            }],
            ..Default::default()
        },
        Prefab {
            name: "goal".to_string(),
            parts: array![[[0x185]]],
            outputs: vec![ValuePort {
                name: "object".to_string(),
                kind: ValueKind::Raw(RawKind::Object),
            }],
            ..Default::default()
        },
        Prefab {
            name: "button_block".to_string(),
            parts: array![[[0x186]]],
            outputs: vec![
                ValuePort {
                    // TODO: Offset
                    name: "on".to_string(),
                    kind: ValueKind::Raw(RawKind::Truth),
                },
                ValuePort {
                    // TODO: Offset
                    name: "object".to_string(),
                    kind: ValueKind::Raw(RawKind::Object),
                },
            ],
            ..Default::default()
        },
        Prefab {
            name: "volume_pitch".to_string(),
            parts: array![[[0x187, 0x188]], [[0x189, 0x18A]], [[0x18B, 0x18C]]],
            inputs: vec![
                ValuePort {
                    name: "channel".to_string(),
                    kind: ValueKind::Raw(RawKind::Number),
                },
                ValuePort {
                    name: "volume".to_string(),
                    kind: ValueKind::Raw(RawKind::Number),
                },
                ValuePort {
                    name: "pitch".to_string(),
                    kind: ValueKind::Raw(RawKind::Number),
                },
            ],
            callable: true,
            ..Default::default()
        },
        Prefab {
            name: "stop_sound".to_string(),
            parts: array![[[0x18D, 0x18E]], [[0x18F, 0x190]]],
            inputs: vec![ValuePort {
                name: "channel".to_string(),
                kind: ValueKind::Raw(RawKind::Number),
            }],
            callable: true,
            ..Default::default()
        },
        Prefab {
            name: "collision".to_string(),
            parts: array![
                [[0x191, 0x192]],
                [[0x193, 0x194]],
                [[0x195, 0x196]],
                [[0x197, 0x198]]
            ],
            inputs: vec![ValuePort {
                name: "object".to_string(),
                kind: ValueKind::Raw(RawKind::Object),
            }],
            outputs: vec![
                ValuePort {
                    name: "object".to_string(),
                    kind: ValueKind::Raw(RawKind::Object),
                },
                ValuePort {
                    name: "impulse".to_string(),
                    kind: ValueKind::Raw(RawKind::Number),
                },
                ValuePort {
                    name: "normal".to_string(),
                    kind: ValueKind::Raw(RawKind::Vector),
                },
            ],
            callable: true,
            callbacks: vec![ExecutePort {
                name: "collided".to_string(),
            }],
            ..Default::default()
        },
        Prefab {
            name: "box_art_sensor".to_string(),
            parts: array![[[0x199, 0x19A]], [[0x19B, 0x19C]]],
            callable: true,
            callbacks: vec![ExecutePort {
                name: "on_screenshot".to_string(),
            }],
            ..Default::default()
        },
        Prefab {
            name: "sin".to_string(),
            parts: array![[[0x19D, 0x19E]]],
            inputs: vec![ValuePort {
                name: "num".to_string(),
                kind: ValueKind::Raw(RawKind::Number),
            }],
            outputs: vec![ValuePort {
                name: "sin".to_string(),
                kind: ValueKind::Raw(RawKind::Number),
            }],
            ..Default::default()
        },
        Prefab {
            name: "folder_empty".to_string(),
            parts: array![[[0x19F]]],
            outputs: vec![ValuePort {
                name: "object".to_string(),
                kind: ValueKind::Raw(RawKind::Object),
            }],
            ..Default::default()
        },
        Prefab {
            name: "folder_locked".to_string(),
            parts: array![[[0x1A0]]],
            outputs: vec![ValuePort {
                name: "object".to_string(),
                kind: ValueKind::Raw(RawKind::Object),
            }],
            ..Default::default()
        },
        Prefab {
            name: "or".to_string(),
            parts: array![[[0x1A1, 0x1A2]], [[0x1A3, 0x1A4]]],
            inputs: vec![
                ValuePort {
                    name: "tru1".to_string(),
                    kind: ValueKind::Raw(RawKind::Truth),
                },
                ValuePort {
                    name: "tru2".to_string(),
                    kind: ValueKind::Raw(RawKind::Truth),
                },
            ],
            outputs: vec![ValuePort {
                name: "disjunction".to_string(),
                kind: ValueKind::Raw(RawKind::Truth),
            }],
            ..Default::default()
        },
        Prefab {
            name: "equal_truths".to_string(),
            parts: array![[[0x1A5, 0x1A6]], [[0x1A7, 0x1A8]]],
            inputs: vec![
                ValuePort {
                    name: "tru1".to_string(),
                    kind: ValueKind::Raw(RawKind::Truth),
                },
                ValuePort {
                    name: "tru2".to_string(),
                    kind: ValueKind::Raw(RawKind::Truth),
                },
            ],
            outputs: vec![ValuePort {
                name: "equality".to_string(),
                kind: ValueKind::Raw(RawKind::Truth),
            }],
            ..Default::default()
        },
        Prefab {
            name: "physics_box".to_string(),
            parts: array![[[0x1A9]]],
            outputs: vec![ValuePort {
                name: "object".to_string(),
                kind: ValueKind::Raw(RawKind::Object),
            }],
            ..Default::default()
        },
        Prefab {
            name: "physics_sphere".to_string(),
            parts: array![[[0x1AA]]],
            outputs: vec![ValuePort {
                name: "object".to_string(),
                kind: ValueKind::Raw(RawKind::Object),
            }],
            ..Default::default()
        },
        Prefab {
            name: "tilt_ball".to_string(),
            parts: array![[[0x1AB]]],
            outputs: vec![ValuePort {
                name: "object".to_string(),
                kind: ValueKind::Raw(RawKind::Object),
            }],
            ..Default::default()
        },
        Prefab {
            name: "set_number".to_string(),
            parts: array![[[0x1AC, 0x1AD]]],
            inputs: vec![ValuePort {
                name: "value".to_string(),
                kind: ValueKind::Raw(RawKind::Number),
            }],
            options: vec![Opt {
                name: "name".to_string(),
                kind: OptKind::Name,
            }],
            callable: true,
            ..Default::default()
        },
        Prefab {
            name: "set_vector".to_string(),
            parts: array![[[0x1AE, 0x1AF]]],
            inputs: vec![ValuePort {
                name: "value".to_string(),
                kind: ValueKind::Raw(RawKind::Vector),
            }],
            options: vec![Opt {
                name: "name".to_string(),
                kind: OptKind::Name,
            }],
            callable: true,
            ..Default::default()
        },
        Prefab {
            name: "set_rotation".to_string(),
            parts: array![[[0x1B0, 0x1B1]]],
            inputs: vec![ValuePort {
                name: "value".to_string(),
                kind: ValueKind::Raw(RawKind::Rotation),
            }],
            options: vec![Opt {
                name: "name".to_string(),
                kind: OptKind::Name,
            }],
            callable: true,
            ..Default::default()
        },
        Prefab {
            name: "set_truth".to_string(),
            parts: array![[[0x1B2, 0x1B3]]],
            inputs: vec![ValuePort {
                name: "value".to_string(),
                kind: ValueKind::Raw(RawKind::Truth),
            }],
            options: vec![Opt {
                name: "name".to_string(),
                kind: OptKind::Name,
            }],
            callable: true,
            ..Default::default()
        },
        Prefab {
            name: "set_object".to_string(),
            parts: array![[[0x1B4, 0x1B5]]],
            inputs: vec![ValuePort {
                name: "value".to_string(),
                kind: ValueKind::Raw(RawKind::Object),
            }],
            options: vec![Opt {
                name: "name".to_string(),
                kind: OptKind::Name,
            }],
            callable: true,
            ..Default::default()
        },
        Prefab {
            name: "set_constraint".to_string(),
            parts: array![[[0x1B6, 0x1B7]]],
            inputs: vec![ValuePort {
                name: "value".to_string(),
                kind: ValueKind::Raw(RawKind::Constraint),
            }],
            options: vec![Opt {
                name: "name".to_string(),
                kind: OptKind::Name,
            }],
            callable: true,
            ..Default::default()
        },
        Prefab {
            name: "inverse".to_string(),
            parts: array![[[0x1B8, 0x1B9]]],
            inputs: vec![ValuePort {
                name: "rot".to_string(),
                kind: ValueKind::Raw(RawKind::Rotation),
            }],
            outputs: vec![ValuePort {
                name: "inverse".to_string(),
                kind: ValueKind::Raw(RawKind::Rotation),
            }],
            ..Default::default()
        },
        Prefab {
            name: "break_rotation".to_string(),
            parts: array![[[0x1BA, 0x1BB]], [[0x1BC, 0x1BD]], [[0x1BE, 0x1BF]]],
            inputs: vec![ValuePort {
                name: "rotation".to_string(),
                kind: ValueKind::Raw(RawKind::Rotation),
            }],
            outputs: vec![
                ValuePort {
                    name: "x".to_string(),
                    kind: ValueKind::Raw(RawKind::Number),
                },
                ValuePort {
                    name: "y".to_string(),
                    kind: ValueKind::Raw(RawKind::Number),
                },
                ValuePort {
                    name: "z".to_string(),
                    kind: ValueKind::Raw(RawKind::Number),
                },
            ],
            ..Default::default()
        },
        Prefab {
            name: "pass_through".to_string(),
            parts: array![[[0x1C0]]],
            outputs: vec![ValuePort {
                name: "object".to_string(),
                kind: ValueKind::Raw(RawKind::Object),
            }],
            ..Default::default()
        },
        Prefab {
            name: "true".to_string(),
            parts: array![[[0x1C1, 0x1C2]]],
            outputs: vec![ValuePort {
                name: "true".to_string(),
                kind: ValueKind::Raw(RawKind::Truth),
            }],
            ..Default::default()
        },
        Prefab {
            name: "false".to_string(),
            parts: array![[[0x1C3, 0x1C4]]],
            outputs: vec![ValuePort {
                name: "false".to_string(),
                kind: ValueKind::Raw(RawKind::Truth),
            }],
            ..Default::default()
        },
        Prefab {
            name: "cos".to_string(),
            parts: array![[[0x1C5, 0x1C6]]],
            inputs: vec![ValuePort {
                name: "num".to_string(),
                kind: ValueKind::Raw(RawKind::Number),
            }],
            outputs: vec![ValuePort {
                name: "cos".to_string(),
                kind: ValueKind::Raw(RawKind::Number),
            }],
            ..Default::default()
        },
        Prefab {
            name: "absolute".to_string(),
            parts: array![[[0x1C7, 0x1C8]]],
            inputs: vec![ValuePort {
                name: "num".to_string(),
                kind: ValueKind::Raw(RawKind::Number),
            }],
            outputs: vec![ValuePort {
                name: "absolute".to_string(),
                kind: ValueKind::Raw(RawKind::Number),
            }],
            ..Default::default()
        },
        Prefab {
            name: "power".to_string(),
            parts: array![[[0x1C9, 0x1CA]], [[0x1CB, 0x1CC]]],
            inputs: vec![
                ValuePort {
                    name: "base".to_string(),
                    kind: ValueKind::Raw(RawKind::Number),
                },
                ValuePort {
                    name: "exponent".to_string(),
                    kind: ValueKind::Raw(RawKind::Number),
                },
            ],
            outputs: vec![ValuePort {
                name: "power".to_string(),
                kind: ValueKind::Raw(RawKind::Number),
            }],
            ..Default::default()
        },
        Prefab {
            name: "list_vector".to_string(),
            parts: array![[[0x1CD, 0x1CE]], [[0x1CF, 0x1D0]]],
            inputs: vec![
                ValuePort {
                    name: "variable".to_string(),
                    kind: ValueKind::Reference(RawKind::Vector),
                },
                ValuePort {
                    name: "index".to_string(),
                    kind: ValueKind::Raw(RawKind::Number),
                },
            ],
            outputs: vec![ValuePort {
                name: "element".to_string(),
                kind: ValueKind::Reference(RawKind::Vector),
            }],
            ..Default::default()
        },
        Prefab {
            name: "list_rotation".to_string(),
            parts: array![[[0x1D1, 0x1D2]], [[0x1D3, 0x1D4]]],
            inputs: vec![
                ValuePort {
                    name: "variable".to_string(),
                    kind: ValueKind::Reference(RawKind::Rotation),
                },
                ValuePort {
                    name: "index".to_string(),
                    kind: ValueKind::Raw(RawKind::Number),
                },
            ],
            outputs: vec![ValuePort {
                name: "element".to_string(),
                kind: ValueKind::Reference(RawKind::Rotation),
            }],
            ..Default::default()
        },
        Prefab {
            name: "list_truth".to_string(),
            parts: array![[[0x1D5, 0x1D6]], [[0x1D7, 0x1D8]]],
            inputs: vec![
                ValuePort {
                    name: "variable".to_string(),
                    kind: ValueKind::Reference(RawKind::Truth),
                },
                ValuePort {
                    name: "index".to_string(),
                    kind: ValueKind::Raw(RawKind::Number),
                },
            ],
            outputs: vec![ValuePort {
                name: "element".to_string(),
                kind: ValueKind::Reference(RawKind::Truth),
            }],
            ..Default::default()
        },
        Prefab {
            name: "list_constraint".to_string(),
            parts: array![[[0x1D9, 0x1DA]], [[0x1DB, 0x1DC]]],
            inputs: vec![
                ValuePort {
                    name: "variable".to_string(),
                    kind: ValueKind::Reference(RawKind::Constraint),
                },
                ValuePort {
                    name: "index".to_string(),
                    kind: ValueKind::Raw(RawKind::Number),
                },
            ],
            outputs: vec![ValuePort {
                name: "element".to_string(),
                kind: ValueKind::Reference(RawKind::Constraint),
            }],
            ..Default::default()
        },
        Prefab {
            name: "world_to_screen".to_string(),
            parts: array![[[0x1DD, 0x1DE]], [[0x1DF, 0x1E0]]],
            inputs: vec![ValuePort {
                name: "world_pos".to_string(),
                kind: ValueKind::Raw(RawKind::Vector),
            }],
            outputs: vec![
                ValuePort {
                    name: "screen_x".to_string(),
                    kind: ValueKind::Reference(RawKind::Number),
                },
                ValuePort {
                    name: "screen_y".to_string(),
                    kind: ValueKind::Reference(RawKind::Number),
                },
            ],
            ..Default::default()
        },
        Prefab {
            name: "greater_than".to_string(),
            parts: array![[[0x1E1, 0x1E2]], [[0x1E3, 0x1E4]]],
            inputs: vec![
                ValuePort {
                    name: "num1".to_string(),
                    kind: ValueKind::Raw(RawKind::Number),
                },
                ValuePort {
                    name: "num2".to_string(),
                    kind: ValueKind::Raw(RawKind::Number),
                },
            ],
            outputs: vec![ValuePort {
                name: "greater_than".to_string(),
                kind: ValueKind::Reference(RawKind::Number),
            }],
            ..Default::default()
        },
        Prefab {
            name: "random_seed".to_string(),
            parts: array![[[0x1E5, 0x1E6]], [[0x1E7, 0x1E8]]],
            inputs: vec![ValuePort {
                name: "seed".to_string(),
                kind: ValueKind::Raw(RawKind::Number),
            }],
            callable: true,
            ..Default::default()
        },
        Prefab {
            name: "get_size".to_string(),
            parts: array![[[0x1E9, 0x1EA]], [[0x1EB, 0x1EC]]],
            inputs: vec![ValuePort {
                name: "object".to_string(),
                kind: ValueKind::Raw(RawKind::Object),
            }],
            outputs: vec![
                ValuePort {
                    name: "min".to_string(),
                    kind: ValueKind::Raw(RawKind::Vector),
                },
                ValuePort {
                    name: "max".to_string(),
                    kind: ValueKind::Raw(RawKind::Vector),
                },
            ],
            ..Default::default()
        },
        Prefab {
            name: "dotted_dirt".to_string(),
            parts: array![[[0x1ED]]],
            outputs: vec![ValuePort {
                name: "object".to_string(),
                kind: ValueKind::Raw(RawKind::Object),
            }],
            ..Default::default()
        },
        Prefab {
            name: "flowers".to_string(),
            parts: array![[[0x1EE]]],
            outputs: vec![ValuePort {
                name: "object".to_string(),
                kind: ValueKind::Raw(RawKind::Object),
            }],
            ..Default::default()
        },
        Prefab {
            name: "foliage".to_string(),
            parts: array![[[0x1EF]]],
            outputs: vec![ValuePort {
                name: "object".to_string(),
                kind: ValueKind::Raw(RawKind::Object),
            }],
            ..Default::default()
        },
        Prefab {
            name: "spotted_foliage".to_string(),
            parts: array![[[0x1F0]]],
            outputs: vec![ValuePort {
                name: "object".to_string(),
                kind: ValueKind::Raw(RawKind::Object),
            }],
            ..Default::default()
        },
        Prefab {
            name: "foliage_top".to_string(),
            parts: array![[[0x1F1]]],
            outputs: vec![ValuePort {
                name: "object".to_string(),
                kind: ValueKind::Raw(RawKind::Object),
            }],
            ..Default::default()
        },
        Prefab {
            name: "foliage_bottom".to_string(),
            parts: array![[[0x1F2]]],
            outputs: vec![ValuePort {
                name: "object".to_string(),
                kind: ValueKind::Raw(RawKind::Object),
            }],
            ..Default::default()
        },
        Prefab {
            name: "foliage_slab".to_string(),
            parts: array![[[0x1F3]]],
            outputs: vec![ValuePort {
                name: "object".to_string(),
                kind: ValueKind::Raw(RawKind::Object),
            }],
            ..Default::default()
        },
        Prefab {
            name: "shrub".to_string(),
            parts: array![[[0x1F4]]],
            outputs: vec![ValuePort {
                name: "object".to_string(),
                kind: ValueKind::Raw(RawKind::Object),
            }],
            ..Default::default()
        },
        Prefab {
            name: "stone".to_string(),
            parts: array![[[0x1F5]]],
            outputs: vec![ValuePort {
                name: "object".to_string(),
                kind: ValueKind::Raw(RawKind::Object),
            }],
            ..Default::default()
        },
        Prefab {
            name: "dotted_stone".to_string(),
            parts: array![[[0x1F6]]],
            outputs: vec![ValuePort {
                name: "object".to_string(),
                kind: ValueKind::Raw(RawKind::Object),
            }],
            ..Default::default()
        },
        Prefab {
            name: "stone_top".to_string(),
            parts: array![[[0x1F7]]],
            outputs: vec![ValuePort {
                name: "object".to_string(),
                kind: ValueKind::Raw(RawKind::Object),
            }],
            ..Default::default()
        },
        Prefab {
            name: "stone_bottom".to_string(),
            parts: array![[[0x1F8]]],
            outputs: vec![ValuePort {
                name: "object".to_string(),
                kind: ValueKind::Raw(RawKind::Object),
            }],
            ..Default::default()
        },
        Prefab {
            name: "stone_slab".to_string(),
            parts: array![[[0x1F9]]],
            outputs: vec![ValuePort {
                name: "object".to_string(),
                kind: ValueKind::Raw(RawKind::Object),
            }],
            ..Default::default()
        },
        Prefab {
            name: "stone_pillar".to_string(),
            parts: array![[[0x1FA]]],
            outputs: vec![ValuePort {
                name: "object".to_string(),
                kind: ValueKind::Raw(RawKind::Object),
            }],
            ..Default::default()
        },
        Prefab {
            name: "stone_half".to_string(),
            parts: array![[[0x1FB]]],
            outputs: vec![ValuePort {
                name: "object".to_string(),
                kind: ValueKind::Raw(RawKind::Object),
            }],
            ..Default::default()
        },
        Prefab {
            name: "wood_half_bottom_x".to_string(),
            parts: array![[[0x1FC]]],
            outputs: vec![ValuePort {
                name: "object".to_string(),
                kind: ValueKind::Raw(RawKind::Object),
            }],
            ..Default::default()
        },
        Prefab {
            name: "wood_half_bottom_z".to_string(),
            parts: array![[[0x1FD]]],
            outputs: vec![ValuePort {
                name: "object".to_string(),
                kind: ValueKind::Raw(RawKind::Object),
            }],
            ..Default::default()
        },
        Prefab {
            name: "wood_half_top_x".to_string(),
            parts: array![[[0x1FE]]],
            outputs: vec![ValuePort {
                name: "object".to_string(),
                kind: ValueKind::Raw(RawKind::Object),
            }],
            ..Default::default()
        },
        Prefab {
            name: "wood_half_top_z".to_string(),
            parts: array![[[0x1FF]]],
            outputs: vec![ValuePort {
                name: "object".to_string(),
                kind: ValueKind::Raw(RawKind::Object),
            }],
            ..Default::default()
        },
        Prefab {
            name: "stick_x".to_string(),
            parts: array![[[0x200]]],
            outputs: vec![ValuePort {
                name: "object".to_string(),
                kind: ValueKind::Raw(RawKind::Object),
            }],
            ..Default::default()
        },
        Prefab {
            name: "stick_y".to_string(),
            parts: array![[[0x201]]],
            outputs: vec![ValuePort {
                name: "object".to_string(),
                kind: ValueKind::Raw(RawKind::Object),
            }],
            ..Default::default()
        },
        Prefab {
            name: "stick_z".to_string(),
            parts: array![[[0x202]]],
            outputs: vec![ValuePort {
                name: "object".to_string(),
                kind: ValueKind::Raw(RawKind::Object),
            }],
            ..Default::default()
        },
        Prefab {
            name: "stick_north_east".to_string(),
            parts: array![[[0x203]]],
            outputs: vec![ValuePort {
                name: "object".to_string(),
                kind: ValueKind::Raw(RawKind::Object),
            }],
            ..Default::default()
        },
        Prefab {
            name: "stick_north_west".to_string(),
            parts: array![[[0x204]]],
            outputs: vec![ValuePort {
                name: "object".to_string(),
                kind: ValueKind::Raw(RawKind::Object),
            }],
            ..Default::default()
        },
        Prefab {
            name: "stick_south_west".to_string(),
            parts: array![[[0x205]]],
            outputs: vec![ValuePort {
                name: "object".to_string(),
                kind: ValueKind::Raw(RawKind::Object),
            }],
            ..Default::default()
        },
        Prefab {
            name: "stick_south_east".to_string(),
            parts: array![[[0x206]]],
            outputs: vec![ValuePort {
                name: "object".to_string(),
                kind: ValueKind::Raw(RawKind::Object),
            }],
            ..Default::default()
        },
        Prefab {
            name: "stick_top_east".to_string(),
            parts: array![[[0x207]]],
            outputs: vec![ValuePort {
                name: "object".to_string(),
                kind: ValueKind::Raw(RawKind::Object),
            }],
            ..Default::default()
        },
        Prefab {
            name: "stick_top_north".to_string(),
            parts: array![[[0x208]]],
            outputs: vec![ValuePort {
                name: "object".to_string(),
                kind: ValueKind::Raw(RawKind::Object),
            }],
            ..Default::default()
        },
        Prefab {
            name: "stick_top_west".to_string(),
            parts: array![[[0x209]]],
            outputs: vec![ValuePort {
                name: "object".to_string(),
                kind: ValueKind::Raw(RawKind::Object),
            }],
            ..Default::default()
        },
        Prefab {
            name: "stick_top_south".to_string(),
            parts: array![[[0x20A]]],
            outputs: vec![ValuePort {
                name: "object".to_string(),
                kind: ValueKind::Raw(RawKind::Object),
            }],
            ..Default::default()
        },
        Prefab {
            name: "stick_bottom_east".to_string(),
            parts: array![[[0x20B]]],
            outputs: vec![ValuePort {
                name: "object".to_string(),
                kind: ValueKind::Raw(RawKind::Object),
            }],
            ..Default::default()
        },
        Prefab {
            name: "stick_bottom_north".to_string(),
            parts: array![[[0x20C]]],
            outputs: vec![ValuePort {
                name: "object".to_string(),
                kind: ValueKind::Raw(RawKind::Object),
            }],
            ..Default::default()
        },
        Prefab {
            name: "stick_bottom_west".to_string(),
            parts: array![[[0x20D]]],
            outputs: vec![ValuePort {
                name: "object".to_string(),
                kind: ValueKind::Raw(RawKind::Object),
            }],
            ..Default::default()
        },
        Prefab {
            name: "stick_bottom_south".to_string(),
            parts: array![[[0x20E]]],
            outputs: vec![ValuePort {
                name: "object".to_string(),
                kind: ValueKind::Raw(RawKind::Object),
            }],
            ..Default::default()
        },
        Prefab {
            name: "motor_x".to_string(),
            parts: array![[[0x20F]]],
            inputs: vec![ValuePort {
                // TODO: Offset
                name: "speed".to_string(),
                kind: ValueKind::Raw(RawKind::Number),
            }],
            outputs: vec![ValuePort {
                name: "object".to_string(),
                kind: ValueKind::Raw(RawKind::Object),
            }],
            ..Default::default()
        },
        Prefab {
            name: "motor_y".to_string(),
            parts: array![[[0x210]]],
            inputs: vec![ValuePort {
                // TODO: Offset
                name: "speed".to_string(),
                kind: ValueKind::Raw(RawKind::Number),
            }],
            outputs: vec![ValuePort {
                name: "object".to_string(),
                kind: ValueKind::Raw(RawKind::Object),
            }],
            ..Default::default()
        },
        Prefab {
            name: "motor_z".to_string(),
            parts: array![[[0x211]]],
            inputs: vec![ValuePort {
                // TODO: Offset
                name: "speed".to_string(),
                kind: ValueKind::Raw(RawKind::Number),
            }],
            outputs: vec![ValuePort {
                name: "object".to_string(),
                kind: ValueKind::Raw(RawKind::Object),
            }],
            ..Default::default()
        },
        Prefab {
            name: "slider_x".to_string(),
            parts: array![[[0x212]]],
            inputs: vec![ValuePort {
                // TODO: Offset
                name: "speed".to_string(),
                kind: ValueKind::Raw(RawKind::Number),
            }],
            outputs: vec![ValuePort {
                name: "object".to_string(),
                kind: ValueKind::Raw(RawKind::Object),
            }],
            ..Default::default()
        },
        Prefab {
            name: "slider_y".to_string(),
            parts: array![[[0x213]]],
            inputs: vec![ValuePort {
                // TODO: Offset
                name: "speed".to_string(),
                kind: ValueKind::Raw(RawKind::Number),
            }],
            outputs: vec![ValuePort {
                name: "object".to_string(),
                kind: ValueKind::Raw(RawKind::Object),
            }],
            ..Default::default()
        },
        Prefab {
            name: "slider_z".to_string(),
            parts: array![[[0x214]]],
            inputs: vec![ValuePort {
                // TODO: Offset
                name: "speed".to_string(),
                kind: ValueKind::Raw(RawKind::Number),
            }],
            outputs: vec![ValuePort {
                name: "object".to_string(),
                kind: ValueKind::Raw(RawKind::Object),
            }],
            ..Default::default()
        },
        Prefab {
            name: "button_plate".to_string(),
            parts: array![[[0x215]]],
            outputs: vec![
                ValuePort {
                    // TODO: Offset
                    name: "on".to_string(),
                    kind: ValueKind::Raw(RawKind::Object),
                },
                ValuePort {
                    // TODO: Offset
                    name: "object".to_string(),
                    kind: ValueKind::Raw(RawKind::Object),
                },
            ],
            ..Default::default()
        },
        Prefab {
            name: "wheel".to_string(),
            parts: array![[[0x216]]],
            outputs: vec![ValuePort {
                name: "object".to_string(),
                kind: ValueKind::Raw(RawKind::Object),
            }],
            ..Default::default()
        },
        Prefab {
            name: "dash_cat".to_string(),
            parts: array![[[0x217]]],
            outputs: vec![ValuePort {
                name: "object".to_string(),
                kind: ValueKind::Raw(RawKind::Object),
            }],
            ..Default::default()
        },
        Prefab {
            name: "marker".to_string(),
            parts: array![[[0x218]]],
            outputs: vec![ValuePort {
                // TODO: Offset
                name: "object".to_string(),
                kind: ValueKind::Raw(RawKind::Object),
            }],
            ..Default::default()
        },
        Prefab {
            name: "tap_dino".to_string(),
            parts: array![[[0x219]]],
            outputs: vec![ValuePort {
                name: "object".to_string(),
                kind: ValueKind::Raw(RawKind::Object),
            }],
            ..Default::default()
        },
        Prefab {
            name: "red_dino".to_string(),
            parts: array![[[0x21A]]],
            outputs: vec![ValuePort {
                name: "object".to_string(),
                kind: ValueKind::Raw(RawKind::Object),
            }],
            ..Default::default()
        },
        Prefab {
            name: "butterfly".to_string(),
            parts: array![[[0x21B]]],
            outputs: vec![ValuePort {
                // TODO: Offset
                name: "object".to_string(),
                kind: ValueKind::Raw(RawKind::Object),
            }],
            ..Default::default()
        },
        Prefab {
            name: "camera_orbit".to_string(),
            parts: array![[[0x21C, 0x21E]], [[0x21F, 0x21D]]],
            ..Default::default()
        },
        Prefab {
            name: "slider_script".to_string(),
            parts: array![[[0x220]]],
            inputs: vec![
                // TODO: Offsets
                ValuePort {
                    name: "slider".to_string(),
                    kind: ValueKind::Raw(RawKind::Object),
                },
                ValuePort {
                    name: "speed".to_string(),
                    kind: ValueKind::Raw(RawKind::Number),
                },
                ValuePort {
                    name: "rail".to_string(),
                    kind: ValueKind::Raw(RawKind::Object),
                },
                ValuePort {
                    name: "axis".to_string(),
                    kind: ValueKind::Raw(RawKind::Vector),
                },
            ],
            ..Default::default()
        },
        Prefab {
            name: "script_block".to_string(),
            parts: array![[[0x221]]],
            ..Default::default()
        },
        Prefab {
            name: "sphere".to_string(),
            parts: array![[[0x222]]],
            outputs: vec![ValuePort {
                name: "object".to_string(),
                kind: ValueKind::Raw(RawKind::Object),
            }],
            ..Default::default()
        },
        Prefab {
            name: "slate".to_string(),
            parts: array![[[0x223]]],
            outputs: vec![ValuePort {
                name: "object".to_string(),
                kind: ValueKind::Raw(RawKind::Object),
            }],
            ..Default::default()
        },
        Prefab {
            name: "dotted_slate".to_string(),
            parts: array![[[0x224]]],
            outputs: vec![ValuePort {
                name: "object".to_string(),
                kind: ValueKind::Raw(RawKind::Object),
            }],
            ..Default::default()
        },
        Prefab {
            name: "slate_north_east".to_string(),
            parts: array![[[0x225]]],
            outputs: vec![ValuePort {
                name: "object".to_string(),
                kind: ValueKind::Raw(RawKind::Object),
            }],
            ..Default::default()
        },
        Prefab {
            name: "slate_north_west".to_string(),
            parts: array![[[0x226]]],
            outputs: vec![ValuePort {
                name: "object".to_string(),
                kind: ValueKind::Raw(RawKind::Object),
            }],
            ..Default::default()
        },
        Prefab {
            name: "slate_south_east".to_string(),
            parts: array![[[0x227]]],
            outputs: vec![ValuePort {
                name: "object".to_string(),
                kind: ValueKind::Raw(RawKind::Object),
            }],
            ..Default::default()
        },
        Prefab {
            name: "slate_south_west".to_string(),
            parts: array![[[0x228]]],
            outputs: vec![ValuePort {
                name: "object".to_string(),
                kind: ValueKind::Raw(RawKind::Object),
            }],
            ..Default::default()
        },
        Prefab {
            name: "slate_top".to_string(),
            parts: array![[[0x229]]],
            outputs: vec![ValuePort {
                name: "object".to_string(),
                kind: ValueKind::Raw(RawKind::Object),
            }],
            ..Default::default()
        },
        Prefab {
            name: "slate_bottom".to_string(),
            parts: array![[[0x22A]]],
            outputs: vec![ValuePort {
                name: "object".to_string(),
                kind: ValueKind::Raw(RawKind::Object),
            }],
            ..Default::default()
        },
        Prefab {
            name: "particle".to_string(),
            parts: array![[[0x22B]]],
            outputs: vec![ValuePort {
                // TODO: Offset
                name: "object".to_string(),
                kind: ValueKind::Raw(RawKind::Object),
            }],
            ..Default::default()
        },
        Prefab {
            name: "increase_number".to_string(),
            parts: array![[[0x22C, 0x22D]]],
            inputs: vec![ValuePort {
                name: "variable".to_string(),
                kind: ValueKind::Reference(RawKind::Number),
            }],
            callable: true,
            ..Default::default()
        },
        Prefab {
            name: "decrease_number".to_string(),
            parts: array![[[0x22E, 0x22F]]],
            inputs: vec![ValuePort {
                name: "variable".to_string(),
                kind: ValueKind::Reference(RawKind::Number),
            }],
            callable: true,
            ..Default::default()
        },
        Prefab {
            name: "loop".to_string(),
            parts: array![[[0x230, 0x231]], [[0x232, 0x233]]],
            inputs: vec![
                ValuePort {
                    name: "start".to_string(),
                    kind: ValueKind::Raw(RawKind::Number),
                },
                ValuePort {
                    name: "stop".to_string(),
                    kind: ValueKind::Raw(RawKind::Number),
                },
            ],
            outputs: vec![ValuePort {
                name: "counter".to_string(),
                kind: ValueKind::Raw(RawKind::Number),
            }],
            callable: true,
            callbacks: vec![ExecutePort {
                name: "do".to_string(),
            }],
            ..Default::default()
        },
        Prefab {
            name: "current_frame".to_string(),
            parts: array![[[0x234, 0x235]]],
            outputs: vec![ValuePort {
                name: "counter".to_string(),
                kind: ValueKind::Raw(RawKind::Number),
            }],
            ..Default::default()
        },
        Prefab {
            name: "late_update".to_string(),
            parts: array![[[0x236, 0x237]], [[0x238, 0x239]]],
            callbacks: vec![ExecutePort {
                name: "after_physics".to_string(),
            }],
            callable: true,
            ..Default::default()
        },
        Prefab {
            name: "dot_product".to_string(),
            parts: array![[[0x23A, 0x23B]], [[0x23C, 0x23D]]],
            inputs: vec![
                ValuePort {
                    name: "vec1".to_string(),
                    kind: ValueKind::Raw(RawKind::Vector),
                },
                ValuePort {
                    name: "vec2".to_string(),
                    kind: ValueKind::Raw(RawKind::Vector),
                },
            ],
            outputs: vec![ValuePort {
                name: "dot_product".to_string(),
                kind: ValueKind::Raw(RawKind::Number),
            }],
            ..Default::default()
        },
        Prefab {
            name: "cross_product".to_string(),
            parts: array![[[0x23E, 0x23F]], [[0x240, 0x241]]],
            inputs: vec![
                ValuePort {
                    name: "vec1".to_string(),
                    kind: ValueKind::Raw(RawKind::Vector),
                },
                ValuePort {
                    name: "vec2".to_string(),
                    kind: ValueKind::Raw(RawKind::Vector),
                },
            ],
            outputs: vec![ValuePort {
                name: "cross_product".to_string(),
                kind: ValueKind::Raw(RawKind::Vector),
            }],
            ..Default::default()
        },
        Prefab {
            name: "normalize".to_string(),
            parts: array![[[0x242, 0x243]]],
            inputs: vec![ValuePort {
                name: "vector".to_string(),
                kind: ValueKind::Raw(RawKind::Vector),
            }],
            outputs: vec![ValuePort {
                name: "normalized".to_string(),
                kind: ValueKind::Raw(RawKind::Vector),
            }],
            ..Default::default()
        },
        Prefab {
            name: "logarithm".to_string(),
            parts: array![[[0x244, 0x245]], [[0x246, 0x247]]],
            inputs: vec![
                ValuePort {
                    name: "number".to_string(),
                    kind: ValueKind::Raw(RawKind::Number),
                },
                ValuePort {
                    name: "base".to_string(),
                    kind: ValueKind::Raw(RawKind::Number),
                },
            ],
            outputs: vec![ValuePort {
                name: "logarithm".to_string(),
                kind: ValueKind::Raw(RawKind::Number),
            }],
            ..Default::default()
        },
        Prefab {
            name: "menu_item".to_string(),
            parts: array![[[0x248, 0x249]], [[0x24A, 0x24B]]],
            inputs: vec![
                ValuePort {
                    name: "variable".to_string(),
                    kind: ValueKind::Reference(RawKind::Number),
                },
                ValuePort {
                    name: "picture".to_string(),
                    kind: ValueKind::Raw(RawKind::Object),
                },
            ],
            options: vec![
                Opt {
                    name: "name".to_string(),
                    kind: OptKind::Name,
                },
                Opt {
                    name: "amount".to_string(),
                    kind: OptKind::Int8,
                },
                Opt {
                    name: "price".to_string(),
                    kind: OptKind::Int8,
                },
            ],
            callable: true,
            ..Default::default()
        },
        Prefab {
            name: "button".to_string(),
            parts: array![[[0x24C, 0x24D]], [[0x24E, 0x24F]]],
            callable: true,
            callbacks: vec![ExecutePort {
                name: "button".to_string(),
            }],
            ..Default::default()
        },
        Prefab {
            name: "joystick".to_string(),
            parts: array![[[0x250, 0x251]], [[0x252, 0x253]]],
            outputs: vec![ValuePort {
                name: "joy_dir".to_string(),
                kind: ValueKind::Raw(RawKind::Vector),
            }],
            callable: true,
            ..Default::default()
        },
    ];

    let prefabs = prefabs
        .into_iter()
        .map(|x| (x.name.to_string(), x))
        .collect();

    prefabs
}
