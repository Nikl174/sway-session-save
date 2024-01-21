pub mod session_tree_tools {
    // Enmu for typical types of existing objects in a tiling compositor
    pub enum CompositorNodeType {
        // the actual window associated with a process PID
        Window,
        // a composition of multiple windows or even WindowCompositions itself
        WindowComposition,
        // a WindowComposition possible to be shown (simultaneously) on a display
        Workspace,
        // a physical output device/display where a Workspace can be shown
        Output,
        // the root Node/Object of the compositor
        Root,
        // fallback, should be used for ignored components in the compositor
        None
    }

    // Trait for an compositor data structure/'tree' which is needed for parsing and saving the
    // window state 
    pub trait CompositorTree {
        // type of the childs in the tree, often the tree object it self implementing CompositorTree
        type ChildCompositorNode;

        // return the Type of the current root CompositorNode
        fn get_node_type(&self) -> CompositorNodeType;

        // Iterate over the tree returning the subtree of the current child (meaning current child
        // is the new returned root of the subtree) and as a typical iterator advancing itself to
        // the next child
        fn next_subtree(&mut self) -> Option<Self::ChildCompositorNode>;
    }


} /* tree_tools */
