pub type EmpId = i32;

#[derive(Debug, Clone)]
pub struct Emp {
    id: EmpId,
    name: String,
}

impl Emp {
    pub fn new(id: EmpId, name: &str) -> Self {
        Self {
            id,
            name: name.to_string(),
        }
    }
    pub fn id(&self) -> EmpId {
        self.id
    }
    pub fn set_name(&mut self, name: &str) {
        self.name = name.to_string();
    }
}
