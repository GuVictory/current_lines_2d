use super::current_line_generator;
use super::entity;
use super::interpolation;
use super::msh_worker;
use std::fs;

#[cfg_attr(tarpaulin, skip)]
pub fn start_generating_vector_field() {
    let mut file_name = String::new();
    println!("Enter file to load mesh:");
    std::io::stdin().read_line(&mut file_name).unwrap();
    file_name.pop();

    println!("[INFO]: Start file upload");
    let (cells, size_x, size_y, min_x, min_y) = match msh_worker::load::load_mesh(&file_name) {
        Ok(val) => val,
        Err(err) => {
            println!("We have error while trying to read file: {}", file_name);
            println!("{}", err);
            panic!(err);
        }
    };

    println!("[DONE]: File upload\n");

    fs::remove_file("result.geo").unwrap();
    let mut geo_render = msh_worker::render::GeoRenderer::new(size_x);
    loop {
        println!("Enter the coordinates x and y of the base point:");
        let mut base_point = String::new();
        std::io::stdin().read_line(&mut base_point).unwrap();
        base_point.pop();
        let data: Vec<&str> = base_point.split(' ').collect();
        let x = match data[0].parse::<f64>() {
            Ok(val) => val,
            Err(err) => {
                println!("No number entered {}", err);
                continue;
            }
        };
        let y = match data[1].parse::<f64>() {
            Ok(val) => val,
            Err(err) => {
                println!("No number entered {}", err);
                continue;
            }
        };

        let mut agree = String::new();
        if x <= min_x || x >= min_x + size_x || y <= min_y || y >= min_y + size_y {
            println!(
                "You entered a point that does not fall on the grid \n{} < x < {}\n{} < y < {}",
                min_x,
                min_x + size_x,
                min_y,
                min_y + size_y
            );
            continue;
        }

        let mut current_line_generator = current_line_generator::CurrentLineGenerator::new(
            cells.clone(),
            size_x,
            size_y,
            min_x,
            min_y,
            &entity::node::Node::new(-1, x, y),
        );
        let nodes = current_line_generator.generate_current_line();
        geo_render.render_line(nodes);
        println!("You entered a point with coordinates: ({}, {})", x, y);

        println!("Enter another point? (Y/N)");
        std::io::stdin().read_line(&mut agree).unwrap();
        if agree.contains("N") {
            break;
        }
    }
}
