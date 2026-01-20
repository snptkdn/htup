#[derive(Debug, Clone, PartialEq)]
pub struct Project {
    pub name: String,
}

impl Project {
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            name: name.into(),
        }
    }
}
