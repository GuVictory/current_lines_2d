use super::color::Colors;
use super::coords::Coords;
use super::vector_field::VectorField;

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Node {
    id: i32,
    pub(crate) vector_field: VectorField,
    pub(crate) coords: Coords,
    color: Colors,
}

impl Node {
    pub fn new(id_p: i32, x_p: f64, y_p: f64) -> Node {
        let vect_cord = Coords::new(x_p, y_p);
        Node {
            id: id_p,
            coords: Coords::new(x_p, y_p),
            vector_field: VectorField::new(&vect_cord),
            color: Colors::GREEN,
        }
    }

    #[cfg_attr(tarpaulin, skip)]
    pub fn the_same(&self, other: &Node) -> bool {
        return self.id == other.id
            && self.coords.the_same(&other.coords)
            && self.color == other.color;
    }

    pub fn set_coords(&mut self, coords: &Coords) {
        self.coords.set_coords(coords.x, coords.y);
    }

    pub fn set_color(&mut self, color: Colors) {
        self.color = color;
    }
}

#[cfg(test)]
#[path = "./tests/node_test.rs"]
mod node_test;
