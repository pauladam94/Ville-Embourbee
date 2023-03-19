use crate::circle::Circle;
use crate::image::Image;
use crate::response::Response;
use crate::state::State;
use crate::vertex::Vertex;

#[derive(Debug, Clone)]
pub enum Drawables {
    Circle(Circle),
    Image(Image),
}

#[derive(Debug, Clone)]
pub struct Node {
    pub id: usize,
    is_dragging: bool,
    drag_start: egui::Pos2,
    pub drawable: Drawables,
}

impl Node {
    pub fn new(id: usize, drawable: Drawables) -> Self {
        Self {
            id,
            is_dragging: false,
            drag_start: egui::Pos2::ZERO,
            drawable,
        }
    }

    pub fn pos(&self) -> egui::Pos2 {
        match self.drawable {
            Drawables::Circle(circle) => circle.center(),
            // Choose if you want the center or the top left corner of the image
            Drawables::Image(image) => image.pos,
        }
    }

    pub fn setpos(&mut self, pos: egui::Pos2) {
        match self.drawable {
            Drawables::Circle(ref mut circle) => circle.center = pos,
            Drawables::Image(ref mut image) => image.pos = pos,
        }
    }

    pub fn contains(&self, pos: egui::Pos2) -> bool {
        match self.drawable {
            Drawables::Circle(circle) => circle.contains(pos),
            Drawables::Image(image) => image.contains(pos),
        }
    }

    pub fn handle_event(&mut self, event: &egui::Event, state: &mut State) {
        // move around the circle with the left click
        match state.clone() {
            State::Idle => {
                if let egui::Event::PointerButton {
                    pos,
                    button: egui::PointerButton::Primary,
                    pressed: true,
                    ..
                } = event
                {
                    if self.contains(*pos) {
                        self.is_dragging = true;
                        self.drag_start = *pos;
                        *state = State::Dragging;
                    }
                }
            }
            State::Dragging => match event {
                egui::Event::PointerMoved(pos) => {
                    if self.is_dragging {
                        let delta = *pos - self.drag_start;
                        self.setpos(self.pos() + delta);
                        self.drag_start = *pos;
                    }
                }
                egui::Event::PointerButton {
                    button: egui::PointerButton::Primary,
                    pressed: false,
                    ..
                } => {
                    if self.is_dragging {
                        self.is_dragging = false;
                        *state = State::Idle;
                    }
                }
                _ => {}
            },
            _ => {}
        }
    }

    pub fn handle_event_graph(
        &mut self,
        event: &egui::Event,
        state: &mut State,
        new_vertex: &mut Vertex,
    ) -> Response {
        // move around the circle with the left click

        // the right click to choose a node
        // left click a right click after to choose another circle
        // and make a vertex between them

        match state.clone() {
            State::Idle => match event {
                egui::Event::PointerButton {
                    pos,
                    button: egui::PointerButton::Primary,
                    pressed: true,
                    ..
                } => {
                    if self.contains(*pos) {
                        self.is_dragging = true;
                        self.drag_start = *pos;
                        *state = State::Dragging;
                        return Response::Dragging;
                    }
                    Response::None
                }
                egui::Event::PointerButton {
                    pos,
                    button: egui::PointerButton::Secondary,
                    pressed: true,
                    ..
                } => {
                    if self.contains(*pos) {
                        // self.is_right_clicked = true;
                        self.drag_start = *pos;
                        *state = State::RightClicked;

                        // Select the first circle
                        // self.is_selected = true;
                        new_vertex.node1.id = self.id;
                        new_vertex.node1.setpos(self.pos());
                        new_vertex.node2.id = self.id;
                        new_vertex.node2.setpos(self.pos());
                        return Response::RightClicked;
                    }
                    Response::None
                }
                _ => Response::None,
            },
            State::Dragging => match event {
                egui::Event::PointerMoved(pos) => {
                    if self.is_dragging {
                        let delta = *pos - self.drag_start;
                        self.setpos(self.pos() + delta);
                        self.drag_start = *pos;
                        return Response::Dragging;
                    }
                    Response::None
                }
                egui::Event::PointerButton {
                    button: egui::PointerButton::Primary,
                    pressed: false,
                    ..
                } => {
                    if self.is_dragging {
                        self.is_dragging = false;
                        *state = State::Idle;
                    }
                    Response::None
                }
                _ => Response::None,
            },
            State::RightClicked => match event {
                egui::Event::PointerButton {
                    pos,
                    button: egui::PointerButton::Secondary,
                    pressed: true,
                    ..
                } => {
                    if self.contains(*pos) {
                        *state = State::Idle;
                        new_vertex.node2.id = self.id;
                        new_vertex.node2.setpos(self.pos());
                        return Response::NewVertex(new_vertex.node1.id, new_vertex.node2.id);
                    }
                    Response::None
                }
                _ => Response::None,
            },
        }
    }

    /*
    pub fn handle_event(&mut self, event: &egui::Event, state: &mut State) {
        match self.drawable {
            Drawables::Circle(circle) => {
                circle.handle_event(event, state);
            }
            Drawables::Image(image) => {
                image.handle_event(event, state);
            }
        }
        self.circle.handle_event(event, state);
    }

    pub fn handle_event_graph(
        &mut self,
        event: &egui::Event,
        state: &mut State,
        new_vertex: &mut Vertex,
    ) -> Response {
        self.circle
            .handle_event_graph(event, state, self.id, new_vertex)
    }
    */

    pub fn follow_mouse(&mut self, event: &egui::Event) {
        match &mut self.drawable {
            Drawables::Circle(circle) => {
                circle.follow_mouse(event);
            }
            Drawables::Image(image) => {
                image.follow_mouse(event);
            }
        }
    }

    pub fn draw(&mut self, ui: &mut egui::Ui, stroke: egui::Stroke) {
        match &mut self.drawable {
            Drawables::Circle(circle) => {
                circle.draw(ui, stroke);
            }
            Drawables::Image(image) => {
                image.draw(ui, stroke);
            }
        }
    }
}

impl std::fmt::Display for Node {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.id)?;
        write!(f, " dragged : {}", self.is_dragging)?;
        Ok(())
    }
}

pub fn pos2_to_node(id: usize, pos: egui::Pos2) -> Node {
    Node {
        id,
        is_dragging: false,
        drag_start: egui::Pos2::ZERO,
        drawable: Drawables::Circle(Circle::new(pos)),
    }
}
