pub struct File {
    name: String,
    content: String
}

impl File {
    pub fn new(name: String, content: String) -> Self {
        Self {
            name, content
        }
    }

    pub fn name(&self) -> String {
        self.name.clone()
    }

    pub fn content(&self) -> String {
        self.content.clone()
    }
}