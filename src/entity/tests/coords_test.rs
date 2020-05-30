use super::*;

#[test]
fn coords_create() {
    let coords_1: Coords = Coords {
        x: 1.2,
        y: 0 as f64
    };

    let coords_2: Coords = Coords::new(1.2, 0 as f64);

    assert!(coords_1.the_same(&coords_2));
}

#[test]
fn coords_set() {
    let mut coords_1: Coords = Coords::new(1 as f64, 0 as f64);
    let coords_2: Coords = Coords::new(1.2, 0 as f64);

    coords_1.set_coords(1.2, 0 as f64);

    assert!(coords_1.the_same(&coords_2));
}

#[test]
fn coords_normalize() {
    let mut coords_1: Coords = Coords::new(3 as f64, 4 as f64);
    let coords_2: Coords = coords_1.get_normalize();

    let coords_3: Coords = Coords::new(3.0/5.0, 4.0/5.0);

    assert!(coords_2.the_same(&coords_3));
}