use super::cross_points::CrossPoints;
use super::line::Line;
use super::node::Node;

#[derive(Debug, Clone, PartialEq)]
pub struct Cell {
    pub(crate) nodes: Vec<Node>,
}

impl Cell {
    pub fn new(node_1: &Node, node_2: &Node, node_3: &Node, node_4: &Node) -> Cell {
        let mut cell = Cell {
            nodes: vec![
                node_1.clone(),
                node_2.clone(),
                node_3.clone(),
                node_4.clone(),
            ],
        };
        cell.sort_nodes();
        cell
    }

    #[cfg_attr(tarpaulin, skip)]
    pub fn sort_nodes(&mut self) {
        loop {
            if self.nodes[0].coords.x > self.nodes[1].coords.x {
                self.nodes.swap(0, 1);
            }

            if self.nodes[0].coords.x == self.nodes[1].coords.x
                && self.nodes[0].coords.y < self.nodes[1].coords.y
            {
                self.nodes.swap(0, 1);
            }

            if self.nodes[1].coords.y < self.nodes[2].coords.y {
                self.nodes.swap(1, 2);
            }
            if self.nodes[2].coords.x < self.nodes[3].coords.x {
                self.nodes.swap(2, 3);
            }
            if self.nodes[3].coords.y > self.nodes[0].coords.y {
                self.nodes.swap(3, 0);
            }

            if self.nodes[0].coords.x <= self.nodes[1].coords.x
                && self.nodes[1].coords.y >= self.nodes[2].coords.y
                && self.nodes[2].coords.x >= self.nodes[3].coords.x
                && self.nodes[3].coords.y <= self.nodes[0].coords.y
            {
                break;
            }
        }
    }

    pub fn the_same(&self, other: &Cell) -> bool {
        return self.nodes[0].the_same(&other.nodes[0])
            && self.nodes[1].the_same(&other.nodes[1])
            && self.nodes[2].the_same(&other.nodes[2])
            && self.nodes[3].the_same(&other.nodes[3]);
    }

    pub fn find_biggest_x(&self) -> f64 {
        let mut result_ind = 0;
        for (ind, el) in self.nodes.iter().enumerate() {
            if el.coords.x > self.nodes[result_ind].coords.x {
                result_ind = ind;
            }
        }
        self.nodes[result_ind].coords.x
    }

    pub fn find_biggest_y(&self) -> f64 {
        let mut result_ind = 0;
        for (ind, el) in self.nodes.iter().enumerate() {
            if el.coords.y > self.nodes[result_ind].coords.y {
                result_ind = ind;
            }
        }
        self.nodes[result_ind].coords.y
    }

    pub fn find_smallest_x(&self) -> f64 {
        let mut result_ind = 0;
        for (ind, el) in self.nodes.iter().enumerate() {
            if el.coords.x < self.nodes[result_ind].coords.x {
                result_ind = ind;
            }
        }
        self.nodes[result_ind].coords.x
    }

    pub fn find_smallest_y(&self) -> f64 {
        let mut result_ind = 0;
        for (ind, el) in self.nodes.iter().enumerate() {
            if el.coords.y < self.nodes[result_ind].coords.y {
                result_ind = ind;
            }
        }
        self.nodes[result_ind].coords.y
    }

    pub fn is_in_x_range(&self, node: &Node) -> bool {
        return node.coords.x >= self.find_smallest_x() && node.coords.x <= self.find_biggest_x();
    }

    pub fn is_in_y_range(&self, node: &Node) -> bool {
        return node.coords.y >= self.find_smallest_y() && node.coords.y <= self.find_biggest_y();
    }

    pub fn contains_node(&self, node: &Node) -> bool {
        // Сначала произведем поиск методо << Заметания >>
        // Проверяем входит ли координата X точки в нашу ячейку
        // Проверяем входит ли координата Y точки в нашу ячейку

        return self.is_in_x_range(node) && self.is_in_y_range(node) &&
            // Теперь произведем более точный поиск методом << Ориентированной ячейки >>
            (Line::new(&self.nodes[0], &self.nodes[1]).find_position(node) >= 0.0
            && Line::new(&self.nodes[1], &self.nodes[2]).find_position(node) >= 0.0
            && Line::new(&self.nodes[2], &self.nodes[3]).find_position(node) >= 0.0
            && Line::new(&self.nodes[3], &self.nodes[0]).find_position(node) >= 0.0);
    }

    pub fn find_cross_point_x(&self, node: &Node) -> Option<CrossPoints> {
        // Создадим прямую, проходящую через переданный узел и точку с такой же ординатой
        let scan_line: Line = Line::new(node, &Node::new(-1, node.coords.x + 1.0, node.coords.y));
        if node.coords.x == self.find_smallest_x() || node.coords.x == self.find_biggest_x() {
            return None;
        }

        if let Some(small) = scan_line.lines_intersect(&Line::new(&self.nodes[3], &self.nodes[0])) {
            if let Some(big) = scan_line.lines_intersect(&Line::new(&self.nodes[1], &self.nodes[2]))
            {
                return Some(CrossPoints::new(&small, &big));
            }
        }
        return None;
    }
}

#[cfg(test)]
#[path = "./tests/cell_test.rs"]
mod cell_test;
