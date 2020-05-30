use super::node::Node;

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct CrossPoints {
    pub(crate) small: Node,
    pub(crate) big: Node,
}

impl CrossPoints {
    pub fn new(node_1: &Node, node_2: &Node) -> CrossPoints {
        CrossPoints {
            small: node_1.clone(),
            big: node_2.clone(),
        }
    }

    pub fn the_same(&self, other: &CrossPoints) -> bool {
        return self.small.the_same(&other.small) && self.big.the_same(&other.big);
    }
}
