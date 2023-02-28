use crate::node;

#[derive(Debug, Default, Clone, serde::Deserialize, serde::Serialize)]
pub struct Graph {
    pub nodes: Vec<node::Node>,
    pub adjacencies: Vec<Vec<usize>>,
    pub stroke: egui::Stroke,
    max_id: usize,
}

impl Graph {
    pub fn new(positions: Vec<egui::Pos2>, adjacencies: Vec<Vec<usize>>, stroke: egui::Stroke) -> Self {
        let mut max_id: usize = 0;
        Self {
            nodes: positions
                .iter()
                .map(|&pos| {
                    max_id += 1;
                    node::pos2_to_node(max_id, pos, stroke)
                })
                .collect(),
            adjacencies,
            max_id: positions.len(),
            stroke,
        }
    }

    pub fn add_node(&mut self, pos: egui::Pos2) {
        self.max_id += 1;
        self.nodes.push(node::pos2_to_node(self.max_id, pos, self.stroke));
        self.adjacencies.push(Vec::new())
    }

    pub fn draw(&mut self, ui: &mut egui::Ui) {
        for (i, node) in self.nodes.iter().enumerate() {
            for j in self.adjacencies[i].iter() {
                ui.painter()
                    .line_segment([node.circle.center, self.nodes[*j].circle.center], self.stroke);
            }
        }
        for node in self.nodes.iter() {
            node.clone().draw(ui)
        }
    }
    pub fn handle_event(&mut self, event: &egui::Event) {
        for node in self.nodes.iter_mut() {
            node.handle_event(event);
        }
    }

    pub fn has_cycle(&self) -> bool {
        let mut visited = vec![false; self.nodes.len()];
        let mut stack = vec![false; self.nodes.len()];
        for i in 0..self.nodes.len() {
            if !visited[i] {
                if self.has_cycle_util(i, &mut visited, &mut stack) {
                    return true;
                }
            }
        }
        false
    }

    fn has_cycle_util(&self, node: usize, visited: &mut Vec<bool>, stack: &mut Vec<bool>) -> bool {
        visited[node] = true;
        stack[node] = true;
        for &adj in self.adjacencies[node].iter() {
            if !visited[adj] && self.has_cycle_util(adj, visited, stack) {
                return true;
            } else if stack[adj] {
                return true;
            }
        }
        stack[node] = false;
        false
    }



    // function that calculates the minimum covering tree
    // returns a graph with the same nodes
    // but only the edges that are in the tree
    pub fn covering_tree(&self, stroke: egui::Stroke) -> Graph {
        let mut graph = Graph::new(Vec::new(), Vec::new(), stroke);
        let mut visited = vec![false; self.nodes.len()];
        let mut vertex = Vec::<(usize, usize, f32)>::new();

        // we add every vertex of the graph in the vertex vector
        for i in 0..self.nodes.len() {
            for &adj in self.adjacencies[i].iter() {
                let dist = (self.nodes[i].circle.center - self.nodes[adj].circle.center).length();
                vertex.push((i, adj, dist));
            }
        }

        // we order the vertex vector by the distance between the nodes
        vertex.sort_by(|a, b| a.2.partial_cmp(&b.2).unwrap());

        // We add the edges in the graph in the order of the vertex vector
        // if the edge doesn't create a cycle we don't add it
        for (i, adj, _) in vertex {
            if !graph.has_cycle() {
                graph.adjacencies[i].push(adj);
                graph.adjacencies[adj].push(i);
            }
        }
        graph
    }
}
