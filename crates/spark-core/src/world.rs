pub struct World {
    pub name: String,
}

impl World {
    pub fn new(name: &str) -> Self {
        World {
            name: name.to_string(),
        }
    }
}
