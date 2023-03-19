use crate::node;

#[derive(Debug, Clone)]
pub struct Vertex {
    pub node1: node::Node,
    pub node2: node::Node,
}

impl Vertex {
    pub fn new(pos1: egui::Pos2, pos2: egui::Pos2) -> Self {
        Self {
            node1: node::pos2_to_node(0, pos1),
            node2: node::pos2_to_node(1, pos2),
        }
    }

    pub fn draw(&mut self, ui: &mut egui::Ui, stroke: egui::Stroke) {
        ui.painter()
            .line_segment([self.node1.pos(), self.node2.pos()], stroke);
        self.node1.draw(ui, stroke);
        self.node2.draw(ui, stroke);
    }
}
