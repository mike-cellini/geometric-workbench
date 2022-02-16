use super::*;

pub enum Orientation {
    Right,
    Left,
}

#[derive(Default)]
pub struct Edge {
    pub id: EdgeHandle,
    pub right_halfedge: HalfEdgeHandle,
    pub left_halfedge: HalfEdgeHandle,
    pub next: EdgeHandle,
    pub prev: EdgeHandle,
}

#[derive(Default)]
pub struct HalfEdge {
    pub id: HalfEdgeHandle,
    pub edge: EdgeHandle,
    pub vertex: VertexHandle,
    pub parent_loop: LoopHandle,
    pub next: Option<HalfEdgeHandle>,
    pub prev: Option<HalfEdgeHandle>,
}

impl HalfEdge {
    pub fn new(
        id: HalfEdgeHandle,
        edge: EdgeHandle,
        vertex: VertexHandle,
        parent_loop: LoopHandle,
    ) -> Self {
        Self {
            id,
            edge,
            vertex,
            parent_loop,
            ..Default::default()
        }
    }

    pub fn list(&self, model: &Model) {
        let v = &model.vertices[self.vertex];
        v.list();
    }

    pub fn mate(&self, model: &Model) -> HalfEdgeHandle {
        let edge = &model.edges[self.edge];
        if edge.left_halfedge == self.id {
            edge.right_halfedge
        } else {
            edge.left_halfedge
        }
    }
}
