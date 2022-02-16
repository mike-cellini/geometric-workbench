mod solid;
mod face;
mod r#loop;
mod half_edge;
mod vertex;

use crate::solid::*;
use crate::face::*;
use crate::r#loop::*;
use crate::half_edge::*;
use crate::vertex::*;

pub type Vector = [u16; 4];
pub type Matrix = [[u16; 4]; 4];
pub type SolidHandle = usize;
pub type FaceHandle = usize;
pub type LoopHandle = usize;
pub type EdgeHandle = usize;
pub type HalfEdgeHandle = usize;
pub type VertexHandle = usize;

#[derive(Default)]
pub struct Model {
    pub solids: Vec<Solid>,
    pub faces: Vec<Face>,
    pub loops: Vec<Loop>,
    pub half_edges: Vec<HalfEdge>,
    pub edges: Vec<Edge>,
    pub vertices: Vec<Vertex>,
}

impl Model {
    pub fn add_solid(&mut self) -> &mut Solid {
        let new_handle = self.solids.len();
        let new_solid = Solid::new(new_handle);
        self.solids.push(new_solid);
        &mut self.solids[new_handle]
    }

    pub fn add_face(&mut self, solid: SolidHandle) -> &mut Face {
        let new_handle = self.faces.len();
        let new_face = Face::new(new_handle, solid);
        self.faces.push(new_face);
        &mut self.faces[new_handle]
    }

    pub fn add_loop(&mut self, face: FaceHandle) -> &mut Loop {
        let new_handle = self.loops.len();
        let new_loop = Loop::new(new_handle, face);
        self.loops.push(new_loop);
        &mut self.loops[new_handle]
    }

    pub fn add_vertex(&mut self, x: u16, y: u16, z: u16) -> &mut Vertex {
        let new_handle = self.vertices.len();
        let new_vertex = Vertex::new(new_handle, x, y, z);
        self.vertices.push(new_vertex);
        &mut self.vertices[new_handle]
    }
    
    pub fn list_solid(&self, handle: SolidHandle) {
        let solid = &self.solids[handle];
        solid.list(self);
    }
}

