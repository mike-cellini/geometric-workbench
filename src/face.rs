use super::*;

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
    pub fn new(id: FaceHandle, solid: SolidHandle) -> Self {
        Self {
            id,
            solid,
            ..Default::default()
        }
    }

    pub fn list(&self, model: &Model) {
        println!("{}", self.id);
        for &l in self.loops.as_slice() {
            let l = &model.loops[l];
            l.list(model);
        }
    }
}

