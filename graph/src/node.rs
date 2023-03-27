use crate::circle::Circle;
use crate::image::Image;
use crate::state::State;

#[derive(Debug, Clone, Copy)]
pub enum Drawables {
    Circle(Circle),
    Image(Image),
}

#[derive(Debug, Clone)]
pub struct Node {
    id: usize,
    pub is_dragging: bool,
    pub drag_start: egui::Pos2,
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

    pub fn new_circle_node(id: Option<usize>, pos: egui::Pos2, stroke: egui::Stroke) -> Self {
        Self {
            id: id.unwrap_or(0),
            is_dragging: false,
            drag_start: egui::Pos2::ZERO,
            drawable: Drawables::Circle(Circle::new(pos, stroke)),
        }
    }

    pub fn pos(&self) -> egui::Pos2 {
        match self.drawable {
            Drawables::Circle(circle) => circle.center(),
            // Choose if you want the center or the top left corner of the image
            Drawables::Image(image) => image.pos,
        }
    }

    pub fn id(&self) -> usize {
        self.id
    }

    pub fn set_pos(&mut self, pos: egui::Pos2) {
        match self.drawable {
            Drawables::Circle(ref mut circle) => circle.center = pos,
            Drawables::Image(ref mut image) => image.pos = pos,
        }
    }

    pub fn set_radius(&mut self, radius: f32) {
        match self.drawable {
            Drawables::Circle(ref mut circle) => circle.radius = radius,
            Drawables::Image(_) => {}
        }
    }

    pub fn set_drawable_image(
        &mut self,
        pos: egui::Pos2,
        size: egui::Vec2,
        texture_id: egui::TextureId,
    ) {
        self.drawable = Drawables::Image(Image::new(pos, size, texture_id));
    }

    pub fn contains(&self, pos: egui::Pos2) -> bool {
        match self.drawable {
            Drawables::Circle(circle) => circle.contains(pos),
            Drawables::Image(image) => image.contains(pos),
        }
    }

    pub fn handle_event(&mut self, event: &egui::Event, state: &mut State) {
        // move around the circle with the left click
        match state {
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
                        self.set_pos(self.pos() + delta);
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

    pub fn draw(&self, ui: &mut egui::Ui) {
        match self.drawable {
            Drawables::Circle(circle) => {
                circle.draw(ui);
            }
            Drawables::Image(image) => {
                image.draw(ui);
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

pub fn pos2_to_node(id: usize, pos: egui::Pos2, stroke: Option<egui::Stroke>) -> Node {
    Node {
        id,
        is_dragging: false,
        drag_start: egui::Pos2::ZERO,
        drawable: Drawables::Circle(Circle::new(
            pos,
            stroke.unwrap_or(egui::Stroke::new(1.0, egui::Color32::RED)),
        )),
    }
}
