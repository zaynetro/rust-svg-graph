extern crate svg_graph;

use svg_graph::{Graph, Bar, Entry};

#[test]
fn bar_graph_to_file() {
    let entries = vec![
        Entry::new("One", 10),
        Entry::new("Two", 20),
        Entry::new("Three", 30),
        Entry::new("Four", 5)
    ];
    let bar = Bar::new(entries);
    match bar.into_file("./images/bar.svg") {
        Err(e) => {
            panic!("Couldn't save to file {}", e);
        }
        Ok(_) => {}
    }
}

//#[test]
//fn bar_graph_render_empty() {
    //let entries = vec![];
    //let bar = Bar::new(entries);
    //let s = bar.render();
    //assert_eq!(s, "<svg></svg>");
//}

//#[test]
//fn bar_graph_render() {
    //let entries = vec![
        //BarEntry::new("One", 10),
        //BarEntry::new("Two", 20),
        //BarEntry::new("Three", 30)
    //];
    //let bar = Bar::new(entries);
    //let s = bar.render();
    //assert_eq!(s, "<svg><g></g><g></g><g></g></svg>");
//}
