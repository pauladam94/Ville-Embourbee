use crate::circle_widget::CircleWidget;
use crate::graph::Graph;
use crate::node::Node;

use egui::{Color32, Pos2, Stroke};

/// We derive Deserialize/Serialize so we can persist app state on shutdown.
#[derive(serde::Deserialize, serde::Serialize)]
#[serde(default)] // if we add new fields, give them default values when deserializing old state
pub struct App {
    // Example stuff:
    show_graph: bool,
    graph: Graph,
    show_covering_tree: bool,
    covering_tree: Graph,
    new_node: Node,
}

impl Default for App {
    fn default() -> Self {
        Self {
            show_graph: true,
            graph: Graph::new(
                vec![
                    Pos2::new(200.0, 200.0),
                    Pos2::new(500.0, 200.0),
                    Pos2::new(700.0, 400.0),
                ],
                vec![vec![1, 2], vec![0, 2], vec![0, 1]],
                Stroke::new(1.0, Color32::WHITE),
            ),
            show_covering_tree: false,
            covering_tree: Graph::new(
                vec![
                    Pos2::new(200.0, 200.0),
                    Pos2::new(500.0, 200.0),
                    Pos2::new(700.0, 400.0),
                ],
                vec![vec![1, 2], vec![0, 2], vec![0, 1]],
                Stroke::new(3.0, Color32::RED),
            ),
            new_node: Node::new(
                1,
                CircleWidget::new(
                    Pos2::new(100.0, 100.0),
                    egui::Stroke::new(3.0, egui::Color32::GREEN),
                ),
            ),
        }
    }
}

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
            show_graph,
            graph,
            show_covering_tree,
            covering_tree,
            new_node,
        } = self;
        // covering tree is a copy of the graph

        *covering_tree = graph.covering_tree(covering_tree.stroke);
        egui::TopBottomPanel::top("top_panel").show(ctx, |ui| {
            // The top panel is often a good place for a menu bar:
            egui::menu::bar(ui, |_ui| {});
        });

        egui::SidePanel::left("side_panel").show(ctx, |ui| {
            if ui.button("Add node").clicked() || ui.input(|i| i.key_pressed(egui::Key::A)) {
                graph.add_node(new_node.circle.center);
            }

            ui.add(egui::Checkbox::new(show_graph, "Show Graph"));
            ui.add(egui::Checkbox::new(
                show_covering_tree,
                "Show covering Tree",
            ));
        });

        egui::CentralPanel::default().show(ctx, |ui| {
            // The central panel the region left after adding TopPanel's and SidePanel's
            egui::warn_if_debug_build(ui);

            // User Input

            // Handle graph events
            let events = ui.input(|i| i.clone().events);
            for event in events.iter() {
                graph.handle_event(event);
                new_node.handle_event(event);
            }

            // Draw the screen
            if *show_graph {
                graph.draw(ui);
            }
            if *show_covering_tree {
                covering_tree.draw(ui);
            }
            new_node.draw(ui);
        });
    }
}
