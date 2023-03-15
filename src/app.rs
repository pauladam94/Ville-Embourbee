use crate::circle_widget::CircleWidget;
use crate::graph::Graph;
use crate::node::Node;
use crate::response::Response;
use crate::state::State;
use crate::vertex::Vertex;

/// We derive Deserialize/Serialize so we can persist app state on shutdown.
#[derive(serde::Deserialize, serde::Serialize)]
#[serde(default)] // if we add new fields, give them default values when deserializing old state
pub struct App {
    state: State,

    graph: Graph,
    show_graph: bool,
    graph_stroke_line: egui::Stroke,
    graph_stroke_node: egui::Stroke,

    covering_tree: Graph,
    show_covering_tree: bool,
    covering_tree_stroke_line: egui::Stroke,
    covering_tree_stroke_node: egui::Stroke,

    new_node: Node,
    show_new_node: bool,
    new_node_stroke: egui::Stroke,

    new_vertex: Vertex,
    show_new_vertex: bool,
    new_vertex_stroke: egui::Stroke,

    width_node: f32,
    width_line: f32,
    radius: f32,

    min_covering_tree_algorithm: bool,
}

impl Default for App {
    fn default() -> Self {
        Self {
            state: State::Idle,

            graph: Graph::new(
                vec![
                    egui::Pos2::new(300.0, 300.0),
                    egui::Pos2::new(500.0, 200.0),
                    egui::Pos2::new(700.0, 400.0),
                ],
                vec![vec![1, 2], vec![0, 2], vec![0, 1]],
            ),
            show_graph: true,
            graph_stroke_line: egui::Stroke::new(2.0, egui::Color32::GREEN),
            graph_stroke_node: egui::Stroke::new(2.0, egui::Color32::GREEN),

            covering_tree: Graph::new(
                vec![
                    egui::Pos2::new(300.0, 300.0),
                    egui::Pos2::new(500.0, 200.0),
                    egui::Pos2::new(700.0, 400.0),
                ],
                vec![vec![1, 2], vec![0, 2], vec![0, 1]],
            ),
            show_covering_tree: false,
            covering_tree_stroke_line: egui::Stroke::new(2.0, egui::Color32::BLUE),
            covering_tree_stroke_node: egui::Stroke::new(2.0, egui::Color32::BLUE),

            new_node: Node::new(0, CircleWidget::new(egui::Pos2::new(200.0, 400.0))),
            show_new_node: false,
            new_node_stroke: egui::Stroke::new(2.0, egui::Color32::LIGHT_BLUE),

            new_vertex: Vertex::new(egui::Pos2::new(200.0, 400.0), egui::Pos2::new(200.0, 400.0)),
            show_new_vertex: false,
            new_vertex_stroke: egui::Stroke::new(2.0, egui::Color32::YELLOW),

            width_node: 10.0,
            width_line: 4.0,
            radius: 20.0,
            min_covering_tree_algorithm: false,
        }
    }
}

/*
use egui::{Button, FontDefinitions, Pos2, Sense, TextStyle, Ui};
use printpdf::*;
use epaint::*;

fn pdf_button(ui: &mut Ui) {
    if ui.button("Save as PDF").clicked() {
        // Define the size of the content that you want to save as a PDF
        let content_size = Pos2::new(300.0, 200.0);

        // Create a canvas with the same size as the content
        let mut canvas = epaint::Canvas::new(content_size);

        // Draw the content onto the canvas
        canvas.fill(epaint::Color32::WHITE);
        canvas.text("Hello, world!", Pos2::new(50.0, 100.0));
        canvas.rectangle(
            epaint::Rect::from_min_max(Pos2::new(20.0, 20.0), Pos2::new(280.0, 180.0)),
            epaint::Color32::BLACK,
        );

        // Create a new PDF document
        let (doc, _, layer) =
            PdfDocument::new("My Document", canvas.width(), canvas.height(), "Layer 1");

        // Add a new page to the PDF document
        let mut graphics = layer.begin_page();

        // Convert the contents of the canvas to an image
        let image = Image::from_rgba8(canvas.width(), canvas.height(), canvas.pixels()).unwrap();

        // Add the image to the PDF document
        graphics.add_image(image, None, None);

        // Save the PDF document to a buffer
        let mut buf: Vec<u8> = Vec::new();
        doc.save(&mut buf).unwrap();

        // Save the buffer to a file
        std::fs::write("output.pdf", buf).unwrap();
    }
}


fn main() {
    let mut font_definitions = FontDefinitions::default();
    font_definitions.font_data.insert(
        "OpenSans-Regular.ttf".to_owned(),
        std::borrow::Cow::Borrowed(include_bytes!("OpenSans-Regular.ttf")),
    );
    let mut ctx = egui::CtxRef::default();
    ctx.set_fonts(font_definitions);
    loop {}
}
*/

impl App {
    /// Called once before the first frame.
    pub fn new(_cc: &eframe::CreationContext<'_>) -> Self {
        // This is also where you can customize the look and feel of egui using
        // `cc.egui_ctx.set_visuals` and `cc.egui_ctx.set_fonts`.

        // Load previous app state (if any).
        // Note that you must enable the `persistence` feature for this to work.
        // if let Some(storage) = cc.storage {
        //     return eframe::get_value(storage, eframe::APP_KEY).unwrap_or_default();
        //}
        Default::default()
    }

    pub fn set_default(&mut self) {
        *self = Default::default();
    }
}

impl eframe::App for App {
    /// Called by the frame work to save state before shutdown.
    fn save(&mut self, storage: &mut dyn eframe::Storage) {
        eframe::set_value(storage, eframe::APP_KEY, self);
    }

    /// Called each time the UI needs repainting, which may be many times per second.
    /// Put your widgets into a `SidePanel`, `TopPanel`, `CentralPanel`, `Window` or `Area`.
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        let Self {
            state,

            graph,
            show_graph,
            graph_stroke_line,
            graph_stroke_node,

            covering_tree,
            show_covering_tree,
            covering_tree_stroke_line,
            covering_tree_stroke_node,

            new_node,
            show_new_node,
            new_node_stroke,

            new_vertex,
            show_new_vertex,
            new_vertex_stroke,

            width_node,
            width_line,
            radius,

            min_covering_tree_algorithm,
        } = self;

        //// UPDATE APP VALUE
        if *show_covering_tree {
            *covering_tree = graph.covering_tree(*min_covering_tree_algorithm);
        }

        // update the width of the stroke for the two graph
        graph_stroke_line.width = *width_line;
        graph_stroke_node.width = *width_node;
        covering_tree_stroke_line.width = *width_line;
        covering_tree_stroke_node.width = *width_node;
        new_node_stroke.width = *width_node;
        new_vertex_stroke.width = *width_node;

        // every node is update with the same radius for the two graph
        for node in graph.nodes.iter_mut() {
            node.circle.radius = *radius;
        }
        for node in covering_tree.nodes.iter_mut() {
            node.circle.radius = *radius;
        }
        new_node.circle.radius = 1.2 * (*radius);
        new_vertex.node1.circle.radius = *radius;
        new_vertex.node2.circle.radius = *radius;

        if *state == State::RightClicked {
            *show_new_vertex = true;
        } else {
            *show_new_vertex = false;
        }

        egui::TopBottomPanel::top("top_panel").show(ctx, |ui| {
            // button to Reset the App
            if ui.button("Reset Graph and State").clicked()
                || ui.input(|i| i.key_pressed(egui::Key::R))
            {
                *graph = Graph::new(
                    vec![
                        egui::Pos2::new(200.0, 200.0),
                        egui::Pos2::new(500.0, 200.0),
                        egui::Pos2::new(700.0, 400.0),
                    ],
                    vec![vec![1, 2], vec![0, 2], vec![0, 1]],
                );
                *state = State::Idle;
            }

            ui.label("To add a node click the button add Node and then press A where you want to add the node");
            ui.label("To add an edge right click on a fist edge then a second");
            ui.label("To move a node around left click on it");
            ui.label("To show the covering tree or the graph you can tick or untick the checkbox");

        });

        egui::SidePanel::left("side_panel").show(ctx, |ui| {
            if ui.button("Add node (or press A)").clicked()
                || ui.input(|i| i.key_pressed(egui::Key::A))
            {
                graph.add_node(new_node.circle.center);
            }

            ui.add(egui::Checkbox::new(
                min_covering_tree_algorithm,
                "min or max covering tree",
            ));

            if ui.button("Add every edge to the graph").clicked() {
                graph.add_every_edge();
            }

            ui.add(egui::Checkbox::new(show_graph, "Show Graph"));
            ui.add(egui::Checkbox::new(
                show_covering_tree,
                "Show covering Tree",
            ));
            ui.add(egui::Checkbox::new(show_new_node, "Add New Node"));

            ui.add(egui::Slider::new(width_line, 0.0..=40.0).text("Width stroke line"));
            ui.add(egui::Slider::new(width_node, 0.0..=40.0).text("Width stroke node"));

            ui.add(egui::Slider::new(radius, 0.0..=40.0).text("Radius"));

            /* DEBUG to show the graph
            ui.label(format!("Current State: {}", state));
            ui.label(format!("Number of nodes {}", graph.nodes.len()));

            // affiche les id des nodes de new_vertex

            ui.label(format!(
                "New Vertex: {} {}",
                new_vertex.node1.id, new_vertex.node2.id
            ));

            ui.label(format!("{graph}"))
            */
        });

        egui::CentralPanel::default().show(ctx, |ui| {
            // The central panel the region left after adding TopPanel's and SidePanel's
            egui::warn_if_debug_build(ui);

            // Handle graph events
            let events = ui.input(|i| i.clone().events);
            for event in events.iter() {
                if *show_graph || *show_covering_tree {
                    if let Response::NewVertex(id1, id2) =
                        graph.handle_event(event, state, new_vertex)
                    {
                        graph.add_edge(id1, id2);
                    }
                }

                // TODO handle better the new_node
                // to be always on top of the hand (with a tick box to choose to
                // add a node or not)
                // new_node.handle_event(event, state);
                if *show_new_node {
                    new_node.follow_mouse(event);
                }

                if *show_new_vertex {
                    new_vertex.node2.follow_mouse(event);
                }

                // TODO handle better the new_vertex

                // new_vertex.node1.handle_event(event, state);
                // new_vertex.node2.handle_event(event, state);
            }

            // Draw the App
            if *show_graph {
                graph.draw(ui, *graph_stroke_line, *graph_stroke_node);
            }
            if *show_covering_tree {
                covering_tree.draw(ui, *covering_tree_stroke_line, *covering_tree_stroke_node);
            }
            if *show_new_node {
                new_node.draw(ui, *new_node_stroke);
            }
            if *show_new_vertex {
                new_vertex.draw(ui, *new_vertex_stroke);
            }
        });
    }
}
