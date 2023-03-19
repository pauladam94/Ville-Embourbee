#[derive(Debug, Clone, Copy, serde::Deserialize, serde::Serialize)]
pub struct Circle {
    pub center: egui::Pos2,
    pub radius: f32,
}

impl Circle {
    // add stroke in the parameters
    pub fn new(center: egui::Pos2) -> Self {
        Self {
            center,
            radius: 10.0,
        }
    }

    pub fn center(&self) -> egui::Pos2 {
        self.center
    }

    pub fn contains(&self, point: egui::Pos2) -> bool {
        let delta = point - self.center;
        delta.length() < self.radius
    }

    pub fn follow_mouse(&mut self, event: &egui::Event) {
        if let egui::Event::PointerMoved(pos) = event {
            self.center = *pos;
        }
    }

    pub fn draw(&self, ui: &mut egui::Ui, stroke: egui::Stroke) {
        ui.painter().circle_stroke(self.center, self.radius, stroke);
        ui.painter()
            .circle_filled(self.center, self.radius, egui::Color32::WHITE);
    }
}
