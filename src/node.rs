use crate::circle_widget;

#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
pub struct Node {
    id: usize,
    pub circle: circle_widget::CircleWidget,
}

impl Node {
    pub fn new(id: usize, circle: circle_widget::CircleWidget) -> Self {
        Self { id, circle }
    }

    pub fn handle_event(&mut self, event: &egui::Event) {
        self.circle.handle_event(event);
    }

    pub fn draw(&mut self, ui: &mut egui::Ui) {
        self.circle.draw(ui);
    }
}

pub fn pos2_to_node(id: usize, pos: egui::Pos2, stroke: egui::Stroke) -> Node {
    Node {
        id,
        circle: circle_widget::CircleWidget::new(pos, stroke),
    }
}
