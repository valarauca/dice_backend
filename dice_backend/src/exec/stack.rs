use super::super::cfgbuilder::Identifier;

#[derive(Default, Clone)]
pub struct Stack {
    data: Vec<Identifier>,
}
impl Stack {
    /// returns the current frame
    pub fn get_current_frame(&self) -> Option<Identifier> {
        if self.data.len() == 0 {
            None
        } else {
            Some(self.data[self.data.len() - 1].clone())
        }
    }

    /// add a frame to the stack
    pub fn push_frame(&mut self, id: Identifier) {
        self.data.push(id);
    }

    /// remove a frame from the stack
    pub fn pop_frame(&mut self) {
        self.data.pop();
    }
}
