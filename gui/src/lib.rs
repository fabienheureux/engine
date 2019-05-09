#[derive(Default, Debug)]
pub struct Node;

type Nodes = Vec<Node>;

#[derive(Default, Debug)]
pub struct GUI {
    nodes: Nodes,
}

impl GUI {
    pub fn new(nodes: Nodes) -> Self {
        Self { nodes }
    }

    pub fn debug_nodes(&self) {
        dbg!(&self.nodes);
    }
}
