use super::*;

#[test]
fn cell_create() {
    let cell_1: Cell = Cell {
        nodes: vec![
            Node::new(-1, 0.0, 1.0),
            Node::new(-1, 1.0, 1.0),
            Node::new(-1, 1.0, 0.0),
            Node::new(-1, 0.0, 0.0),
        ],
    };

    let cell_2: Cell = Cell::new(
        &Node::new(-1, 0.0, 0.0),
        &Node::new(-1, 0.0, 1.0),
        &Node::new(-1, 1.0, 1.0),
        &Node::new(-1, 1.0, 0.0),
    );

    assert!(
        cell_1.the_same(&cell_2),
        "found cells: {:#?}, {:#?}",
        cell_1,
        cell_2
    );
}

#[test]
fn cell_find_biggest_x_test() {
    let cell: Cell = Cell::new(
        &Node::new(-1, 0.0, 0.0),
        &Node::new(-1, 0.0, 1.0),
        &Node::new(-1, 1.0, 1.0),
        &Node::new(-1, 1.0, 0.0),
    );

    assert_eq!(cell.find_biggest_x(), 1.0);
}

#[test]
fn cell_find_biggest_y_test() {
    let cell: Cell = Cell::new(
        &Node::new(-1, 0.0, 0.0),
        &Node::new(-1, 0.0, 2.0),
        &Node::new(-1, 1.0, 2.0),
        &Node::new(-1, 1.0, 0.0),
    );

    assert_eq!(cell.find_biggest_y(), 2.0);
}

#[test]
fn cell_find_smallest_x_test() {
    let cell: Cell = Cell::new(
        &Node::new(-1, 0.0, 0.0),
        &Node::new(-1, 0.0, 1.0),
        &Node::new(-1, 1.0, 1.0),
        &Node::new(-1, 1.0, 0.0),
    );

    assert_eq!(cell.find_smallest_x(), 0.0);
}

#[test]
fn cell_find_smallest_y_test() {
    let cell: Cell = Cell::new(
        &Node::new(-1, 0.0, -1.0),
        &Node::new(-1, 0.0, 2.0),
        &Node::new(-1, 1.0, 2.0),
        &Node::new(-1, 1.0, -1.0),
    );

    assert_eq!(cell.find_smallest_y(), -1.0);
}

#[test]
fn cell_is_in_x_range_test() {
    let cell: Cell = Cell::new(
        &Node::new(-1, 0.0, 0.0),
        &Node::new(-1, 0.0, 2.0),
        &Node::new(-1, 2.0, 2.0),
        &Node::new(-1, 2.0, 0.0),
    );
    assert!(cell.is_in_x_range(&Node::new(-1, 0.5, 0.5)));
}

#[test]
fn cell_is_in_x_range_2_test() {
    let cell: Cell = Cell::new(
        &Node::new(-1, 0.0, 0.0),
        &Node::new(-1, 0.0, 2.0),
        &Node::new(-1, 2.0, 2.0),
        &Node::new(-1, 2.0, 0.0),
    );

    assert!(
        cell.is_in_x_range(&Node::new(-1, 0.0, 0.5)),
        "cell: {:#?}",
        cell
    );
}

#[test]
fn cell_is_in_x_range_fail_test() {
    let cell: Cell = Cell::new(
        &Node::new(-1, 0.0, 0.0),
        &Node::new(-1, 0.0, 2.0),
        &Node::new(-1, 2.0, 2.0),
        &Node::new(-1, 2.0, 0.0),
    );

    assert!(!cell.is_in_x_range(&Node::new(-1, 4.0, 0.5)));
}

#[test]
fn cell_is_in_y_range_test() {
    let cell: Cell = Cell::new(
        &Node::new(-1, 0.0, 0.0),
        &Node::new(-1, 0.0, 2.0),
        &Node::new(-1, 2.0, 2.0),
        &Node::new(-1, 2.0, 0.0),
    );
    assert!(cell.is_in_y_range(&Node::new(-1, 0.5, 0.5)));
}

#[test]
fn cell_is_in_y_range_2_test() {
    let cell: Cell = Cell::new(
        &Node::new(-1, 0.0, 0.0),
        &Node::new(-1, 0.0, 2.0),
        &Node::new(-1, 2.0, 2.0),
        &Node::new(-1, 2.0, 0.0),
    );

    assert!(
        cell.is_in_x_range(&Node::new(-1, 0.0, 2.0)),
        "cell: {:#?}",
        cell
    );
}

#[test]
fn cell_is_in_y_range_fail_test() {
    let cell: Cell = Cell::new(
        &Node::new(-1, 0.0, 0.0),
        &Node::new(-1, 0.0, 2.0),
        &Node::new(-1, 2.0, 2.0),
        &Node::new(-1, 2.0, 0.0),
    );

    assert!(!cell.is_in_x_range(&Node::new(-1, 4.0, 5.5)));
}

#[test]
fn cell_contains_point_test() {
    let cell: Cell = Cell::new(
        &Node::new(-1, 0.0, 0.0),
        &Node::new(-1, 0.0, 1.0),
        &Node::new(-1, 1.0, 1.0),
        &Node::new(-1, 1.0, 0.0),
    );
    assert!(cell.contains_node(&Node::new(-1, 0.5, 0.5)));
}

#[test]
fn cell_contains_point_fail_1_test() {
    let cell: Cell = Cell::new(
        &Node::new(-1, 0.0, 0.0),
        &Node::new(-1, 0.0, 1.0),
        &Node::new(-1, 1.0, 1.0),
        &Node::new(-1, 1.0, 0.0),
    );
    assert!(!cell.contains_node(&Node::new(-1, 3.0, 0.5)));
}

#[test]
fn cell_contains_point_fail_2_test() {
    let cell: Cell = Cell::new(
        &Node::new(-1, 0.0, 0.0),
        &Node::new(-1, 0.0, 1.0),
        &Node::new(-1, 1.0, 1.0),
        &Node::new(-1, 1.0, 0.0),
    );
    assert!(!cell.contains_node(&Node::new(-1, 0.5, 5.5)));
}

#[test]
fn cell_contains_point_fail_3_test() {
    let cell: Cell = Cell::new(
        &Node::new(-1, 0.0, 0.0),
        &Node::new(-1, 0.0, 1.0),
        &Node::new(-1, 1.0, 1.0),
        &Node::new(-1, 1.0, 0.0),
    );
    assert!(!cell.contains_node(&Node::new(-1, 3.0, 8.5)));
}

#[test]
fn cell_found_cross_points_test() {
    let cell: Cell = Cell::new(
        &Node::new(-1, 0.0, 0.0),
        &Node::new(-1, 0.0, 2.0),
        &Node::new(-1, 2.0, 2.0),
        &Node::new(-1, 2.0, 0.0),
    );
    let node: Node = Node::new(-1, 0.5, 0.5);
    let cross = CrossPoints::new(&Node::new(-1, 0.0, 0.5), &Node::new(-1, 2.0, 0.5));

    if let Some(cross_points) = cell.find_cross_point_x(&node) {
        assert!(cross_points.the_same(&cross));
    }
}

#[test]
fn cell_found_cross_points_fail_test() {
    let cell: Cell = Cell::new(
        &Node::new(-1, 0.0, 0.0),
        &Node::new(-1, 0.0, 2.0),
        &Node::new(-1, 2.0, 2.0),
        &Node::new(-1, 2.0, 0.0),
    );
    let node: Node = Node::new(-1, 0.0, 0.5);
    assert_eq!(cell.find_cross_point_x(&node), None);
}
