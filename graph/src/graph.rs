use crate::node::{pos2_to_node, Node};
use crate::state::State;
use crate::vertex::{TemporaryVertex, Vertex};

/*
           new_vertex: Vertex::new(
               egui::Pos2::new(200.0, 400.0),
               egui::Pos2::new(200.0, 400.0),
               egui::Stroke::new(2.0, egui::Color32::YELLOW),
               egui::Stroke::new(2.0, egui::Color32::YELLOW),
           ),
           show_new_vertex: false,
           new_vertex_stroke: egui::Stroke::new(2.0, egui::Color32::YELLOW),
*/

#[derive(Debug, Clone, Default)]
pub struct Graph {
    pub nodes: Vec<Node>,
    pub adjacencies: Vec<Vec<usize>>,
    vertices: Vec<Vertex>,
    max_id: usize,

    state: State,

    new_vertex: TemporaryVertex,

    add_node: Option<Node>,
}

impl Graph {
    // CONSTRUCTOR /////////////////////////////////////////////////////////////////
    pub fn new(nodes: Vec<Node>, adjacencies: Vec<Vec<usize>>) -> Self {
        let max_id = nodes.len();
        let mut vertices: Vec<Vertex> = Vec::new();
        for (i, _) in nodes.iter().enumerate() {
            for j in adjacencies[i].iter() {
                if i < *j {
                    vertices.push(Vertex::new(i, *j, None));
                }
            }
        }

        Self {
            nodes,
            adjacencies,
            max_id,
            vertices,

            state: State::default(),

            new_vertex: TemporaryVertex::default(),

            add_node: None,
        }
    }

    // SETTERS /////////////////////////////////////////////////////////////////////
    pub fn set_radius_nodes(&mut self, radius: f32) {
        for node in self.nodes.iter_mut() {
            node.set_radius(radius);
        }
        // set radius of the new vertex
        if let Some(node) = self.new_vertex.first_mut() {
            node.set_radius(radius);
        }
        if let Some(node) = self.new_vertex.second_mut() {
            node.set_radius(radius);
        }
    }

    pub fn set_stroke_nodes(&mut self, stroke: egui::Stroke) {
        for node in self.nodes.iter_mut() {
            node.set_stroke(stroke);
        }
        // set stroke of the new vertex
        if let Some(node) = self.new_vertex.first_mut() {
            node.set_stroke(stroke);
        }
        if let Some(node) = self.new_vertex.second_mut() {
            node.set_stroke(stroke);
        }
    }

    pub fn set_stroke_vertex(&mut self, stroke: egui::Stroke) {
        for vertex in self.vertices.iter_mut() {
            vertex.set_stroke(stroke);
        }
    }

    pub fn set_width_nodes(&mut self, width: f32) {
        for node in self.nodes.iter_mut() {
            node.set_width(width);
        }
        // set width of the new vertex
        if let Some(node) = self.new_vertex.first_mut() {
            node.set_width(width);
        }
        if let Some(node) = self.new_vertex.second_mut() {
            node.set_width(width);
        }
    }

    pub fn set_width_vertex(&mut self, width: f32) {
        for vertex in self.vertices.iter_mut() {
            vertex.set_width(width);
        }
    }

    pub fn set_textures_vertex(&mut self, textures_id: Vec<egui::TextureId>) {
        for vertex in self.vertices.iter_mut() {
            vertex.set_textures(textures_id.clone());
        }
    }

    pub fn set_textures_nodes(&mut self, textures_id: Vec<egui::TextureId>, size: Vec<egui::Vec2>) {
        for (i, texture_id) in textures_id
            .iter()
            .enumerate()
            .take(std::cmp::min(self.nodes.len(), textures_id.len()))
        {
            let alpha: f32 = 200. / size[i].x;
            let pos_node = self.nodes[i].pos();
            self.nodes[i].set_drawable_image(pos_node, size[i] * alpha, *texture_id);
        }
    }

    // DRAW /////////////////////////////////////////////////////////////////////
    pub fn draw(&mut self, ui: &mut egui::Ui) {
        // draw every vertex
        for vertex in self.vertices.iter() {
            vertex.draw(
                ui,
                self.nodes[vertex.node_id1].pos(),
                self.nodes[vertex.node_id2].pos(),
            );
        }

        // draw every node
        for node in self.nodes.iter() {
            node.draw(ui);
        }

        // draw the new vertex
        if self.new_vertex.first_is_some() {
            self.new_vertex.draw(ui);
        }
    }
    pub fn update(&mut self, event: &egui::Event) {
        for node in self.nodes.iter_mut() {
            // move around the circle with the left click
            // the right click to choose the first node then the second the same way
            // it makes a vertex between them if they are not already connected
            // if they are already connected, it removes the vertex (TODO)

            match self.state {
                State::Idle => match event {
                    egui::Event::PointerButton {
                        pos,
                        button: egui::PointerButton::Primary,
                        pressed: true,
                        ..
                    } => {
                        if node.contains(*pos) {
                            node.is_dragging = true;
                            node.drag_start = *pos;
                            self.state = State::Dragging;
                        }
                    }
                    egui::Event::PointerButton {
                        pos,
                        button: egui::PointerButton::Secondary,
                        pressed: true,
                        ..
                    } => {
                        if node.contains(*pos) {
                            // self.is_right_clicked = true;
                            node.drag_start = *pos;
                            self.state = State::RightClicked;

                            // Select the first node
                            assert!(self.new_vertex.node_selected_id1.is_none());
                            self.new_vertex.select_first(node.to_owned());
                            assert!(self.new_vertex.node_selected_id1.is_some());
                            self.new_vertex.select_second(node.to_owned());
                            assert!(self.new_vertex.node_selected_id2.is_some());
                        }
                    }
                    _ => {}
                },
                State::Dragging => match event {
                    egui::Event::PointerMoved(pos) => {
                        if node.is_dragging {
                            let delta = *pos - node.drag_start;
                            node.set_pos(node.pos() + delta);
                            node.drag_start = *pos;
                        }
                    }
                    egui::Event::PointerButton {
                        button: egui::PointerButton::Primary,
                        pressed: false,
                        ..
                    } => {
                        if node.is_dragging {
                            node.is_dragging = false;
                            self.state = State::Idle;
                        }
                    }
                    _ => {}
                },
                State::RightClicked => match event {
                    egui::Event::PointerButton {
                        pos,
                        button: egui::PointerButton::Secondary,
                        pressed: true,
                        ..
                    } => {
                        if node.contains(*pos) {
                            self.state = State::Idle;

                            assert!(self.new_vertex.node_selected_id1.is_some());
                            self.new_vertex.select_second(node.to_owned());

                            // add the edge
                            // Cannot call add_edge because of borrowing
                            // REFACTORING
                            // TODO use the function add_edge rewrite it
                            // self.nodes;

                            Graph::add_edge_(
                                self.new_vertex.node_selected_id1.unwrap(),
                                self.new_vertex.node_selected_id2.unwrap(),
                                egui::Stroke::new(2.0, egui::Color32::GREEN),
                                &mut self.adjacencies,
                                &mut self.vertices,
                            );

                            // reset the new vertex
                            self.new_vertex = TemporaryVertex::default();
                        }
                    }
                    _ => {}
                },
            }
        }

        // Update new vertex
        if let Some(node2) = self.new_vertex.first_mut() {
            node2.follow_mouse(event);
        }

        // Update the vertices
        /*
        for vertex in self.vertices.iter_mut() {
            vertex.update(
                self.nodes[vertex.node_id1].pos(),
                self.nodes[vertex.node_id2].pos(),
            );
        }
        */
    }

    pub fn debug(&self, ui: &mut egui::Ui) {
        ui.label(format!("Current State: {}", self.state));
        ui.label(format!("Number of nodes {}", self.nodes.len()));

        // affiche les id des nodes de new_vertex

        ui.label(format!("New Vertex: {}", self.new_vertex,));

        ui.label(format!("{self}"));
    }

    pub fn add_node(&mut self, pos: egui::Pos2, stroke: egui::Stroke) {
        self.nodes
            .push(pos2_to_node(self.max_id, pos, Some(stroke)));
        self.max_id += 1;
        self.adjacencies.push(Vec::new());
    }

    pub fn add_edge(&mut self, id1: usize, id2: usize, stroke: egui::Stroke) {
        Graph::add_edge_(id1, id2, stroke, &mut self.adjacencies, &mut self.vertices);
    }

    pub fn add_edge_(
        id1: usize,
        id2: usize,
        stroke: egui::Stroke,
        adjacencies: &mut Vec<Vec<usize>>,
        vertices: &mut Vec<Vertex>,
    ) {
        if !adjacencies[id1].contains(&id2) && !adjacencies[id2].contains(&id1) && id1 != id2 {
            adjacencies[id1].push(id2);
            adjacencies[id2].push(id1);
            vertices.push(Vertex::new(id1, id2, Some(stroke)));
        }
    }

    // function that checks if the graph has a cycle
    pub fn has_cycle(&self) -> bool {
        for i in 0..self.nodes.len() {
            if self.has_cycle_from(i) {
                return true;
            }
        }
        false
    }

    pub fn has_cycle_from(&self, start: usize) -> bool {
        let mut visited = vec![false; self.nodes.len()];
        let mut queue = vec![(start, start)];
        while !queue.is_empty() {
            // pop the first element of the queue
            let (last, current) = queue.pop().unwrap();

            if visited[current] {
                // if we already visited the node, we have a cycle
                return true;
            } else {
                // otherwise we mark it as visited
                // add its neighbours to the queue with the exception of the node we came from
                visited[current] = true;
                for &adj in self.adjacencies[current].iter() {
                    if adj != last {
                        queue.push((current, adj));
                    }
                }
            }
        }
        false
    }

    // function that calculates the minimum covering tree
    // returns a graph with the same nodes
    // but only the edges that are in the tree
    pub fn covering_tree(&self, min_covering_tree_algo: bool) -> Graph {
        let mut graph = self.graph_without_edges();
        let mut vertex = Vec::<(usize, usize, f32)>::new();

        // we add every vertex of the graph in the vertex vector
        for i in 0..self.nodes.len() {
            for &adj in self.adjacencies[i].iter() {
                let dist = (self.nodes[i].pos() - self.nodes[adj].pos()).length();
                vertex.push((i, adj, dist));
            }
        }

        // we order the vertex vector by the distance between the nodes
        if min_covering_tree_algo {
            vertex.sort_by(|(_, _, w1), (_, _, w2)| w1.partial_cmp(w2).unwrap().reverse());
        } else {
            vertex.sort_by(|(_, _, w1), (_, _, w2)| w1.partial_cmp(w2).unwrap());
        }

        // We add the edges in the graph in the order of the weight of every vertex
        // if the edge doesn't create a cycle we don't add it
        for (i, adj, _) in vertex {
            if !graph.has_cycle() {
                graph.add_edge(i, adj, egui::Stroke::new(0.0, egui::Color32::RED));
            }
        }
        graph
    }

    // add every edge possible to the graph
    pub fn add_every_edge(&mut self) {
        for i in 0..self.nodes.len() {
            for j in 0..self.nodes.len() {
                if i != j {
                    self.add_edge(i, j, egui::Stroke::new(1.0, egui::Color32::RED));
                }
            }
        }
    }

    pub fn graph_without_edges(&self) -> Graph {
        let Graph { nodes, max_id, .. } = self;
        Graph {
            nodes: nodes.to_vec(),
            adjacencies: vec![Vec::new(); nodes.len()],
            max_id: *max_id,
            vertices: Vec::new(),

            new_vertex: TemporaryVertex::default(),

            state: State::Idle,

            add_node: None,
        }
    }

    pub fn pos2_to_graph(
        positions: Vec<egui::Pos2>,
        adjacencies: Vec<Vec<usize>>,
        node_stroke: egui::Stroke,
    ) -> Self {
        let mut max_id: usize = 0;
        let result = Self {
            nodes: positions
                .iter()
                .map(|&pos| {
                    let result = pos2_to_node(max_id, pos, Some(node_stroke));
                    max_id += 1;
                    result
                })
                .collect(),
            adjacencies,
            max_id: positions.len(),
            vertices: Vec::new(),

            state: State::default(),

            new_vertex: TemporaryVertex::default(),

            add_node: None,
        };
        if result.adjacencies.len() != positions.len() {
            println!("{result}");
            panic!("The number of nodes and the number of adjacency lists must be the same");
        }
        result
    }
}

impl std::fmt::Display for Graph {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Nodes: ")?;
        for node in self.nodes.iter() {
            write!(f, "| {node} |")?;
        }
        writeln!(f)?;

        writeln!(f, "Adjacencies:")?;

        for (i, adj) in self.adjacencies.iter().enumerate() {
            write!(f, "\t {i} -> ")?;
            for j in adj.iter() {
                write!(f, "| {j} |")?;
            }
            writeln!(f)?;
        }

        write!(f, "Vertices:")?;
        for vertex in self.vertices.iter() {
            write!(f, "\t {vertex}")?;
        }
        writeln!(f)?;
        writeln!(f, "Max id: {}", self.max_id)?;
        Ok(())
    }
}
