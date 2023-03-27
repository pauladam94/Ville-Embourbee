use crate::image::Image;
use crate::node::Node;

#[derive(Debug, Clone)]
enum VertexDrawable {
    Straight(egui::Stroke),
    Cobblestone(Image),
}

#[derive(Debug, Clone)]
pub struct Vertex {
    pub node_id1: usize,
    pub node_id2: usize,
    drawable: VertexDrawable,
}

impl Vertex {
    pub fn new_straight_vertex(node_id1: usize, node_id2: usize, stroke: egui::Stroke) -> Self {
        Self {
            node_id1,
            node_id2,
            drawable: VertexDrawable::Straight(stroke),
        }
    }

    pub fn new_cobblestone_vertex(node_id1: usize, node_id2: usize, image: Image) -> Self {
        Self {
            node_id1,
            node_id2,
            drawable: VertexDrawable::Cobblestone(image),
        }
    }
}

#[derive(Debug, Clone)]
pub struct TemporaryVertex {
    pub node_selected_id1: Option<usize>,
    pub node_selected_id2: Option<usize>,
    node1: Option<Node>,
    node2: Option<Node>,
    stroke: egui::Stroke,
}

impl Default for TemporaryVertex {
    fn default() -> Self {
        Self {
            node_selected_id1: None,
            node_selected_id2: None,
            node1: None,
            node2: None,
            stroke: egui::Stroke::new(2.0, egui::Color32::RED),
        }
    }
}

impl TemporaryVertex {
    pub fn first (&self) -> Option<&Node> {
        self.node1.as_ref()
    }

    pub fn first_mut (&mut self) -> Option<&mut Node> {
        self.node1.as_mut()
    }

    pub fn second (&self) -> Option<&Node> {
        self.node2.as_ref()
    }

    pub fn first_is_some(&self) -> bool {
        self.node1.is_some()
    }

    pub fn first_is_none(&self) -> bool {
        self.node1.is_none()
    }

    pub fn second_is_some(&self) -> bool {
        self.node2.is_some()
    }

    pub fn second_is_none(&self) -> bool {
        self.node2.is_none()
    }

    pub fn select_first(&mut self, node: Node) {
        self.node_selected_id1 = Some(node.id());
        self.node1.as_mut().unwrap().set_pos(node.pos());
    }

    pub fn select_second(&mut self, node: Node) {
        self.node_selected_id2 = Some(node.id());
        self.node2.as_mut().unwrap().set_pos(node.pos());
    }

    pub fn draw(&self, ui: &mut egui::Ui) {
        ui.painter().line_segment(
            [self.node1.as_ref().unwrap().pos(), self.node2.as_ref().unwrap().pos()],
            self.stroke,
        );
        self.node1.as_ref().unwrap().draw(ui);
        self.node2.as_ref().unwrap().draw(ui);
    }
}
