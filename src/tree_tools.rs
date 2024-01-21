pub mod compositor_tree {
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
        None,
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
} /* compositor_tree */

pub(crate) mod session_tree {

    enum WindowCompositionLayout {
        VerticalSplit,
        HorizontalSplit,
        Tabbed,
        Stacked,
        None, // TODO: add single window enum or same as None?
    }

    struct WindowCompositionProperties {}

    pub(crate) struct Session {
        workspaces: Vec<Workspace>,
    }

    pub(crate) struct Workspace {
        output: String,
        window_composition: WindowCompositionTree,
    }

    struct WindowCompositionTree {
        uuid: i64,
        window_compositions: Vec<WindowCompositionTree>,
        // TODO: wrap geometry to struct?
        x_position: i32,
        y_position: i32,
        width: i32,
        heigth: i32,
        layout: Option<WindowCompositionLayout>,
        properties: Option<WindowCompositionProperties>,
        process_pid: Option<i32>,
    }
    
} /* session_tree */