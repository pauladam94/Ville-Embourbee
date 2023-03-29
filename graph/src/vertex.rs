use crate::image::Image;
use crate::node::Node;

#[derive(Debug, Clone)]
enum VertexDrawable {
    Straight(egui::Stroke),
    Cobblestone(Vec<egui::TextureId>),
}

impl std::fmt::Display for VertexDrawable {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            VertexDrawable::Straight(stroke) => write!(f, "Straight({})", stroke.width)?,
            VertexDrawable::Cobblestone(images) => write!(f, "Cobblestone({})", images.len())?,
        };
        Ok(())
    }
}

#[derive(Debug, Clone)]
pub struct Vertex {
    pub node_id1: usize,
    pub node_id2: usize,
    drawable: VertexDrawable,
}

impl Vertex {
    // CONSTRUCTORS /////////////////////////////////////////////////////////////////
    pub fn new(node_id1: usize, node_id2: usize, stroke: Option<egui::Stroke>) -> Self {
        Self {
            node_id1,
            node_id2,
            drawable: VertexDrawable::Straight(
                stroke.unwrap_or(egui::Stroke::new(1.0, egui::Color32::GREEN)),
            ),
        }
    }

    pub fn new_straight_vertex(node_id1: usize, node_id2: usize, stroke: egui::Stroke) -> Self {
        Self {
            node_id1,
            node_id2,
            drawable: VertexDrawable::Straight(stroke),
        }
    }

    pub fn new_cobblestone_vertex(
        node_id1: usize,
        node_id2: usize,
        textures_id: Vec<egui::TextureId>,
    ) -> Self {
        Self {
            node_id1,
            node_id2,
            drawable: VertexDrawable::Cobblestone(textures_id),
        }
    }

    // SETTER //////////////////////////////////////////////////////////////////////
    pub fn set_stroke(&mut self, stroke: egui::Stroke) {
        match self.drawable {
            VertexDrawable::Straight(ref mut s) => *s = stroke,
            VertexDrawable::Cobblestone(_) => {}
        }
    }

    pub fn set_width(&mut self, width: f32) {
        match self.drawable {
            VertexDrawable::Straight(ref mut s) => s.width = width,
            VertexDrawable::Cobblestone(_) => {}
        }
    }

    pub fn set_textures(&mut self, textures_id: Vec<egui::TextureId>) {
        self.drawable = VertexDrawable::Cobblestone(textures_id);
    }

    /*
    pub fn update(&mut self, pos1: egui::Pos2, pos2: egui::Pos2) {
        match &mut self.drawable {
            VertexDrawable::Straight(_) => {}
            VertexDrawable::Cobblestone(images) => {
                let mut pos = pos1;
                let mut dir = pos2 - pos1;
                let mut dist = dir.length();
                dir = dir / dist;
                dist -= 20.0;
                pos += dir * 10.0;
                for image in images {
                    image.pos = pos;
                    pos += dir * 20.0;
                    dist -= 20.0;
                }
            }
        }
    }
    */

    pub fn draw(&self, ui: &mut egui::Ui, pos1: egui::Pos2, pos2: egui::Pos2) {
        match &self.drawable {
            VertexDrawable::Straight(stroke) => {
                ui.painter().line_segment([pos1, pos2], *stroke);
            }
            VertexDrawable::Cobblestone(textures_id) => {
                let y = 80.0;
                let x = 1.46 * y;

                let size = egui::Vec2::new(x, y);
                // We give 10 % of the length of the vertex to not draw the texture on the node
                let pourc_clean = 23. / 100.;
                /*
                let inv_length = 1. / (pos1 - pos2).length();
                let pourc_clean = if 200. * inv_length > 1. {
                    200. * inv_length
                } else {
                    1.
                };
                */
                let clean_pos1 = pos1 + (pos2 - pos1) * pourc_clean;
                let clean_pos2 = pos1 + (pos2 - pos1) * (1. - pourc_clean);

                let n_textures = textures_id.len();
                let n_cobblestone: i32 = std::cmp::max(
                    ((clean_pos1 - clean_pos2).length() / size.x).floor() as i32,
                    1,
                );
                let angle = (pos2 - pos1).angle();

                for i in 0..n_cobblestone + 1 {
                    let pos =
                        clean_pos1 + (clean_pos2 - clean_pos1) * (i as f32 / n_cobblestone as f32);
                    Image::new(pos, size, textures_id[(i % n_textures as i32) as usize]) //.draw(ui);
                        .draw_rotate_center(ui, angle);
                }
            }
        }
    }
}

impl std::fmt::Display for Vertex {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "({}, {}): {}",
            self.node_id1, self.node_id2, self.drawable
        )
    }
}

/// Temporary vertex used to draw a new vertex
/// in a graph without using a full Vertex
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
    pub fn first(&self) -> Option<&Node> {
        self.node1.as_ref()
    }

    pub fn first_mut(&mut self) -> Option<&mut Node> {
        self.node1.as_mut()
    }

    pub fn second(&self) -> Option<&Node> {
        self.node2.as_ref()
    }

    pub fn second_mut(&mut self) -> Option<&mut Node> {
        self.node2.as_mut()
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
        self.node1 = Some(Node::new_circle_node(Some(0), node.pos(), self.stroke));
    }

    pub fn select_second(&mut self, node: Node) {
        self.node_selected_id2 = Some(node.id());
        self.node2 = Some(Node::new_circle_node(Some(0), node.pos(), self.stroke));
    }

    pub fn draw(&self, ui: &mut egui::Ui) {
        ui.painter().line_segment(
            [
                self.node1.as_ref().unwrap().pos(),
                self.node2.as_ref().unwrap().pos(),
            ],
            self.stroke,
        );
        self.node1.as_ref().unwrap().draw(ui);
        self.node2.as_ref().unwrap().draw(ui);
    }
}

impl std::fmt::Display for TemporaryVertex {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "First node: ")?;
        // print "None" if node1 is None else print node1
        match &self.node1 {
            Some(node) => write!(f, "{} ", node)?,
            None => write!(f, "None ")?,
        }
        write!(f, "| Second node : ")?;
        // print "None" if node1 is None else print node1
        match &self.node1 {
            Some(node) => writeln!(f, "{}", node)?,
            None => writeln!(f, "None")?,
        }
        Ok(())
    }
}
