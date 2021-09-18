#[derive(Clone, Debug)]
pub enum AppMessage {
    EnterSandboxMode,
    ExitSandboxMode,
    TogglePlayback(usize),
}
