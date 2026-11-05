#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum SessionState { #[default] Initializing, GreeterActive, UserSessionActive, Terminated }

#[derive(Debug, Default)]
pub struct DesktopSessionManager {
    state: SessionState,
}

impl DesktopSessionManager {
    pub fn new() -> Self { Self::default() }
    pub fn activate_greeter(&mut self) { self.state = SessionState::GreeterActive; }
    pub fn log_in_user(&mut self) { self.state = SessionState::UserSessionActive; }
    pub fn get_state(&self) -> SessionState { self.state }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_desktop_session_lifecycle() {
        let mut session = DesktopSessionManager::new();
        session.activate_greeter();
        assert_eq!(session.get_state(), SessionState::GreeterActive);
        session.log_in_user();
        assert_eq!(session.get_state(), SessionState::UserSessionActive);
    }
}
