use super::*;

#[derive(Default)]
pub struct Face {
    pub id: FaceHandle,
    pub solid: SolidHandle,
    pub outer_cycle: CycleHandle,
    pub cycles: Vec<CycleHandle>,
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
        for &l in self.cycles.as_slice() {
            let l = &model.cycles[l];
            l.list(model);
        }
    }
}

