use super::node::Node;

#[derive(Copy, Clone)]
#[derive(PartialEq)]
pub struct Line {
    first_point: Node,
    second_point: Node,
    coef_a: f64,
    coef_b: f64,
    coef_c: f64,
}

impl Line {
    pub fn new(first_point_p: &Node, second_point:&Node) -> Line{
        Line {
            first_point: first_point_p.clone(),
            second_point: second_point.clone(),
            coef_a: first_point_p.coords.y - second_point.coords.y,
            coef_b: second_point.coords.x - first_point_p.coords.x,
            coef_c: first_point_p.coords.x * second_point.coords.y
                    - second_point.coords.x * first_point_p.coords.y
        }
    }

    #[cfg_attr(tarpaulin, skip)]
    pub fn the_same(&self, other: &Line) -> bool {
        return self.first_point.the_same(&other.first_point) &&
            self.second_point.the_same(&other.second_point)
    }


    // Предположим, у нас есть 3 точки: А(х1,у1), Б(х2,у2), С(х3,у3).
    // Через точки А и Б проведена прямая.
    // И нам надо определить, как расположена точка С относительно прямой АБ.
    // Для этого вычисляем значение:
    //  D = (х3 - х1) * (у2 - у1) - (у3 - у1) * (х2 - х1)
    //      - Если D = 0 - значит, точка С лежит на прямой АБ.
    //      - Если D < 0 - значит, точка С лежит слева от прямой.
    //      - Если D > 0 - значит, точка С лежит справа от прямой.

    pub fn find_position(&self, node: &Node) -> f64 {
        return  (node.coords.x - self.first_point.coords.x) *
                (self.second_point.coords.y - self.first_point.coords.y) -
                (node.coords.y - self.first_point.coords.y) *
                (self.second_point.coords.x - self.first_point.coords.x)
    }

    pub fn len(&self) -> f64 {
        return  (self.second_point.coords.x - self.first_point.coords.x) *
                (self.second_point.coords.x - self.first_point.coords.x) +
                (self.second_point.coords.y - self.first_point.coords.y) *
                (self.second_point.coords.y - self.first_point.coords.y)
    }

    // Метод, который позволяет проверить параллельны ли две прямые
    pub fn are_parall(&self, other: &Line) -> bool {
        return  (self.coef_a == 0.0 && other.coef_a == 0.0) ||
                (self.coef_b == 0.0 && other.coef_b == 0.0) ||
                (
                    (self.coef_a / other.coef_a) == (self.coef_b / other.coef_b) &&
                    (self.coef_a != 0.0 && self.coef_b != 0.0 && other.coef_a != 0.0 && other.coef_b != 0.0)
                )
    }

    // Метод для поска точек пересечения двух прямых
    pub fn lines_intersect(&self, other: &Line) -> Option<Node> {
        if self.are_parall(&other) {
            return None
        }

        let det: f64 = self.coef_a * other.coef_b - other.coef_a * self.coef_b;
        let x: f64 = (self.coef_b * other.coef_c - other.coef_b * self.coef_c) / det;
        let y: f64 = (other.coef_a * self.coef_c - self.coef_a * other.coef_c) / det;

        return Some(Node::new(-1, x, y))
    }
}

#[cfg(test)]
#[path = "./tests/line_test.rs"]
mod line_test;