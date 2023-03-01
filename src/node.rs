use crate::circle_widget;
use crate::graph::Graph;
use crate::response::Response;
use crate::state::State;
use crate::vertex::Vertex;

#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
pub struct Node {
    pub id: usize,
    pub circle: circle_widget::CircleWidget,
}

impl Node {
    pub fn new(id: usize, circle: circle_widget::CircleWidget) -> Self {
        Self { id, circle }
    }

    pub fn handle_event(&mut self, event: &egui::Event, state: &mut State) {
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

    pub fn follow_mouse(&mut self, event: &egui::Event) {
        self.circle.follow_mouse(event);
    }

    pub fn draw(&mut self, ui: &mut egui::Ui, stroke: egui::Stroke) {
        self.circle.draw(ui, stroke);
    }
}

impl std::fmt::Display for Node {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.id)?;
        write!(f, " dragged : {}", self.circle.is_dragging)?;
        Ok(())
    }
}

pub fn pos2_to_node(id: usize, pos: egui::Pos2) -> Node {
    Node {
        id,
        circle: circle_widget::CircleWidget::new(pos),
    }
}
