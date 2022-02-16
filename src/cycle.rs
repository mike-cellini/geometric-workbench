use super::*;

#[derive(Default)]
pub struct Cycle {
    pub id: CycleHandle,
    pub halfedge: Option<HalfEdgeHandle>,
    pub face: FaceHandle,
    pub next: CycleHandle,
    pub prev: CycleHandle,
}

impl Cycle {
    pub fn new(id: CycleHandle, face: FaceHandle) -> Self {
        Self {
            id,
            face,
            ..Default::default()
        }
    }

    pub fn list(&self, model: &Model) {
        print!("cycle:");

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

