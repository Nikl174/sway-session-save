pub mod session_tree {
    use json::{object, JsonValue};

    // TODO: IMPLEMENT programm abstraction
    pub trait Programm: Clone {}

    impl<T: std::clone::Clone> Programm for T {}

    #[derive(Debug, Clone, Copy)]
    pub enum WindowCompositionLayout {
        VerticalSplit,
        HorizontalSplit,
        Tabbed,
        Stacked,
        None, // TODO: add single window enum or same as None?
    }

    #[derive(Debug, Clone, Copy)]
    pub struct ExtraProperties {}

    #[derive(Debug, Clone)]
    pub struct WindowCompositionProperties<T>
    where
        T: Programm,
    {
        pub uuid: i64,
        pub layout: WindowCompositionLayout,
        pub geometry: WindowCompositionGeometry,
        //pub output: Option<String>, // TODO: not optimal. only needed in Workspace ?
        pub programm: Option<T>,
        // unneeded?
        pub process_pid: Option<i32>,
        pub extra_properties: Option<ExtraProperties>,
    }

    //pub fn composition_properties_to_json<T: Programm>(
    //    props: &WindowCompositionProperties<T>,
    //) -> JsonValue {
    //    object! {
    //        uuid: props.uuid,
    //        layout: props.layout,
    //        geometry: object! {
    //            x_position: props.geometry.x_position,
    //            y_position: props.geometry.y_position,
    //            width: props.geometry.width,
    //            heigth: props.geometry.heigth,
    //        },
    //        output: match &props.output {
    //            None => "null".to_string(),
    //            Some(o) => o.clone(),
    //        },
    //        // TODO: IMPLEMENT programm abstraction
    //        programm: "NOT IMPLEMENTED",
    //        process_pid: json::stringify(props.process_pid),
    //        extra_properties: "NOT_USED",
    //
    //
    //    }
    //}

    #[derive(Debug, Clone)]
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
        pub output: Option<String>,
    }

    #[derive(Debug, Clone)]
    pub struct WindowCompositionNode<T>
    where
        T: Programm,
    {
        pub(crate) properties: WindowCompositionProperties<T>,
        pub(crate) window_compositions: Vec<WindowCompositionNode<T>>,
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

    impl<T: Clone> Into<JsonValue> for WindowCompositionProperties<T> {
        fn into(self) -> JsonValue {
            object! {
            uuid: self.uuid,
            layout: self.layout,
            geometry: object! {
                x_position: self.geometry.x_position,
                y_position: self.geometry.y_position,
                width: self.geometry.width,
                heigth: self.geometry.heigth,
            },
            //output: match &self.output {
            //    None => "null".to_string(),
            //    Some(o) => o.clone(),
            //},
            // TODO: IMPLEMENT programm abstraction
            programm: "NOT IMPLEMENTED",
            process_pid: json::stringify(self.process_pid),
            extra_properties: "NOT_USED",
            }
        }
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

    use std::{io, iter::zip};

    use json::{object, JsonValue};

    use super::session_tree::{self, Programm, Session, WindowCompositionNode, Workspace};

    // Enmu for typical types of existing objects in a tiling compositor
    pub enum CompositorNodeType {
        // the actual window associated with a process PID
        Window,
        // a composition of multiple windows or even WindowCompositions itself
        WindowComposition,
        // a WindowComposition possible to be shown (simultaneously) on a display
        Workspace,
        //// a physical output device/display where a Workspace can be shown
        //Output,
        // the root Node/Object of the compositor
        Root,
        // fallback, should be used for ignored components in the compositor
        None,
    }

    impl Into<JsonValue> for CompositorNodeType {
        fn into(self) -> JsonValue {
            match self {
                CompositorNodeType::Window => "Window".into(),
                CompositorNodeType::WindowComposition => "WindowComposition".into(),
                CompositorNodeType::Workspace => "Workspace".into(),
                //CompositorNodeType::Output => "Output".into(),
                CompositorNodeType::Root => "Root".into(),
                CompositorNodeType::None => JsonValue::Null,
            }
        }
    }

    // Trait for an compositor data structure/'tree' which is needed for parsing and saving the
    // window state
    pub trait CompositorNode<T>: Sized + Clone
    where
        T: Programm,
    {
        // return the Type of the current root CompositorNode
        fn get_node_type(&self) -> CompositorNodeType;

        // Returns the properties of the current node
        fn get_properties(&self) -> session_tree::WindowCompositionProperties<T>;

        // Returns the output of the current Node as a string-representation or None, if node has
        // no output
        // Especially important for Nodes with Type: CompositorNode::Workspace because this is used
        // to determine, on which output the workspace is going to land on reconstruction
        fn get_ouptut(&self) -> Option<String>;
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
                        output: node_root.get_ouptut(),
                    }],
                })
            }
            CompositorNodeType::WindowComposition | CompositorNodeType::Workspace | CompositorNodeType::None => {
                let output = node_root.get_ouptut();
                let comp_node = construct_composition_node(node_root);
                return Ok(Session {
                    workspaces: vec![Workspace {
                        window_composition: comp_node,
                        output: output,
                    }],
                });
            }
            CompositorNodeType::Root => {
                let node_iter = node_root.clone().into_iter();
                let comp_node = construct_composition_node(node_root);
                let mut workspaces: Vec<Workspace<T>> = Vec::new();
                for (node, comp_node) in zip(node_iter, comp_node) {
                    workspaces.push(Workspace {
                        window_composition: comp_node,
                        output: node.get_ouptut(),
                    });
                }
                return Ok(Session { workspaces });
            }
        }
    }

    impl<T: Programm> Into<JsonValue> for WindowCompositionNode<T> {
        fn into(self) -> JsonValue {
                let node_type: JsonValue = match self.window_compositions.is_empty() {
                    true => CompositorNodeType::Window.into(),
                    false => CompositorNodeType::WindowComposition.into(), 
                };
            let props: JsonValue = self.clone().properties.into();
            let node_iter = self.into_iter();
            let json_data = node_iter.fold(json::JsonValue::new_array(), |mut acc, node| {
                let json: JsonValue = node.into();
                acc.push(json).expect("Couldn't push json value into json object in convert_composition_node_to_json!");
                acc
            }
            );
            object! {node_type: node_type,properties: props, nodes:json_data}
        }
    }

    impl<T: Programm> Into<JsonValue> for Workspace<T> {
        fn into(self) -> JsonValue {
            let node_type: JsonValue = CompositorNodeType::Workspace.into();
            let nodes: JsonValue = self.window_composition.window_compositions.into();
            let properties: JsonValue = self.window_composition.properties.into();
            let output: JsonValue = json::stringify(self.output).into();
            object! {node_type: node_type, output: output, properties: properties, nodes: nodes}
        }
    }

    impl<T: Programm> Into<JsonValue> for Session<T> {
        fn into(self) -> JsonValue {
            let node_type: JsonValue = CompositorNodeType::Root.into();

            let mut json_ws = json::Array::new();
            for workspace in self.workspaces {
                json_ws.push(workspace.into());
            }
            object! {
                node_type: node_type,
                workspaces: json_ws,
            }
        }
    }
} /* compositor_tree */
