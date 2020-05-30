use super::coords::Coords;

#[derive(Debug)]
#[derive(Copy, Clone)]
#[derive(PartialEq)]
pub struct VectorField {
    pub(crate) coords: Coords,
    pub(crate) coords_normalize: Coords,
}

impl VectorField {
    pub fn new(coords_p: &Coords) -> VectorField {
        // TODO: Добавить инициализацию координат вектора исходя из векторной функции
        VectorField {
            coords: Coords::new(coords_p.x, coords_p.y),
            coords_normalize: coords_p.get_normalize()
        }
    }

    #[cfg_attr(tarpaulin, skip)]
    pub fn the_same(&self, other: &VectorField) -> bool {
        return self.coords.the_same(&other.coords)
    }

    pub fn set_coords(&mut self, coords_p: &Coords) {
        self.coords = coords_p.clone();
        self.coords_normalize = coords_p.get_normalize();
    }

}

#[cfg(test)]
#[path = "./tests/vector_field_test.rs"]
mod vector_field_test;
