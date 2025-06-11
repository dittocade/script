use ndarray::{array, Array3};

use crate::game::{OptKind, RawKind, ValueKind};

#[derive(Debug, Default)]
#[allow(unused)]
pub struct Script {
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

pub fn get_scripts() -> Vec<Script> {
    vec![
        Script {
            name: "stone_block".to_string(),
            parts: array![[[0x01]]],
            outputs: vec![ ValuePort {
                name: "object".to_string(),
                kind: ValueKind::Raw(RawKind::Object),
            }],
            ..Default::default()
        },
        Script {
            name: "bricks".to_string(),
            parts: array![[[0x02]]],
            outputs: vec![ ValuePort {
                name: "object".to_string(),
                kind: ValueKind::Raw(RawKind::Object),
            }],
            ..Default::default()
        },
        Script {
            name: "grass".to_string(),
            parts: array![[[0x03]]],
            outputs: vec![ ValuePort {
                name: "object".to_string(),
                kind: ValueKind::Raw(RawKind::Object),
            }],
            ..Default::default()
        },
        Script {
            name: "spotted_grass".to_string(),
            parts: array![[[0x04]]],
            outputs: vec![ ValuePort {
                name: "object".to_string(),
                kind: ValueKind::Raw(RawKind::Object),
            }],
            ..Default::default()
        },
        Script {
            name: "dirt".to_string(),
            parts: array![[[0x05]]],
            outputs: vec![ ValuePort {
                name: "object".to_string(),
                kind: ValueKind::Raw(RawKind::Object),
            }],
            ..Default::default()
        },
        Script {
            name: "wood_x".to_string(),
            parts: array![[[0x06]]],
            outputs: vec![ ValuePort {
                name: "object".to_string(),
                kind: ValueKind::Raw(RawKind::Object),
            }],
            ..Default::default()
        },
        Script {
            name: "wood_z".to_string(),
            parts: array![[[0x07]]],
            outputs: vec![ ValuePort {
                name: "object".to_string(),
                kind: ValueKind::Raw(RawKind::Object),
            }],
            ..Default::default()
        },
        Script {
            name: "wood_y".to_string(),
            parts: array![[[0x08]]],
            outputs: vec![ ValuePort {
                name: "object".to_string(),
                kind: ValueKind::Raw(RawKind::Object),
            }],
            ..Default::default()
        },
        Script {
            name: "comment".to_string(),
            parts: array![[[0x0F]]],
            options: vec![Opt {
                name: "value".to_string(),
                kind: OptKind::Name,
            }],
            ..Default::default()
        },
        Script {
            name: "inspect_number".to_string(),
            parts: array![[[0x10, 0x11]], [[0x12, 0x13]]],
            inputs: vec![ValuePort {
                name: "number".to_string(),
                kind: ValueKind::Raw(RawKind::Number),
            }],
            callable: true,
            ..Default::default()
        },
        Script {
            name: "inspect_vector".to_string(),
            parts: array![[[0x14, 0x15]], [[0x16, 0x17]]],
            inputs: vec![ValuePort {
                name: "vector".to_string(),
                kind: ValueKind::Raw(RawKind::Vector),
            }],
            callable: true,
            ..Default::default()
        },
        Script {
            name: "inspect_rotation".to_string(),
            parts: array![[[0x18, 0x19]], [[0x1A, 0x1B]]],
            inputs: vec![ValuePort {
                name: "rotation".to_string(),
                kind: ValueKind::Raw(RawKind::Rotation),
            }],
            callable: true,
            ..Default::default()
        },
        Script {
            name: "inspect_truth".to_string(),
            parts: array![[[0x1C, 0x1D]], [[0x1E, 0x1F]]],
            inputs: vec![ValuePort {
                name: "truth".to_string(),
                kind: ValueKind::Raw(RawKind::Truth),
            }],
            callable: true,
            ..Default::default()
        },
        Script {
            name: "inspect_object".to_string(),
            parts: array![[[0x20, 0x21]], [[0x22, 0x23]]],
            inputs: vec![ValuePort {
                name: "truth".to_string(),
                kind: ValueKind::Raw(RawKind::Object),
            }],
            callable: true,
            ..Default::default()
        },
        Script {
            name: "inspect_object".to_string(),
            parts: array![[[0x20, 0x21]], [[0x22, 0x23]]],
            inputs: vec![ValuePort {
                name: "truth".to_string(),
                kind: ValueKind::Raw(RawKind::Object),
            }],
            callable: true,
            ..Default::default()
        },
        Script {
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
        Script {
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
        Script {
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
        Script {
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
        Script {
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
        Script {
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
        Script {
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
        Script {
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
        Script {
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
        Script {
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
        Script {
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
        Script {
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
        Script {
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
        Script {
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
        Script {
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
        Script {
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
        Script {
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
        Script {
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
        Script {
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
        Script {
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
        Script {
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
        Script {
            name: "subtract_numbers".to_string(),
            parts: array![[[0x64, 0x65]], [[0x65, 0x67]]],
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
        Script {
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
        Script {
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
        Script {
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
        Script {
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
        Script {
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
        Script {
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
        Script {
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
                name: "result".to_string(),
                kind: ValueKind::Raw(RawKind::Truth),
            }],
            ..Default::default()
        },
        Script {
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
                name: "result".to_string(),
                kind: ValueKind::Raw(RawKind::Truth),
            }],
            ..Default::default()
        },
        Script {
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
                name: "result".to_string(),
                kind: ValueKind::Raw(RawKind::Truth),
            }],
            ..Default::default()
        },
        Script {
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
                name: "result".to_string(),
                kind: ValueKind::Raw(RawKind::Truth),
            }],
            ..Default::default()
        },
        Script {
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
        Script {
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
        Script {
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
        Script {
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
        Script {
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
        Script {
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
        Script {
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
        Script {
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
        Script {
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
        Script {
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
        Script {
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
        Script {
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
        Script {
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
        Script {
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
        Script {
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
        Script {
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
        Script {
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
        Script {
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
        Script {
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
        Script {
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
        Script {
            name: "accelerometer".to_string(),
            parts: array![[[0xE0, 0xE1]], [[0xE2, 0xE3]]],
            outputs: vec![ValuePort {
                name: "direction".to_string(),
                kind: ValueKind::Raw(RawKind::Vector),
            }],
            ..Default::default()
        },
        Script {
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
        Script {
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
        Script {
            name: "play_sensor".to_string(),
            parts: array![[[0xEE, 0xEF]], [[0xF0, 0xF1]]],
            callable: true,
            callbacks: vec![ExecutePort {
                name: "on_play".to_string(),
            }],
            ..Default::default()
        },
        Script {
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
        Script {
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
        Script {
            name: "win".to_string(),
            parts: array![[[0xFC, 0xFD]], [[0xFE, 0xFF]]],
            options: vec![Opt {
                name: "delay".to_string(),
                kind: OptKind::Int8,
            }],
            callable: true,
            ..Default::default()
        },
        Script {
            name: "lose".to_string(),
            parts: array![[[0x100, 0x101]], [[0x102, 0x103]]],
            options: vec![Opt {
                name: "delay".to_string(),
                kind: OptKind::Int8,
            }],
            callable: true,
            ..Default::default()
        },
        Script {
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
        Script {
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
        Script {
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
        Script {
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
        Script {
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
        Script {
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
        Script {
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
        Script {
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
        Script {
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
        Script {
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
        Script {
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
        Script {
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
        Script {
            name: "destroy_object".to_string(),
            parts: array![[[0x140, 0x141]], [[0x142, 0x143]]],
            inputs: vec![ValuePort {
                name: "object".to_string(),
                kind: ValueKind::Raw(RawKind::Object),
            }],
            callable: true,
            ..Default::default()
        },
        Script {
            name: "set_gravity".to_string(),
            parts: array![[[0x144, 0x145]], [[0x146, 0x147]]],
            inputs: vec![ValuePort {
                name: "gravity".to_string(),
                kind: ValueKind::Raw(RawKind::Vector),
            }],
            callable: true,
            ..Default::default()
        },
        Script {
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
        Script {
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
        Script {
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
        Script {
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
        Script {
            name: "add_constraint".to_string(),
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
    ]
}
