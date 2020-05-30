use crate::entity::cell::Cell;
use crate::entity::coords::Coords;
use crate::entity::cross_points::CrossPoints;
use crate::entity::line::Line;
use crate::entity::node::Node;

pub fn interpolation(cell: &Cell, base_node: &mut Node) -> bool {
    if let Some(cross_points) = first_interpolation(cell, base_node) {
        second_interpolation(cross_points, base_node);
        return true;
    }
    return false;
}

pub fn first_interpolation(cell: &Cell, baseNode: &mut Node) -> Option<CrossPoints> {
    return if let Some(mut cross_points) = cell.find_cross_point_x(baseNode) {
        let t = Line::new(&cell.nodes[3], &cross_points.small).len()
            / Line::new(&cell.nodes[0], &cell.nodes[3]).len();

        let tau = Line::new(&cell.nodes[2], &cross_points.big).len()
            / Line::new(&cell.nodes[1], &cell.nodes[2]).len();

        let u_a = t * cell.nodes[0].vector_field.coords.x
            + (1.0 - t) * cell.nodes[3].vector_field.coords.x;
        let v_a = t * cell.nodes[0].vector_field.coords.y
            + (1.0 - t) * cell.nodes[3].vector_field.coords.y;

        let u_b = tau * cell.nodes[1].vector_field.coords.x
            + (1.0 - tau) * cell.nodes[2].vector_field.coords.x;
        let v_b = tau * cell.nodes[1].vector_field.coords.y
            + (1.0 - tau) * cell.nodes[2].vector_field.coords.y;

        cross_points
            .small
            .vector_field
            .set_coords(&Coords::new(u_a, v_a));
        cross_points
            .big
            .vector_field
            .set_coords(&Coords::new(u_b, v_b));

        Some(cross_points)
    } else {
        None
    };
}

pub fn second_interpolation(cross_points: CrossPoints, base_node: &mut Node) {
    let p = Line::new(base_node, &cross_points.big).len()
        / Line::new(&cross_points.small, &cross_points.big).len();

    let u = p * cross_points.small.vector_field.coords.x
        + (1.0 - p) * cross_points.big.vector_field.coords.x;
    let v = p * cross_points.small.vector_field.coords.y
        + (1.0 - p) * cross_points.big.vector_field.coords.y;

    base_node.vector_field.set_coords(&Coords::new(u, v));
}
