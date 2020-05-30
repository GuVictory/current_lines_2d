use crate::entity::cell::Cell;
use crate::entity::coords::Coords;
use crate::entity::node::Node;
use crate::interpolation::interpolation;

pub struct CurrentLineGenerator {
    cells: Vec<Cell>,
    nodes: Vec<Node>,
    size: (f64, f64),
    min_x: f64,
    min_y: f64,

    current_node: Node,
    current_cell: Cell,
    dx: f64,
    dy: f64,
}

impl CurrentLineGenerator {
    pub fn new(
        cells: Vec<Cell>,
        size_x: f64,
        size_y: f64,
        min_x: f64,
        min_y: f64,
        base_node: &Node,
    ) -> CurrentLineGenerator {
        CurrentLineGenerator {
            nodes: vec![],
            size: (size_x, size_y),
            min_x,
            min_y,
            current_node: base_node.clone(),
            current_cell: cells[0].clone(),
            dx: size_x / 100.0,
            dy: size_y / 100.0,
            cells,
        }
    }

    pub fn generate_current_line(&mut self) -> Vec<Node> {
        self.find_current_cell();
        self.draw_loop();
        return self.nodes.clone();
    }

    fn find_current_cell(&mut self) {
        for cell in &self.cells {
            if cell.contains_node(&self.current_node) {
                self.current_cell = cell.clone();
                break;
            }
        }
    }

    fn draw_loop(&mut self) {
        let mut lines_counter = 0.0;
        while lines_counter < 1000.0 * self.size.0 {
            interpolation(&self.current_cell, &mut self.current_node);
            self.nodes.push(self.current_node.clone());

            self.current_node.set_coords(&Coords::new(
                self.current_node.coords.x
                    + self.dx * self.current_node.vector_field.coords_normalize.x,
                self.current_node.coords.y
                    + self.dy * self.current_node.vector_field.coords_normalize.y,
            ));

            // TODO: Добавить отрисовку паследней точки
            if self.current_node.coords.x < self.min_x
                || self.current_node.coords.y < self.min_y
                || self.current_node.coords.x > self.min_x + self.size.0
                || self.current_node.coords.y > self.min_y + self.size.1
            {
                break;
            }

            if !self.current_cell.contains_node(&self.current_node) {
                self.find_current_cell();
            }

            lines_counter += 1.0;
        }
    }
}
