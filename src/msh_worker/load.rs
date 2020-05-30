use super::CurrentFilePlace;
use crate::entity::cell::Cell;
use crate::entity::coords::Coords;
use crate::entity::node::Node;
use std::fs::File;
use std::io::Error;
use std::io::{BufRead, BufReader};

// Функция возвращает:
// вектор ячеек
// размер по x
// размер по y
// минимальную координату по x
// минимальную координату по y
pub fn load_mesh(file_name: &String) -> Result<(Vec<Cell>, f64, f64, f64, f64), Error> {
    let file = File::open(file_name)?;

    let reader = BufReader::new(file);

    let mut file_counter = 0;
    let mut node_data_space = 7;
    let mut file_place = CurrentFilePlace::MeshFormat;
    let mut nodes: Vec<Node> = vec![];
    let mut cells: Vec<Cell> = vec![];

    for (ind, line) in reader.lines().enumerate() {
        let line = line?;

        if file_counter == 0 {
            if line.starts_with("$MeshFormat") {
                file_counter = 2;
                continue;
            }
            if line.starts_with("$Nodes") {
                file_place = CurrentFilePlace::Nodes;
                continue;
            }
            if line.starts_with("$Elements") {
                file_place = CurrentFilePlace::Elements;
                continue;
            }
            if line.starts_with("$NodeData") {
                file_place = CurrentFilePlace::NodeData;
                continue;
            }

            if file_place == CurrentFilePlace::Nodes {
                file_counter = match line.trim().parse() {
                    Ok(val) => val,
                    Err(_) => {
                        continue;
                    }
                };
                println!("[INFO]: Number of Nodes: {}", file_counter);
                continue;
            }
            if file_place == CurrentFilePlace::Elements {
                file_counter = match line.trim().parse() {
                    Ok(val) => val,
                    Err(_) => {
                        continue;
                    }
                };
                println!("[INFO]: Number of Elements: {}", file_counter);
                continue;
            }

            if file_place == CurrentFilePlace::NodeData {
                if node_data_space == 0 {
                    file_counter = match line.trim().parse() {
                        Ok(val) => val,
                        Err(_) => {
                            continue;
                        }
                    };
                    println!("[INFO]: Number of NodeData: {}", file_counter);
                } else {
                    node_data_space -= 1;
                }
                continue;
            }
        } else {
            if file_place == CurrentFilePlace::Nodes {
                let v: Vec<&str> = line.split(' ').collect();
                nodes.push(create_node(v));
            }
            if file_place == CurrentFilePlace::Elements {
                let data: Vec<&str> = line.split(' ').collect();
                let n1 = match data[6].parse::<usize>() {
                    Ok(val) => val,
                    Err(err) => panic!(err),
                };
                let n2 = match data[5].parse::<usize>() {
                    Ok(val) => val,
                    Err(err) => panic!(err),
                };
                let n3 = match data[4].parse::<usize>() {
                    Ok(val) => val,
                    Err(err) => panic!(err),
                };
                let n4 = match data[3].parse::<usize>() {
                    Ok(val) => val,
                    Err(err) => panic!(err),
                };
                cells.push(Cell::new(
                    &nodes[n1 - 1].clone(),
                    &nodes[n2 - 1].clone(),
                    &nodes[n3 - 1].clone(),
                    &nodes[n4 - 1].clone(),
                ));
            }
            if file_place == CurrentFilePlace::NodeData {
                let data: Vec<&str> = line.split(' ').collect();

                let id = match data[0].parse::<usize>() {
                    Ok(val) => val,
                    Err(err) => panic!(err),
                };
                let x = match data[1].parse::<f64>() {
                    Ok(val) => val,
                    Err(err) => panic!(err),
                };
                let y = match data[2].parse::<f64>() {
                    Ok(val) => val,
                    Err(err) => panic!(err),
                };

                nodes[id - 1].vector_field.set_coords(&Coords::new(x, y))
            }
            file_counter -= 1;
            // println!("{}. {}", ind + 1, line);
        }
    }

    let mut min_x = 0.0;
    let mut max_x = 0.0;
    let mut min_y = 0.0;
    let mut max_y = 0.0;

    if nodes.len() != 0 {
        min_x = nodes[0].coords.x;
        max_x = nodes[0].coords.x;
        min_y = nodes[0].coords.y;
        max_y = nodes[0].coords.y;
    }

    for val in &nodes {
        if min_x > val.coords.x {
            min_x = val.coords.x;
        }
        if min_y > val.coords.y {
            min_y = val.coords.y;
        }
        if max_x < val.coords.x {
            max_x = val.coords.x;
        }
        if max_y < val.coords.y {
            max_y = val.coords.y;
        }
    }
    println!(
        "[INFO]: Mesh size: x = {}, y = {} \n",
        min_x + max_x,
        min_y + max_y
    );
    Ok((cells, min_x + max_x, min_y + max_y, min_x, min_y))
}

fn create_node(data: Vec<&str>) -> Node {
    let id = match data[0].parse::<i32>() {
        Ok(val) => val,
        Err(err) => {
            println!("{}", err);
            panic!(err)
        }
    };
    let x = match data[1].parse::<f64>() {
        Ok(val) => val,
        Err(err) => panic!(err),
    };
    let y = match data[2].parse::<f64>() {
        Ok(val) => val,
        Err(err) => panic!(err),
    };

    Node::new(id, x, y)
}
