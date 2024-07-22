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
        T: Programm,
    {
        pub uuid: i64,
        pub layout: WindowCompositionLayout,
        pub output: Option<String>, // TODO: not optimal. only needed in Workspace
        pub geometry: WindowCompositionGeometry,
        pub programm: Option<T>,
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
        T: Programm,
    {
        pub(crate) workspaces: Vec<Workspace<T>>,
    }

    pub(crate) struct Workspace<T>
    where
        T: Programm,
    {
        pub(crate) window_composition: WindowCompositionNode<T>,
    }

    pub(crate) struct WindowCompositionNode<T>
    where
        T: Programm,
    {
        pub(crate) properties: WindowCompositionProperties<T>,
        pub(crate) window_compositions: Vec<WindowCompositionNode<T>>,
    }
} /* session_tree */
pub mod compositor_tree {
    use super::session_tree::{self, Programm, Session, WindowCompositionNode};

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
    pub trait CompositorNode<T>: Sized
    where
        T: Programm,
    {
        // return the Type of the current root CompositorNode
        fn get_node_type(&self) -> CompositorNodeType;

        // Returns the properties of the current node
        fn get_properties(&self) -> session_tree::WindowCompositionProperties<T>;
    }

    fn construct_composition_node<T, C>(node_root: C) -> WindowCompositionNode<T>
    where
        T: Programm,
        C: CompositorNode<T> + Iterator<Item = C>,
    {
        // recursion base case
        match node_root.get_node_type() {
            CompositorNodeType::Window => {
                return WindowCompositionNode {
                    properties: node_root.get_properties(),
                    window_compositions: Vec::new(),
                }
            }
            _ => ()
        }
        let props = node_root.get_properties();
        // recursion construct 
        let node_vec: Vec<WindowCompositionNode<T>> = node_root.fold(Vec::new(), |mut acc, node| {
            let props = node.get_properties();
            let n_vec = construct_composition_node(node);
            acc.push(WindowCompositionNode {properties: props, window_compositions: vec![n_vec]});
            return acc

        });

        WindowCompositionNode {
            properties: props,
            window_compositions: node_vec,
        }
    }
} /* compositor_tree */
