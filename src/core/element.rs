pub trait Element {}

pub struct WoopsaElement {
    pub name: String,
}

impl WoopsaElement {
    fn new(&mut self, name: String) {
        self.name = name;
    }
}

impl Element for WoopsaElement {}
