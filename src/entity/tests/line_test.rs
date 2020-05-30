use super::*;

#[test]
fn line_create() {
    let line_1: Line = Line {
        first_point: Node::new(-1, 1.1, 1.1),
        second_point: Node::new(-1, 2.1, 1.1),
        coef_a: 1.1 - 1.1,
        coef_b: 2.1 - 1.1,
        coef_c: 1.1 * 1.1 - 2.1 * 1.1
    };

    let line_2: Line = Line::new( &Node::new(-1, 1.1, 1.1), &Node::new(-1, 2.1, 1.1));

    assert!(line_1.the_same(&line_2));
}


#[test]
fn find_position_left_test() {
    let line: Line = Line::new( &Node::new(-1, 1.0, 1.0), &Node::new(-1, 2.0, 1.1));

    let node: Node = Node::new(-1, 1.5, 3.0);

    assert!(line.find_position(&node) < 0.0);
}

#[test]
fn find_position_right_test() {
    let line: Line = Line::new( &Node::new(-1, 1.0, 1.0), &Node::new(-1, 2.0, 1.1));

    let node: Node = Node::new(-1, 1.5, -3.0);

    assert!(line.find_position(&node) > 0.0);
}

#[test]
fn find_position_lay_on_it_test() {
    let line: Line = Line::new( &Node::new(-1, 1.0, 1.0), &Node::new(-1, 2.0, 1.0));

    let node: Node = Node::new(-1, 1.5, 1.0);

    assert_eq!(line.find_position(&node), 0.0);
}

#[test]
fn find_len_test() {
    let line: Line = Line::new( &Node::new(-1, 1.0, 1.0), &Node::new(-1, 2.0, 1.0));
    assert_eq!(line.len(), 1.0);
}

#[test]
fn find_len_2_test() {
    let line: Line = Line::new( &Node::new(-1, 1.0, 1.0), &Node::new(-1, 1.0, 1.0));
    assert_eq!(line.len(), 0.0);
}

#[test]
fn are_parall_test() {
    let line_1: Line = Line::new( &Node::new(-1, 1.0, 1.0), &Node::new(-1, 2.0, 1.0));
    let line_2: Line = Line::new( &Node::new(-1, 1.0, 3.0), &Node::new(-1, 2.0, 3.0));
    assert!(line_1.are_parall(&line_2));
}

#[test]
fn are_parall_2_test() {
    let line_1: Line = Line::new( &Node::new(-1, 1.0, 1.0), &Node::new(-1, 2.0, 1.0));
    let line_2: Line = Line::new( &Node::new(-1, 1.0, 0.0), &Node::new(-1, 2.0, 3.0));
    assert!(!line_1.are_parall(&line_2));
}

#[test]
fn find_intersect_test() {
    let line_1: Line = Line::new( &Node::new(-1, 1.0, 1.0), &Node::new(-1, 2.0, 1.0));
    let line_2: Line = Line::new( &Node::new(-1, 1.0, 1.0), &Node::new(-1, 2.0, 3.0));
    let node: Node = Node::new(-1, 1.0, 1.0);

    if let Some(intersect) = line_1.lines_intersect(&line_2) {
        assert_eq!(intersect, node);
    }
}

#[test]
fn find_intersect_2_test() {
    let line_1: Line = Line::new( &Node::new(-1, 1.0, 1.0), &Node::new(-1, 2.0, 1.0));
    let line_2: Line = Line::new( &Node::new(-1, 1.0, 3.0), &Node::new(-1, 2.0, 3.0));
    assert_eq!(line_1.lines_intersect(&line_2), None);
}
