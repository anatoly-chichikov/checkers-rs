#[derive(Clone)]
pub struct AIState {
    pub is_thinking: bool,
    pub last_error: Option<String>,
}

impl AIState {
    pub fn new() -> Self {
        Self {
            is_thinking: false,
            last_error: None,
        }
    }

    pub fn start_thinking(&mut self) {
        self.is_thinking = true;
        self.last_error = None;
    }

    pub fn stop_thinking(&mut self) {
        self.is_thinking = false;
    }

    pub fn set_error(&mut self, error: String) {
        self.last_error = Some(error);
        self.is_thinking = false;
    }

    pub fn clear_error(&mut self) {
        self.last_error = None;
    }
}

impl Default for AIState {
    fn default() -> Self {
        Self::new()
    }
}