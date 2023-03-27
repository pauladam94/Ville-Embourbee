#[derive(Debug)]
pub enum Response {
    None,
    NewVertex(usize, usize),
    RightClicked,
    Dragging,
}
