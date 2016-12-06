pub struct Entry {
    pub label: String,
    pub value: i32
}

impl Entry {
    pub fn new<S>(label: S, value: i32) -> Entry where S: Into<String> {
        Entry {
            label: label.into(),
            value: value
        }
    }
}
