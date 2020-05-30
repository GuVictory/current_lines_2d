mod current_line_generator;
mod entity;
mod interpolation;
mod msh_worker;
mod vector_field_generator;

#[cfg_attr(tarpaulin, skip)]
fn main() {
    vector_field_generator::start_generating_vector_field();
}
