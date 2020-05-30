use super::*;

#[test]
fn vector_field_create() {
    let vector_field_1: VectorField = VectorField {
        coords: Coords::new(1.1, 1.1),
        coords_normalize: Coords::new(1.1, 1.1).get_normalize()
    };

    let vector_field_2: VectorField = VectorField::new(&Coords::new(1.1, 1.1));

    assert!(vector_field_1.the_same(&vector_field_2));
}

#[test]
fn coords_set() {
    let mut vector_field_1: VectorField = VectorField::new(&Coords::new(2.1, 2.1));
    let vector_field_2: VectorField = VectorField::new(&Coords::new(1.1, 1.1));

    vector_field_1.set_coords(&Coords::new(1.1, 1.1));

    assert!(vector_field_1.the_same(&vector_field_2));
}