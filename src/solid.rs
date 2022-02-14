use super::*;

#[derive(Default)]
pub struct Solid {
    pub id: SolidHandle,
    pub faces: Vec<FaceHandle>,
    pub edges: Vec<EdgeHandle>,
    pub vertices: Vec<VertexHandle>,
    pub next: Option<SolidHandle>,
    pub prev: Option<SolidHandle>,
}

impl Solid {
    pub fn new(id: SolidHandle) -> Self {
        Self {
            id,
            ..Default::default()
        }
    }

    pub fn list(&self, model: &Model) {
        for &f in self.faces.as_slice() {
            let f = &model.faces[f];
            f.list(model);
        }
    }
}
