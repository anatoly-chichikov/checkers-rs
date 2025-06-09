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

    pub fn start_thinking(&self) -> Self {
        let mut new_state = self.clone();
        new_state.is_thinking = true;
        new_state.last_error = None;
        new_state
    }

    pub fn set_error(&self, error: String) -> Self {
        let mut new_state = self.clone();
        new_state.last_error = Some(error);
        new_state.is_thinking = false;
        new_state
    }

    pub fn clear_error(&self) -> Self {
        let mut new_state = self.clone();
        new_state.last_error = None;
        new_state
    }
}

impl Default for AIState {
    fn default() -> Self {
        Self::new()
    }
}
