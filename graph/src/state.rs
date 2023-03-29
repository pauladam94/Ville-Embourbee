#[derive(Default, Debug, PartialEq, Clone)]
pub enum State {
    #[default]
    Idle,
    Dragging,
    RightClicked,
    AddNode,
}

impl std::fmt::Display for State {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            State::Idle => write!(f, "Idle")?,
            State::Dragging => write!(f, "Dragging")?,
            State::RightClicked => write!(f, "RightClicked")?,
            State::AddNode => write!(f, "AddNode")?,
        }
        Ok(())
    }
}
