use super::*;

#[test]
fn node_create() {
    let vect_cord = Coords::new(1.1, 2.3);
    let node_1: Node = Node {
        id: 0,
        coords: Coords::new(1.1, 2.3),
        vector_field: VectorField::new(&vect_cord),
        color: Colors::GREEN
    };

    let node_2: Node = Node::new(0, 1.1, 2.3);

    assert!(node_1.the_same(&node_2));
}

#[test]
fn coords_set() {
    let mut node_1: Node = Node::new(0, 2.1, 1.2);
    let node_2: Node = Node::new(0, 1.1, 2.3);

    node_1.set_coords(&node_2.coords);

    assert!(node_1.the_same(&node_2));
}

#[test]
fn color_set() {
    let mut node_1: Node = Node::new(0, 1.1, 1.2);
    let mut node_2: Node = Node::new(0, 1.1, 1.2);

    node_1.set_color(Colors::ORANGE);
    node_2.set_color(Colors::ORANGE);

    assert!(node_1.the_same(&node_2));
}