use swayipc::{Connection, Event, EventType, Fallible};

mod tree_tools;
mod sway_tree;
// Example subscribung for events,
// just copied from https://github.com/JayceFayne/swayipc-rs/blob/master/examples/hovered_window/src/main.rs 
fn main() -> Fallible<()> {
    for event in Connection::new()?.subscribe([EventType::Window])? {
        match event? {
            Event::Window(w) => println!(
                "{}",
                w.container.name.unwrap_or_else(|| "unnamed.".to_owned())
            ),
            _ => unreachable!(),
        }
    }
    Ok(())
}
