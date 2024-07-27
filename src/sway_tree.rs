use crate::tree_tools::{
    self,
    compositor_tree::{CompositorNode, CompositorNodeType},
    session_tree::{
        Programm, WindowCompositionGeometry, WindowCompositionLayout, WindowCompositionProperties,
    },
};
use swayipc::{Node, NodeType};

trait ClonableIterator: Iterator {
    fn clone_box(&self) -> Box<dyn ClonableIterator<Item = Self::Item>>;
}

impl<T, I> ClonableIterator for T
where
    T: Iterator<Item = I> + Clone + 'static,
{
    fn clone_box(&self) -> Box<dyn ClonableIterator<Item = I>> {
        Box::new(self.clone())
    }
}
pub struct SwayNode {
    node: Node,
    child_nodes: Box<dyn ClonableIterator<Item = Node>>,
    floating_nodes: Box<dyn ClonableIterator<Item = Node>>,
}

impl Clone for SwayNode {
    fn clone(&self) -> Self {
        SwayNode {
            node: self.node.clone(),
            child_nodes: self.child_nodes.clone_box(),
            floating_nodes: self.floating_nodes.clone_box(),
        }
    }
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

impl<T> CompositorNode<T> for SwayNode
where
    T: Programm,
{
    //type Item = SwayNode;
    fn get_node_type(&self) -> CompositorNodeType {
        match self.node.node_type {
            NodeType::Root => CompositorNodeType::Root,
            NodeType::Workspace => CompositorNodeType::Workspace,
            //NodeType::Output => CompositorNodeType::Output,
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
            //output: self.node.output.clone(),
            geometry: WindowCompositionGeometry {
                x_position: self.node.rect.x,
                y_position: self.node.rect.y,
                width: self.node.rect.width,
                heigth: self.node.rect.height,
            },
            layout,
            programm: None,
            process_pid: self.node.pid,
            extra_properties: None,
        }
    }

    fn get_ouptut(&self) -> Option<String> {
        self.node.output.clone()
    }
}

impl<'a> Iterator for SwayNode {
    type Item = SwayNode;

    fn next(&mut self) -> Option<Self::Item> {
        match self.child_nodes.next() {
            Some(node) => Some(SwayNode::new(node)),
            None => match self.floating_nodes.next() {
                Some(f_node) => Some(SwayNode::new(f_node)),
                None => None,
            },
        }
    }
}
