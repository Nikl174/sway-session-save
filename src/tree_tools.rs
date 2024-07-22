pub mod session_tree {

    pub trait Programm {}

    pub enum WindowCompositionLayout {
        VerticalSplit,
        HorizontalSplit,
        Tabbed,
        Stacked,
        None, // TODO: add single window enum or same as None?
    }

    pub struct ExtraProperties {}

    pub struct WindowCompositionProperties<T>
    where
        Option<T>: Programm,
    {
        pub uuid: i64,
        pub layout: WindowCompositionLayout,
        pub output: Option<String>, // TODO: not optimal. only needed in Workspace
        pub geometry: WindowCompositionGeometry,
        pub programm: T,
        // unneeded?
        pub process_pid: Option<i32>,
        pub extra_properties: Option<ExtraProperties>,
    }

    pub struct WindowCompositionGeometry {
        pub x_position: i32,
        pub y_position: i32,
        pub width: i32,
        pub heigth: i32,
    }

    pub(crate) struct Session<T>
    where
        Option<T>: Programm,
    {
        workspaces: Vec<Workspace<T>>,
    }

    struct Workspace<T>
    where
        Option<T>: Programm,
    {
        window_composition: WindowCompositionNode<T>,
    }

    struct WindowCompositionNode<T>
    where
        Option<T>: Programm,
    {
        properties: WindowCompositionProperties<T>,
        window_compositions: Vec<WindowCompositionNode<T>>,
    }
} /* session_tree */
pub mod compositor_tree {
    use super::session_tree::{self, Programm, Session};

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
    pub trait CompositorNode<T>
    where
        Option<T>: Programm,
    {
        type Item;
        // return the Type of the current root CompositorNode
        fn get_node_type(&self) -> CompositorNodeType;

        //// Iterate over the subtrees returning the next child-node of the current node
        fn next_subtree(&mut self) -> Option<<Self as CompositorNode<T>>::Item>;

        // Returns the properties of the current node
        fn get_properties(&self) -> session_tree::WindowCompositionProperties<T>;
    }

    pub fn construct_compositor_tree<T, C>(node_root: C) -> Session<T>
    where
        Option<T>: Programm,
        C: CompositorNode<T>,
    {

    }
} /* compositor_tree */
