#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
pub struct CircleWidget {
    pub center: egui::Pos2,
    radius: f32,
    stroke: egui::Stroke,
    is_dragging: bool,
    drag_start: egui::Pos2,
    is_right_clicked: bool,
}

impl CircleWidget {
    // add stroke in the parameters
    pub fn new(center: egui::Pos2, stroke: egui::Stroke) -> Self {
        Self {
            center,
            radius: 10.0,
            stroke,
            is_dragging: false,
            drag_start: egui::Pos2::ZERO,
            is_right_clicked: false,
        }
    }

    fn to_shape(&self) -> egui::Shape {
        egui::Shape::circle_stroke(self.center, self.radius, self.stroke)
    }

    fn contains_point(&self, point: egui::Pos2) -> bool {
        let delta = point - self.center;
        delta.length() < self.radius
    }

    pub fn handle_event(&mut self, event: &egui::Event) {
        match event {
            egui::Event::PointerMoved(pos) => {
                if self.is_dragging {
                    let delta = *pos - self.drag_start;
                    self.center += delta;
                    self.drag_start = *pos;
                }
            }
            egui::Event::PointerButton {
                pos,
                button: egui::PointerButton::Primary,
                pressed: true,
                modifiers,
            } => {
                if self.contains_point(*pos) && !modifiers.ctrl {
                    self.is_dragging = true;
                    self.drag_start = *pos;
                }
            }
            egui::Event::PointerButton {
                pos,
                button: egui::PointerButton::Secondary,
                pressed: true,
                ..
            } => {
                if self.contains_point(*pos) {
                    self.is_right_clicked = true;
                }
            }
            egui::Event::PointerButton {
                button: egui::PointerButton::Primary,
                pressed: false,
                ..
            } => {
                self.is_dragging = false;
            }
            _ => {}
        }
    }

    pub fn draw(&self, ui: &mut egui::Ui) {
        ui.painter().add(self.to_shape());
    }
}
