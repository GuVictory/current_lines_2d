

#[derive(Debug)]
#[derive(Copy, Clone)]
#[derive(PartialEq)]
pub struct Coords {
    pub(crate) x: f64,
    pub(crate) y: f64,
}

impl Coords {
    pub fn new(x_p: f64, y_p: f64) -> Coords {
        Coords {
            x: x_p as f64,
            y: y_p as f64
        }
    }

    #[cfg_attr(tarpaulin, skip)]
    pub fn the_same(&self, other: &Coords) -> bool {
        return self.x == other.x && self.y == other.y
    }

    pub fn set_coords(&mut self, x_p: f64, y_p: f64) {
        self.x = x_p;
        self.y = y_p;
    }

    pub fn get_normalize(&self) -> Coords {
        let length= (self.x*self.x + self.y*self.y).sqrt();

        return Coords::new( self.x / length,  self.y / length)
    }


}

#[cfg(test)]
#[path = "./tests/coords_test.rs"]
mod coords_test;
