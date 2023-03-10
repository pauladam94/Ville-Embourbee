#[derive(Debug, PartialEq, Clone, serde::Deserialize, serde::Serialize)]
pub enum State {
    Idle,
    Dragging,
    RightClicked,
}

impl std::fmt::Display for State {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            State::Idle => write!(f, "Idle")?,
            State::Dragging => write!(f, "Dragging")?,
            State::RightClicked => write!(f, "RightClicked")?,
        }
        Ok(())
    }
}
