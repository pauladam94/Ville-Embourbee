// use eframe::glow::SIMULTANEOUS_TEXTURE_AND_DEPTH_WRITE;

// use eframe::glow::UNSIGNED_INT_IMAGE_2D;

use crate::response::Response;
use crate::state::State;
use crate::vertex::Vertex;

#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
pub struct CircleWidget {
    pub center: egui::Pos2,
    pub radius: f32,
    pub is_dragging: bool,
    drag_start: egui::Pos2,
    is_right_clicked: bool,
    is_selected: bool,
}

impl CircleWidget {
    // add stroke in the parameters
    pub fn new(center: egui::Pos2) -> Self {
        Self {
            center,
            radius: 10.0,
            is_dragging: false,
            drag_start: egui::Pos2::ZERO,
            is_right_clicked: false,
            is_selected: false,
        }
    }

    fn contains_point(&self, point: egui::Pos2) -> bool {
        let delta = point - self.center;
        delta.length() < self.radius
    }

    pub fn handle_event(&mut self, event: &egui::Event, state: &mut State) {
        // move around the circle with the left click
        match state.clone() {
            State::Idle => match event {
                egui::Event::PointerButton {
                    pos,
                    button: egui::PointerButton::Primary,
                    pressed: true,
                    ..
                } => {
                    if self.contains_point(*pos) {
                        self.is_dragging = true;
                        self.drag_start = *pos;
                        *state = State::Dragging;
                    }
                }
                _ => {}
            },
            State::Dragging => match event {
                egui::Event::PointerMoved(pos) => {
                    if self.is_dragging {
                        let delta = *pos - self.drag_start;
                        self.center += delta;
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
        node_id: usize,
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
                    if self.contains_point(*pos) {
                        self.is_dragging = true;
                        self.drag_start = *pos;
                        *state = State::Dragging;
                        return Response::Dragging
                    }
                    Response::None
                }
                egui::Event::PointerButton {
                    pos,
                    button: egui::PointerButton::Secondary,
                    pressed: true,
                    ..
                } => {
                    if self.contains_point(*pos) {
                        // self.is_right_clicked = true;
                        self.drag_start = *pos;
                        *state = State::RightClicked;

                        // Select the first circle
                        // self.is_selected = true;
                        new_vertex.node1.id = node_id;
                        new_vertex.node1.circle.center = self.center;
                        return Response::RightClicked
                    }
                    Response::None
                }
                _ => Response::None,
            },
            State::Dragging => match event {
                egui::Event::PointerMoved(pos) => {
                    if self.is_dragging {
                        let delta = *pos - self.drag_start;
                        self.center += delta;
                        self.drag_start = *pos;
                        return Response::Dragging
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
                    if self.contains_point(*pos) {
                        *state = State::Idle;
                        new_vertex.node2.id = node_id;
                        new_vertex.node2.circle.center = self.center;
                        return Response::NewVertex(new_vertex.node1.id, new_vertex.node2.id)
                    }
                    Response::None
                }
                _ => Response::None,
            },
        }
    }

    pub fn follow_mouse(&mut self, event: &egui::Event) {
        match event {
            egui::Event::PointerMoved(pos) => {
                self.center = *pos;
            }
            _ => {}
        }
    }

    pub fn draw(&self, ui: &mut egui::Ui, stroke: egui::Stroke) {
        ui.painter().circle_stroke(self.center, self.radius, stroke);
        ui.painter()
            .circle_filled(self.center, self.radius, egui::Color32::WHITE);
    }
}
