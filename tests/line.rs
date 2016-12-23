extern crate svg_graph;

use svg_graph::{Graph, LineBuilder, Entry};

#[test]
fn line_graph_to_file() {
    let entries = vec![
        Entry::new("One", 120),
        Entry::new("Two", 200),
        Entry::new("Three", 290),
        Entry::new("Four", 50),
        Entry::new("Five", 21),
        Entry::new("Six", 170),
        Entry::new("Seven", 77),
        Entry::new("Eight", 128)
    ];
    let line = LineBuilder::new()
                .width(500.0)
                .height(500.0)
                .entries(entries)
                .build();
    match line.into_file("./images/line.svg") {
        Err(e) => {
            panic!("Couldn't save to file {}", e);
        }
        Ok(_) => {}
    }
}
