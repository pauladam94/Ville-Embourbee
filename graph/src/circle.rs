#[derive(Debug, Clone, Copy)]
pub struct Circle {
    center: egui::Pos2,
    radius: f32,
    stroke: egui::Stroke,
}

impl Circle {
    // CONSTRUCTOR /////////////////////////////////////////////////////////////////
    // add stroke in the parameters
    pub fn new(center: egui::Pos2, stroke: egui::Stroke) -> Self {
        Self {
            center,
            radius: 10.0,
            stroke,
        }
    }

    // SETTER //////////////////////////////////////////////////////////////////////
    pub fn set_radius(&mut self, radius: f32) {
        self.radius = radius;
    }

    pub fn set_stroke(&mut self, stroke: egui::Stroke) {
        self.stroke = stroke;
    }

    pub fn set_width(&mut self, width: f32) {
        self.stroke.width = width;
    }

    pub fn set_color(&mut self, color: egui::Color32) {
        self.stroke.color = color;
    }

    pub fn set_center(&mut self, center: egui::Pos2) {
        self.center = center;
    }

    /*
    |----------------|
    |                |
    |      Hey       |
    |                |
    |----------------|
    */

    // GETTERS //////////////////////////////////////////////////////////////////////
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

    pub fn draw(&self, ui: &mut egui::Ui) {
        ui.painter()
            .circle_stroke(self.center, self.radius, self.stroke);
        ui.painter()
            .circle_filled(self.center, self.radius, egui::Color32::WHITE);
    }
}
