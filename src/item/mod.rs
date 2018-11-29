use std::collections::HashMap;

/// A representation of an item within the World to be interacted with by the user
pub struct Item {
    name: String,
    desc: String,
    /// Items contained within the Item if capable
    pub contents: Option<HashMap<String, Box<Item>>>,
}

impl Item {
    pub fn new(name: &str, desc: &str, contents: Option<HashMap<String, Box<Item>>>) -> Self {
        Self {
            name: name.to_owned(),
            desc: desc.to_owned(),
            contents,
        }
    }
    pub fn name(&self) -> String {
        self.name.clone()
    }
    pub fn desc(&self) -> String {
        self.desc.clone()
    }
}
