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
    pub fn list_solid(&self, handle: SolidHandle) {
        let solid = &self.solids[handle];
        solid.list(self);
    }
}

#[derive(Default)]
pub struct Solid {
    pub id: SolidHandle,
    pub faces: Vec<FaceHandle>,
    pub edges: Vec<EdgeHandle>,
    pub vertices: Vec<VertexHandle>,
    pub next: SolidHandle,
    pub prev: SolidHandle,
}

impl Solid {
    pub fn list(&self, model: &Model) {
        for &f in self.faces.as_slice() {
            let f = &model.faces[f];
            f.list(model);
        }
    }
}

#[derive(Default)]
pub struct Face {
    pub id: FaceHandle,
    pub solid: SolidHandle,
    pub outer_loop: LoopHandle,
    pub loops: Vec<LoopHandle>,
    pub face_equation: Vector,
    pub next: FaceHandle,
    pub prev: FaceHandle,
}

impl Face {
    pub fn list(&self, model: &Model) {
        println!("{}", self.id);
        for &l in self.loops.as_slice() {
            let l = &model.loops[l];
            l.list(model);
        }
    }
}

#[derive(Default)]
pub struct Loop {
    pub id: LoopHandle,
    pub halfedge: HalfEdgeHandle,
    pub face: FaceHandle,
    pub next: LoopHandle,
    pub prev: LoopHandle,
}

impl Loop {
    pub fn list(&self, model: &Model) {
        print!("loop:");

        let mut he = &model.half_edges[self.halfedge];
        loop {
            he.list(model);
            he = &model.half_edges[he.next];
            if he.id == self.halfedge {
                break;
            }
        }
    }
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
    pub next: HalfEdgeHandle,
    pub prev: HalfEdgeHandle,
}

impl HalfEdge {
    pub fn list(&self, model: &Model) {
        let v = &model.vertices[self.vertex];
        v.list();
    }
}

#[derive(Default)]
pub struct Vertex {
    pub id: VertexHandle,
    pub edge: EdgeHandle,
    pub coordinates: Vector,
    pub next: VertexHandle,
    pub prev: VertexHandle,
}

impl Vertex {
    pub fn list(&self) {
        print!(
            " {}: ({} {} {})",
            self.id, self.coordinates[0], self.coordinates[1], self.coordinates[2]
        );
    }
}
