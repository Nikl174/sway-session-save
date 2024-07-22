use crate::tree_tools::{
    self,
    compositor_tree::{CompositorNode, CompositorNodeType},
    session_tree::{
        Programm, WindowCompositionGeometry, WindowCompositionLayout, WindowCompositionProperties,
    },
};
use swayipc::{Node, NodeType};

struct SwayNode {
    node: Node,
    child_nodes: Box<dyn Iterator<Item = Node>>,
    floating_nodes:Box<dyn Iterator<Item = Node>>,
}

impl SwayNode {
    pub fn new(node: Node) -> Self {
        Self {
            node: node.clone(),
            child_nodes: Box::new(node.nodes.into_iter()),
            floating_nodes: Box::new(node.floating_nodes.into_iter()),
        }
    }
}

//impl<'a> Iterator for SwayNode {
//    type Item = SwayNode;
//
//    fn next(&mut self) -> Option<Self::Item> {
//        match self.child_nodes.next() {
//            Some(node) => Some(SwayNode::new(node)),
//            None => match self.floating_nodes.next() {
//                Some(f_node) => Some(SwayNode::new(f_node)),
//                None => None,
//            },
//        }
//    }
//}

impl<T: Programm> CompositorNode<T> for SwayNode {
    type Item = SwayNode;
    fn get_node_type(&self) -> CompositorNodeType {
        match self.node.node_type {
            NodeType::Root => CompositorNodeType::Root,
            NodeType::Workspace => CompositorNodeType::Workspace,
            NodeType::Output => CompositorNodeType::Output,
            NodeType::Con | NodeType::FloatingCon => {
                if self.node.nodes.is_empty() {
                    CompositorNodeType::Window
                } else {
                    CompositorNodeType::WindowComposition
                }
            }
            NodeType::Dockarea => CompositorNodeType::None,
            _ => CompositorNodeType::None,
        }
    }
    fn next_subtree(&mut self) -> Option<SwayNode> {
        match self.child_nodes.next() {
            Some(node) => Some(SwayNode::new(node)),
            None => match self.floating_nodes.next() {
                Some(f_node) => Some(SwayNode::new(f_node)),
                None => None,
            },
        }
    }

    fn get_properties(&self) -> tree_tools::session_tree::WindowCompositionProperties<T> {
        let layout: WindowCompositionLayout = match self.node.layout {
            swayipc::NodeLayout::None => WindowCompositionLayout::None,
            swayipc::NodeLayout::SplitH => WindowCompositionLayout::HorizontalSplit,
            swayipc::NodeLayout::SplitV => WindowCompositionLayout::VerticalSplit,
            swayipc::NodeLayout::Tabbed => WindowCompositionLayout::Tabbed,
            swayipc::NodeLayout::Stacked => WindowCompositionLayout::Stacked,
            _ => WindowCompositionLayout::None,
        };
        WindowCompositionProperties {
            uuid: self.node.id,
            output: self.node.output.clone(),
            geometry: WindowCompositionGeometry {
                x_position: self.node.geometry.x,
                y_position: self.node.geometry.x,
                width: self.node.geometry.width,
                heigth: self.node.geometry.height,
            },
            layout,
            programm: None,
            process_pid: self.node.pid,
            extra_properties: None,
        }
    }
}
