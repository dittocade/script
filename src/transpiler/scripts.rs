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
    ]
}
