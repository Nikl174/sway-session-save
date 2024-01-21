use crate::tree_tools::{self, session_tree_tools::{CompositorTree, self, CompositorNodeType}};
use swayipc::{Node, NodeType};


impl CompositorTree for Node {
    type ChildCompositorNode = Node;
    fn get_node_type(&self) -> CompositorNodeType {
        match self.node_type {
            NodeType::Root => CompositorNodeType::Root,
            NodeType::Workspace => CompositorNodeType::Workspace,
            NodeType::Output => CompositorNodeType::Output,
            NodeType::Con | NodeType::FloatingCon => {
                    if self.nodes.is_empty() {
                        CompositorNodeType::Window
                    } else {
                        CompositorNodeType::WindowComposition
                    }
                },
            NodeType::Dockarea => CompositorNodeType::None,
            _ => CompositorNodeType::None
        }
    }
    fn next_subtree(&mut self) -> Option<Self::ChildCompositorNode> {
        let node_iter = &mut self.nodes.into_iter();
        let floating_node_iter = &mut self.floating_nodes.into_iter();

        match node_iter.next() {
            Some(node) => Some(node),
            None => floating_node_iter.next()
        }

    }
}
