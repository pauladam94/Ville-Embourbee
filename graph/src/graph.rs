use crate::node::{pos2_to_node, Node};
use crate::state::State;
use crate::vertex::{TemporaryVertex, Vertex};

#[derive(Debug, Clone, Default)]
pub struct Graph {
    pub nodes: Vec<Node>,
    pub adjacencies: Vec<Vec<usize>>,
    vertices: Vec<Vertex>,
    max_id: usize,

    state: State,

    new_vertex: TemporaryVertex,

    new_node: Option<Node>,
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

            new_node: None,
        }
    }

    pub fn new_default_with_color(color: egui::Color32) -> Self {
        Graph::default().set_color_nodes(color).to_owned()
    }

    // SETTERS /////////////////////////////////////////////////////////////////////
    pub fn set_radius_nodes(&mut self, radius: f32) -> &mut Self {
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
        if let Some(node) = self.new_node.as_mut() {
            node.set_radius(radius);
        }
        self
    }

    pub fn set_stroke_nodes(&mut self, stroke: egui::Stroke) -> &mut Self {
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
        if let Some(node) = self.new_node.as_mut() {
            node.set_stroke(stroke);
        }
        self
    }

    pub fn set_stroke_vertex(&mut self, stroke: egui::Stroke) -> &mut Self {
        for vertex in self.vertices.iter_mut() {
            vertex.set_stroke(stroke);
        }
        self
    }

    pub fn set_width_nodes(&mut self, width: f32) -> &mut Self {
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
        if let Some(node) = self.new_node.as_mut() {
            node.set_width(width);
        }
        self
    }

    pub fn set_width_vertex(&mut self, width: f32) -> &mut Self {
        for vertex in self.vertices.iter_mut() {
            vertex.set_width(width);
        }
        // set width temporary vertex
        self.new_vertex.set_width(width);

        self
    }

    pub fn set_textures_vertex(&mut self, textures_id: Vec<egui::TextureId>) -> &mut Self {
        for vertex in self.vertices.iter_mut() {
            vertex.set_textures(textures_id.clone());
        }
        self
    }

    pub fn set_textures_nodes(
        &mut self,
        width_image: f32,
        textures_id: Vec<egui::TextureId>,
        size: Vec<egui::Vec2>,
    ) -> &mut Self {
        for i in 0..self.nodes.len() {
            let alpha: f32 = width_image / size[i % textures_id.len()].x;
            let pos_node = self.nodes[i].pos();
            self.nodes[i].set_drawable_image(
                pos_node,
                size[i % textures_id.len()] * alpha,
                textures_id[i % textures_id.len()],
            );
        }
        self
    }

    pub fn set_width_cobblestone_vertex(&mut self, width: f32) -> &mut Self {
        for vertex in self.vertices.iter_mut() {
            vertex.set_width_cobblestone(width);
        }
        self
    }

    pub fn set_color_nodes(&mut self, color: egui::Color32) -> &mut Self {
        for node in self.nodes.iter_mut() {
            node.set_color(color);
        }
        // set color of the new vertex
        if let Some(node) = self.new_vertex.first_mut() {
            node.set_color(color);
        }
        if let Some(node) = self.new_vertex.second_mut() {
            node.set_color(color);
        }
        if let Some(node) = self.new_node.as_mut() {
            node.set_color(color);
        }
        self
    }

    pub fn set_color_vertex(&mut self, color: egui::Color32) -> &mut Self {
        for vertex in self.vertices.iter_mut() {
            vertex.set_color(color);
        }
        self
    }

    // DRAW /////////////////////////////////////////////////////////////////////
    pub fn draw(&mut self, ui: &mut egui::Ui) {
        // draw every vertex
        for vertex in self.vertices.iter() {
            vertex.draw(
                ui,
                self.nodes[vertex.node_id1()].pos(),
                self.nodes[vertex.node_id2()].pos(),
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

        if let Some(node) = self.new_node {
            node.draw(ui);
        }
    }
    pub fn update(&mut self, event: &egui::Event) {
        // TODO: Refactor to not loop over the nodes at the begginning
        // every time we clicck, it clicks two times => draw 2 nodes for nothing
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
                            node.set_is_dragging(true);
                            node.set_drag_start(*pos);
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
                            node.set_drag_start(*pos);
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
                        if node.is_dragging() {
                            let delta = *pos - node.drag_start();
                            node.set_pos(node.pos() + delta);
                            node.set_drag_start(*pos);
                        }
                    }
                    egui::Event::PointerButton {
                        button: egui::PointerButton::Primary,
                        pressed: false,
                        ..
                    } => {
                        if node.is_dragging() {
                            node.set_is_dragging(false);
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

                            Graph::add_rm_edge_(
                                self.new_vertex.node_selected_id1.unwrap(),
                                self.new_vertex.node_selected_id2.unwrap(),
                                &mut self.adjacencies,
                                &mut self.vertices,
                            );

                            // reset the new vertex
                            self.new_vertex = TemporaryVertex::default();
                        }
                    }
                    _ => {}
                },
                _ => {}
            }
        }

        // Update new vertex
        if let Some(node2) = self.new_vertex.first_mut() {
            node2.follow_mouse(event);
        }
        // Update new node
        if let Some(node) = self.new_node.as_mut() {
            node.follow_mouse(event);
            if let egui::Event::Key {
                key: egui::Key::A, ..
            } = event
            {
                self.add_node(
                    self.new_node.unwrap().pos(),
                    egui::Stroke::new(2.0, egui::Color32::GREEN),
                );
            }
        }
    }

    // UI ///////////////////////////////////////////////////////////////////

    /// Ui to add a new node
    pub fn add_node_ui(&mut self, ui: &mut egui::Ui) {
        match self.state {
            State::AddNode => {
                if ui.button("Stop Adding New Nodes").clicked() {
                    self.end_add_node_ui();
                }
            }
            _ => {
                if ui.button("Add New Node").clicked() {
                    self.begin_add_node_ui();
                }
            }
        }
    }

    pub fn begin_add_node_ui(&mut self) {
        self.new_node = Some(Node::new_circle_node(
            None,
            egui::Pos2::new(200.0, 400.0),
            egui::Stroke::new(2.0, egui::Color32::BLUE),
        ));
        self.state = State::AddNode;
    }

    pub fn end_add_node_ui(&mut self) {
        self.new_node = None;
        self.state = State::Idle;
    }

    /// Debug Text for the graph
    pub fn debug(&self, ui: &mut egui::Ui) {
        ui.label(format!("Current State: {}", self.state));
        ui.label(format!("Number of nodes {}", self.nodes.len()));

        // affiche les id des nodes de new_vertex
        ui.label(format!("New Vertex: {}", self.new_vertex,));

        ui.label(format!("{self}"));
    }

    /// Add a node at a certain position to the graph
    pub fn add_node(&mut self, pos: egui::Pos2, stroke: egui::Stroke) {
        self.nodes
            .push(pos2_to_node(self.max_id, pos, Some(stroke)));
        self.max_id += 1;
        self.adjacencies.push(Vec::new());
    }

    /// Add an edge between two nodes
    /// borrow the graph (when you have ownership of the graph)
    pub fn add_edge(&mut self, id1: usize, id2: usize) {
        Self::add_edge_(id1, id2, &mut self.adjacencies, &mut self.vertices);
    }

    /// Add an edge between two nodes
    /// borrow only the adjacencies and vertices
    pub fn add_edge_(
        id1: usize,
        id2: usize,
        adjacencies: &mut [Vec<usize>],
        vertices: &mut Vec<Vertex>,
    ) {
        if !adjacencies[id1].contains(&id2) && !adjacencies[id2].contains(&id1) && id1 != id2 {
            adjacencies[id1].push(id2);
            adjacencies[id2].push(id1);
            vertices.push(Vertex::new(id1, id2, None));
        }
    }

    /// Add an edge between two nodes, if the edge already exists, remove it
    pub fn add_rm_edge(&mut self, id1: usize, id2: usize) {
        Self::add_rm_edge_(id1, id2, &mut self.adjacencies, &mut self.vertices);
    }

    pub fn add_rm_edge_(
        id1: usize,
        id2: usize,
        adjacencies: &mut [Vec<usize>],
        vertices: &mut Vec<Vertex>,
    ) {
        if adjacencies[id1].contains(&id2) && adjacencies[id2].contains(&id1) {
            // remove the edge
            Self::rm_edge_(id1, id2, adjacencies, vertices);
        } else {
            // add the edge
            adjacencies[id1].push(id2);
            adjacencies[id2].push(id1);
            vertices.push(Vertex::new(id1, id2, None));
        }
    }

    // add every edge possible to the graph
    pub fn add_every_edge(&mut self) {
        for i in 0..self.nodes.len() {
            for j in 0..self.nodes.len() {
                if i != j {
                    self.add_edge(i, j);
                }
            }
        }
    }

    pub fn rm_edge(&mut self, id1: usize, id2: usize) {
        self.adjacencies[id1].retain(|&x| x != id2);
        self.adjacencies[id2].retain(|&x| x != id1);
        self.vertices.retain(|x| {
            (x.node_id1() != id1 || x.node_id2() != id2)
                & (x.node_id1() != id2 || x.node_id2() != id1)
        });
    }

    pub fn rm_edge_(
        id1: usize,
        id2: usize,
        adjacencies: &mut [Vec<usize>],
        vertices: &mut Vec<Vertex>,
    ) {
        adjacencies[id1].retain(|&x| x != id2);
        adjacencies[id2].retain(|&x| x != id1);
        vertices.retain(|x| {
            (x.node_id1() != id1 || x.node_id2() != id2)
                & (x.node_id1() != id2 || x.node_id2() != id1)
        });
    }

    /// Checks if the graph has a cycle
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
        while let Some((last, current)) = queue.pop() {
            if visited[current] {
                // Already visited the node => there is a cycle
                return true;
            } else {
                // Mark the current node visited
                visited[current] = true;
                // Add its neighbours to the queue with the exception of the node we came from
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
        let mut vertices_ordered = Vec::<(usize, usize, f32)>::new();

        // we add every vertex of the graph in the vertex vector
        for vertex in self.vertices.iter() {
            vertices_ordered.push((
                vertex.node_id1(),
                vertex.node_id2(),
                self.nodes[vertex.node_id1()]
                    .pos()
                    .distance(self.nodes[vertex.node_id2()].pos()),
            ));
        }

        // we order the vertex vector by the distance between the nodes
        if min_covering_tree_algo {
            vertices_ordered
                .sort_by(|(_, _, w1), (_, _, w2)| w1.partial_cmp(w2).unwrap().reverse());
        } else {
            vertices_ordered.sort_by(|(_, _, w1), (_, _, w2)| w1.partial_cmp(w2).unwrap());
        }

        // We add the edges in the graph in the order of the weight of every vertex
        // if the edge doesn't create a cycle we don't add it
        for (i, adj, _) in vertices_ordered {
            graph.add_edge(i, adj);
            if graph.has_cycle() {
                graph.rm_edge(i, adj);
            }
        }
        graph
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

            new_node: None,
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

            new_node: None,
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
