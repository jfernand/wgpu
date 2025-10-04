use wgpu::VertexBufferLayout;
use crate::primitives::triangle::Triangle;
use crate::vertices::Vertex;

pub mod triangle;

pub trait Primitive<'a, T: 'a> {
    fn desc() -> wgpu::VertexBufferLayout<'static>;
    fn vertices(&'a self) -> &'a [T];

    fn len(&'a self) -> usize {
        self.vertices().len()
    }

    fn indices(&'a self) -> &'a [i32];
}

impl<'a> Primitive<'a, Vertex> for Triangle<'a> {
    fn desc() -> VertexBufferLayout<'static> {
        Vertex::desc()
    }

    fn vertices(&'a self) -> &'a [Vertex] {
        self.vertices
    }

    fn indices(&'a self) -> &'a [i32] {
        self.indices
    }
}
