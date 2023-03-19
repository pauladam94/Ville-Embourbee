#[derive(Debug, Clone, Copy)]
pub struct Image {
    pub pos: egui::Pos2,
    image: egui::widgets::Image,
}

impl Image {
    pub fn new(pos: egui::Pos2, image: egui::widgets::Image) -> Self {
        Self { pos, image }
    }

    pub fn contains(&self, point: egui::Pos2) -> bool {
        // TODO: check if the point is inside the image
        self.image.size();

        true
    }

    pub fn follow_mouse(&mut self, event: &egui::Event) {
        if let egui::Event::PointerMoved(pos) = event {
            self.pos = *pos;
        }
    }

    pub fn rect(&self) -> egui::Rect {
        egui::Rect::from_min_size(self.pos, self.image.size())
    }

    pub fn draw(&self, ui: &mut egui::Ui, stroke: egui::Stroke) {
        self.image.paint_at(ui, self.rect());
    }
}
