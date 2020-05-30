use crate::entity::node::Node;
use std::fs::OpenOptions;
use std::io::{Error, Write};

pub struct GeoRenderer {
    size: f64,
    main_counter: usize,
    node_counter: usize,
}

impl GeoRenderer {
    pub fn new(size: f64) -> GeoRenderer {
        let mut file = OpenOptions::new()
            .create(true)
            .write(true)
            .open("result.geo")
            .unwrap();
        file.write("".as_bytes());
        GeoRenderer {
            size,
            main_counter: 1,
            node_counter: 0,
        }
    }

    pub fn render_line(&mut self, nodes: Vec<Node>) -> Result<(), Error> {
        let mut file = OpenOptions::new()
            .write(true)
            .append(true)
            .create(true)
            .open("result.geo")?;

        let start_node_point = self.main_counter;

        for val in &nodes {
            file.write_all(
                format!(
                    "Point({}) = {{{},{},{},{}}};\n",
                    self.main_counter, val.coords.x, val.coords.y, 0.0, self.size
                )
                .as_bytes(),
            );
            self.node_counter += 1;
            self.main_counter += 1;
        }

        let mut nodes_number: Vec<String> = vec![];

        for i in start_node_point..(start_node_point + self.node_counter) {
            nodes_number.push(i.to_string());
        }
        file.write_all(
            format!(
                "Line({}) = {{{}}};\n",
                self.main_counter,
                nodes_number.join(", ")
            )
            .as_bytes(),
        );
        self.main_counter += 1;
        self.node_counter = 0;
        Ok(())
    }
}
