pub mod session_tree {
    use json::{object, JsonValue};

    // TODO: IMPLEMENT programm abstraction
    pub trait Programm {}

    impl<T> Programm for T {}

    #[derive(Debug, Clone, Copy)]
    pub enum WindowCompositionLayout {
        VerticalSplit,
        HorizontalSplit,
        Tabbed,
        Stacked,
        None, // TODO: add single window enum or same as None?
    }

    impl Into<JsonValue> for WindowCompositionLayout {
        fn into(self) -> JsonValue {
            match self {
                WindowCompositionLayout::VerticalSplit => "VerticalSplit".into(),
                WindowCompositionLayout::HorizontalSplit => "HorizontalSplit".into(),
                WindowCompositionLayout::Tabbed => "Tabbed".into(),
                WindowCompositionLayout::Stacked => "Stacked".into(),
                WindowCompositionLayout::None => "null".into(),
            }
        }
    }

    #[derive(Debug, Clone, Copy)]
    pub struct ExtraProperties {}

    pub struct WindowCompositionProperties<T>
    where
        T: Programm,
    {
        pub uuid: i64,
        pub layout: WindowCompositionLayout,
        pub geometry: WindowCompositionGeometry,
        pub output: Option<String>, // TODO: not optimal. only needed in Workspace
        pub programm: Option<T>,
        // unneeded?
        pub process_pid: Option<i32>,
        pub extra_properties: Option<ExtraProperties>,
    }

    pub fn composition_properties_to_json<T: Programm>(
        props: &WindowCompositionProperties<T>,
    ) -> JsonValue {
        object! {
            uuid: props.uuid,
            layout: props.layout,
            geometry: object! {
                x_position: props.geometry.x_position,
                y_position: props.geometry.y_position,
                width: props.geometry.width,
                heigth: props.geometry.heigth,
            },
            output: match &props.output {
                None => "null".to_string(),
                Some(o) => o.clone(),
            },
            // TODO: IMPLEMENT programm abstraction
            programm: "NOT IMPLEMENTED",
            process_pid: json::stringify(props.process_pid),
            extra_properties: "NOT_USED",


        }
    }

    pub struct WindowCompositionGeometry {
        pub x_position: i32,
        pub y_position: i32,
        pub width: i32,
        pub heigth: i32,
    }

    pub struct Session<T>
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

    pub struct WindowCompositionNode<T>
    where
        T: Programm,
    {
        pub(crate) properties: WindowCompositionProperties<T>,
        pub(crate) window_compositions: Vec<WindowCompositionNode<T>>,
    }

    impl<T: Programm> IntoIterator for WindowCompositionNode<T> {
        type Item = WindowCompositionNode<T>;

        type IntoIter = std::vec::IntoIter<WindowCompositionNode<T>>;

        fn into_iter(self) -> Self::IntoIter {
            self.window_compositions.into_iter()
        }
    }
} /* session_tree */
pub mod compositor_tree {

    use std::io;

    use json::{object, JsonValue};

    use super::session_tree::{
        self, composition_properties_to_json, Programm, Session, WindowCompositionNode, Workspace,
    };

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

    pub fn construct_composition_node<T, C>(node_root: C) -> WindowCompositionNode<T>
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
            _ => (),
        }
        let props = node_root.get_properties();
        // recursion construct
        let node_vec: Vec<WindowCompositionNode<T>> =
            node_root.fold(Vec::new(), |mut acc, node| {
                let props = node.get_properties();
                let n_vec = construct_composition_node(node);
                acc.push(WindowCompositionNode {
                    properties: props,
                    window_compositions: vec![n_vec],
                });
                return acc;
            });

        WindowCompositionNode {
            properties: props,
            window_compositions: node_vec,
        }
    }

    // TODO: implement own errors
    pub fn construct_session<T, C>(node_root: C) -> Result<Session<T>, io::Error>
    where
        T: Programm,
        C: CompositorNode<T> + Iterator<Item = C>,
    {
        match node_root.get_node_type() {
            CompositorNodeType::Window => {
                return Ok(Session {
                    workspaces: vec![Workspace {
                        window_composition: WindowCompositionNode {
                            window_compositions: Vec::new(),
                            properties: node_root.get_properties(),
                        },
                    }],
                })
            }
            // TODO: ist this right for WindowComposition?
            //CompositorNodeType::WindowComposition | CompositorNodeType::Workspace => {
            _ => {
                let comp_node = construct_composition_node(node_root);
                return Ok(Session {
                    workspaces: vec![Workspace {
                        window_composition: comp_node,
                    }],
                });
            } //CompositorNodeType::Output => todo!(),
              //CompositorNodeType::Root => todo!(),
              //CompositorNodeType::None => todo!(),
        }
    }

    pub fn convert_composition_node_to_json<T: Programm>(
        node_root: WindowCompositionNode<T>,
    ) -> JsonValue {
        let props = composition_properties_to_json(&node_root.properties);
        let node_iter = node_root.into_iter();
        let json_data = node_iter.fold(json::JsonValue::new_array(), |mut acc, node| {
            let props_j = composition_properties_to_json(&node.properties);
            let mut json_nodes = json::JsonValue::new_array();
            for n in node.window_compositions {
                json_nodes.push(convert_composition_node_to_json(n)).expect("Couldn't push json value into json object in convert_composition_node_to_json fold!");
            }
            let json = object! {
                properties: props_j,
                nodes: json_nodes
            };
            acc.push(json).expect("Couldn't push json value into json object in convert_composition_node_to_json!");
            acc
        });
        object! {properties: props, nodes:json_data}
    }
} /* compositor_tree */
