use crate::vertices::Vertex;

pub(crate) const VERTICES: &[Vertex] = &[
    Vertex {
        position: [0.0, 0.5, 0.0],
        color: [1.0, 0.0, 0.0],
    },
    Vertex {
        position: [-0.5, -0.5, 0.0],
        color: [0.0, 1.0, 0.0],
    },
    Vertex {
        position: [0.5, -0.5, 0.0],
        color: [0.0, 0.0, 1.0],
    },
];

pub struct Triangle<'a> {
    pub vertices: &'a [Vertex],
    pub indices: &'a [i32],
}

impl<'a> Triangle<'a> {
    pub(crate) fn new(vertices: impl Into<&'a [Vertex]>) -> Self {
        Triangle {
            vertices: vertices.into(),
            indices: [0, 1, 2].as_slice(),
        }
    }
}
