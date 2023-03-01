use crate::node;
use crate::response::Response;
use crate::state::State;
use crate::vertex::Vertex;

#[derive(Debug, Default, Clone, serde::Deserialize, serde::Serialize)]
pub struct Graph {
    pub nodes: Vec<node::Node>,
    pub adjacencies: Vec<Vec<usize>>,
    max_id: usize,
}

impl Graph {
    pub fn new(positions: Vec<egui::Pos2>, adjacencies: Vec<Vec<usize>>) -> Self {
        let mut max_id: usize = 0;
        let result = Self {
            nodes: positions
                .iter()
                .map(|&pos| {
                    let result = node::pos2_to_node(max_id, pos);
                    max_id += 1;
                    result
                })
                .collect(),
            adjacencies,
            max_id: positions.len(),
        };
        if result.adjacencies.len() != positions.len() {
            println!("{}", result);
            panic!("The number of nodes and the number of adjacency lists must be the same");
        }
        result
    }

    pub fn add_node(&mut self, pos: egui::Pos2) {
        self.nodes.push(node::pos2_to_node(self.max_id, pos));
        self.max_id += 1;
        self.adjacencies.push(Vec::new());
    }

    pub fn add_edge(&mut self, id1: usize, id2: usize) {
        if !self.adjacencies[id1].contains(&id2)
            && !self.adjacencies[id2].contains(&id1)
            && id1 != id2
        {
            self.adjacencies[id1].push(id2);
            self.adjacencies[id2].push(id1);
        }
    }

    pub fn draw(&mut self, ui: &mut egui::Ui, stroke_line: egui::Stroke, stroke_node: egui::Stroke) {
        for (i, node) in self.nodes.iter().enumerate() {
            for j in self.adjacencies[i].iter() {
                ui.painter()
                    .line_segment([node.circle.center, self.nodes[*j].circle.center], stroke_line);
            }
        }
        for node in self.nodes.iter() {
            node.clone().draw(ui, stroke_node)
        }
    }
    pub fn handle_event(
        &mut self,
        event: &egui::Event,
        state: &mut State,
        new_vertex: &mut Vertex,
    ) -> Response {
        let mut result = Response::None;
        let mut result_prio = Response::None;
        for node in self.nodes.iter_mut() {
            match node.handle_event_graph(event, state, new_vertex) {
                Response::NewVertex(id1, id2) => {
                    result_prio = Response::NewVertex(id1, id2);
                }
                Response::RightClicked => {
                    result = Response::RightClicked;
                }
                Response::Dragging => {
                    result = Response::Dragging;
                }
                Response::None => {}
            };
        }
        if let Response::NewVertex(_, _) = result_prio {
            result_prio
        } else {
            result
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
        let mut queue = vec![(start,start)];
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
                    if adj!=last {
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
    pub fn covering_tree(&self) -> Graph {
        let positions: Vec<egui::Pos2> = self.nodes.iter().map(|node| node.circle.center).collect();
        let mut graph = Graph::new(positions.clone(), vec![Vec::new(); positions.len()]);
        let mut vertex = Vec::<(usize, usize, f32)>::new();

        // we add every vertex of the graph in the vertex vector
        for i in 0..self.nodes.len() {
            for &adj in self.adjacencies[i].iter() {
                let dist = (self.nodes[i].circle.center - self.nodes[adj].circle.center).length();
                vertex.push((i, adj, dist));
            }
        }

        // we order the vertex vector by the distance between the nodes
        vertex.sort_by(|(_,_,w1), (_,_,w2)| w1.partial_cmp(w2).unwrap());

        // We add the edges in the graph in the order of the vertex vector
        // if the edge doesn't create a cycle we don't add it
        for (i, adj, _) in vertex {
            graph.adjacencies[i].push(adj);
            graph.adjacencies[adj].push(i);
            if graph.has_cycle() {
                graph.adjacencies[i].pop();
                graph.adjacencies[adj].pop();
            }
        }
        graph
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
}

impl std::fmt::Display for Graph {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Nodes : ")?;
        for node in self.nodes.iter() {
            write!(f, "| {} |", node)?;
        }
        writeln!(f, "")?;

        writeln!(f, "Adjacencies :")?;
        let mut i: usize = 0;
        for adj in self.adjacencies.iter() {
            write!(f, "\t {} -> ", i)?;
            i += 1;
            for j in adj.iter() {
                write!(f, "| {} |", j)?;
            }
            writeln!(f, "")?;
        }
        writeln!(f, "Max id : {}", self.max_id)?;
        Ok(())
    }
}
