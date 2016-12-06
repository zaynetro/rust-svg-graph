//! The `svg_graph` crate documentation

#![feature(plugin)]
#![plugin(maud_macros)]

extern crate maud;

mod graph;
mod line;
mod bar;
mod pie;
mod entry;
mod scale;
mod axis;

pub use graph::Graph;
pub use entry::Entry;
pub use bar::Bar;
pub use line::Line;
pub use pie::Pie;

//#[cfg(test)]
//mod tests {
    //use super::*;

    //#[test]
    //fn bar_entry_height() {
        //let entries = vec![
            //BarEntry::new("One", 10),
            //BarEntry::new("Two", 20)
        //];
        //let bar = Bar::new(entries);
        ////assert_eq!(bar.max_entry_value(), 20);
        ////assert_eq!(bar.entry_height(10), 250.0);
        ////assert_eq!(bar.entry_height(20), 500.0);
    //}
//}
