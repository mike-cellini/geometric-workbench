use super::*;

#[derive(Default)]
pub struct Vertex {
    pub id: VertexHandle,
    pub half_edge: Option<HalfEdgeHandle>,
    pub coordinates: Vector,
    pub next: VertexHandle,
    pub prev: VertexHandle,
}

impl Vertex {
    pub fn new(id: VertexHandle, x: u16, y: u16, z: u16) -> Self {
        Self {
            id,
            coordinates: [x, y, z, 1],
            ..Default::default()
        }
    }

    pub fn list(&self) {
        print!(
            " {}: ({} {} {})",
            self.id, self.coordinates[0], self.coordinates[1], self.coordinates[2]
        );
    }

    pub fn list_neighbors(&self, model: &Model) {
        match self.half_edge {
            Some(he) => {
                let he = &model.half_edges[he];
                loop {
                    if let Some(next) = he.next {
                        let next = &model.half_edges[next];
                        print!("{} ", next.id);
                    }
                    match model.half_edges[he.mate(model)].next {
                        Some(mate_next) => {
                            if mate_next == self.half_edge.unwrap() {
                                break;
                            }
                        }
                        None => {
                            println!("Invalid model");
                            break;
                        }
                    }
                }
            }
            None => println!("No adjacent vertices"),
        }
    }
}
