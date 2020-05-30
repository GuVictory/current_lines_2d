pub mod load;
pub mod render;

#[derive(PartialEq)]
pub enum CurrentFilePlace {
    MeshFormat,
    Nodes,
    Elements,
    NodeData,
}
