use json::JsonValue;
use sway_tree::SwayNode;
use swayipc::{Connection, Event, EventType, Fallible};

pub mod tree_tools;
mod sway_tree;

// Example subscribung for events,
// just copied from https://github.com/JayceFayne/swayipc-rs/blob/master/examples/hovered_window/src/main.rs
fn main() -> Fallible<()> {
    // for event in Connection::new()?.subscribe([EventType::Workspace])? {
    //     match event? {
    //         Event::Window(w) => println!(
    //             "{}",
    //             w.container.name.unwrap_or_else(|| "unnamed.".to_owned())
    //         ),
    //         _ => unreachable!(),
    //     }
    // }
    let tree_node = Connection::new()
        .expect("Sway ipc Connection error!")
        .get_tree()
        .expect("Couldn't get the sway tree");
    //print!("{:?}",tree_node.node_type);
    let node = tree_tools::compositor_tree::construct_session::<SwayNode, _>(SwayNode::new(tree_node)).unwrap();
    let json: JsonValue = node.into();
    println!("{}",json);

    Ok(())
}
