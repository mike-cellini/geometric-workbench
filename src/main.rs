use geometric_workbench::*;

fn main() {
    let mut model = Model {
        ..Default::default()
    };

    let mut solid = Solid {
        id: 0,
        next: 0,
        prev: 0,
        ..Default::default()
    };

    let mut face = Face {
        id: 0,
        next: 0,
        prev: 0,
        ..Default::default()
    };

    let mut l = Loop {
        id: 0,
        face: 0,
        next: 0,
        prev: 0,
        ..Default::default()
    };

    let v1 = Vertex {
        id: 0,
        coordinates: [0, 0, 0, 0],
        next: 1,
        prev: 3,
        ..Default::default()
    };

    let he1 = HalfEdge {
        id: 0,
        vertex: 0,
        next: 1,
        prev: 3,
        parent_loop: 0,
        ..Default::default()
    };

    let v2 = Vertex {
        id: 1,
        coordinates: [1, 0, 0, 0],
        next: 2,
        prev: 0,
        ..Default::default()
    };

    let he2 = HalfEdge {
        id: 2,
        vertex: 1,
        next: 2,
        prev: 0,
        ..Default::default()
    };

    let v3 = Vertex {
        id: 2,
        coordinates: [1, 1, 0, 0],
        next: 3,
        prev: 1,
        ..Default::default()
    };

    let he3 = HalfEdge {
        id: 2,
        vertex: 2,
        next: 3,
        prev: 1,
        ..Default::default()
    };

    let v4 = Vertex {
        id: 3,
        coordinates: [0, 1, 0, 0],
        next: 0,
        prev: 2,
        ..Default::default()
    };

    let he4 = HalfEdge {
        id: 3,
        vertex: 3,
        next: 0,
        prev: 2,
        ..Default::default()
    };

    solid.faces.push(face.id);
    face.loops.push(l.id);
    l.halfedge = he1.id;

    model.solids.push(solid);
    model.faces.push(face);
    model.loops.push(l);
    model.half_edges.push(he1);
    model.half_edges.push(he2);
    model.half_edges.push(he3);
    model.half_edges.push(he4);
    model.vertices.push(v1);
    model.vertices.push(v2);
    model.vertices.push(v3);
    model.vertices.push(v4);

    model.list_solid(0);
}
