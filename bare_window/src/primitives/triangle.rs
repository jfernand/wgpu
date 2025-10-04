use crate::vertices::Vertex;

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
