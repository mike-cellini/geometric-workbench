use super::*;

#[derive(Default)]
pub struct Loop {
    pub id: LoopHandle,
    pub halfedge: Option<HalfEdgeHandle>,
    pub face: FaceHandle,
    pub next: LoopHandle,
    pub prev: LoopHandle,
}

impl Loop {
    pub fn new(id: LoopHandle, face: FaceHandle) -> Self {
        Self {
            id,
            face,
            ..Default::default()
        }
    }

    pub fn list(&self, model: &Model) {
        print!("loop:");

        if let Some(he) = self.halfedge {
            let mut he = &model.half_edges[he];
            loop {
                he.list(model);
                if let Some(next) = he.next {
                    he = &model.half_edges[next];
                    if he.id == self.halfedge.unwrap() {
                        break;
                    }
                }
            }
        }
    }        
}

