extern crate svg_graph;

use svg_graph::{Graph, Pie, Entry};

#[test]
fn pie_graph_to_file() {
    let entries = vec![
        Entry::new("One", 10),
        Entry::new("Two", 10),
        Entry::new("Three", 20),
        Entry::new("Four", 30),
        Entry::new("Five", 5),
        Entry::new("Six", 15),
        Entry::new("Seven", 15),
        Entry::new("Eight", 15),
        Entry::new("Nine", 15),
        Entry::new("Ten", 15),
    ];
    let pie = Pie::new(entries);
    match pie.into_file("./images/pie.svg") {
        Err(e) => {
            panic!("Couldn't save to file {}", e);
        }
        Ok(_) => {}
    }
}
