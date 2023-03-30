use egui;
use egui_extras::RetainedImage;
use graph::graph::Graph;
use std::collections::HashMap;

pub struct App {
    graph: Graph,
    show_graph: bool,

    covering_tree: Graph,
    show_covering_tree: bool,
    min_covering_tree_algorithm: bool,

    width_node: f32,
    width_vertex: f32,
    node_radius: f32,
    width_image: f32,
    width_cobblestone: f32,

    show_ui: bool,

    dark_mode: bool,

    textures: HashMap<String, RetainedImage>,
}

impl Default for App {
    fn default() -> Self {
        let house1 = egui_extras::RetainedImage::from_image_bytes(
            "../data/house1.png",
            include_bytes!("../../data/house1.png"),
        )
        .unwrap();
        let house2 = egui_extras::RetainedImage::from_image_bytes(
            "../data/house2.png",
            include_bytes!("../../data/house2.png"),
        )
        .unwrap();
        let house3 = egui_extras::RetainedImage::from_image_bytes(
            "../data/house3.png",
            include_bytes!("../../data/house3.png"),
        )
        .unwrap();
        let house4 = egui_extras::RetainedImage::from_image_bytes(
            "../data/house4.png",
            include_bytes!("../../data/house4.png"),
        )
        .unwrap();
        let house5 = egui_extras::RetainedImage::from_image_bytes(
            "../data/house5.png",
            include_bytes!("../../data/house5.png"),
        )
        .unwrap();
        let cobblestone1 = egui_extras::RetainedImage::from_image_bytes(
            "../data/cobblestone1.png",
            include_bytes!("../../data/cobblestone1.png"),
        )
        .unwrap();
        let cobblestone2 = egui_extras::RetainedImage::from_image_bytes(
            "../data/cobblestone2.png",
            include_bytes!("../../data/cobblestone2.png"),
        )
        .unwrap();
        // TODO Change this in a hashmap
        let mut textures: HashMap<String, RetainedImage> = HashMap::new();
        textures.insert("house1".to_string(), house1);
        textures.insert("house2".to_string(), house2);
        textures.insert("house3".to_string(), house3);
        textures.insert("house4".to_string(), house4);
        textures.insert("house5".to_string(), house5);
        textures.insert("cobblestone1".to_string(), cobblestone1);
        textures.insert("cobblestone2".to_string(), cobblestone2);
        Self {
            graph: Graph::new_default_with_color(egui::Color32::GREEN),
            show_graph: true,

            covering_tree: Graph::new_default_with_color(egui::Color32::LIGHT_RED),
            show_covering_tree: false,

            width_node: 10.0,
            width_vertex: 4.0,
            node_radius: 20.0,
            width_image: 100.0,
            width_cobblestone: 50.0,

            min_covering_tree_algorithm: false,

            show_ui: true,

            dark_mode: true,

            textures,
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

    pub fn set_default(&mut self) {
        *self = Default::default();
    }
}

impl eframe::App for App {
    /// Called each time the UI needs repainting, which may be many times per second.
    /// Put your widgets into a `SidePanel`, `TopPanel`, `CentralPanel`, `Window` or `Area`.
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        let Self {
            graph,
            show_graph,

            covering_tree,
            show_covering_tree,

            width_node,
            width_vertex,
            node_radius,
            width_image,
            width_cobblestone,

            min_covering_tree_algorithm,

            show_ui,

            dark_mode,

            textures,
        } = self;

        //// update APP VALUE
        if *show_covering_tree {
            *covering_tree = graph.covering_tree(*min_covering_tree_algorithm);
        }

        // update the width of the stroke for the two graph
        // TODO DOES NOT WORK
        // graph_stroke_vertex.width = *width_vertex;
        // graph_stroke_node.width = *width_node;
        // covering_tree_stroke_vertex.width = *width_vertex;
        // covering_tree_stroke_node.width = *width_node;
        // new_node_stroke.width = *width_node;

        // every node is update with the same radius for the two graph
        graph
            .set_width_vertex(*width_vertex)
            .set_width_nodes(*width_node)
            .set_radius_nodes(*node_radius)
            .set_width_cobblestone_vertex(*width_cobblestone);

        covering_tree
            .set_width_vertex(*width_vertex)
            .set_width_nodes(*width_node)
            .set_radius_nodes(*node_radius)
            .set_width_cobblestone_vertex(*width_cobblestone);

        egui::CentralPanel::default().show(ctx, |ui| {
            // The central panel the region left after adding TopPanel's and SidePanel's
            egui::warn_if_debug_build(ui);

            // Draw the App
            if *show_graph {
                graph.draw(ui);
            }
            if *show_covering_tree {
                covering_tree.draw(ui);
            }

            // Handle graph events
            let events = ui.input(|i| i.clone().events);
            for event in events.iter() {
                if *show_graph || *show_covering_tree {
                    graph.update(event);
                }
            }
        });

        egui::Window::new("UI")
            .resizable(false)
            .collapsible(false)
            .show(ctx, |ui| {
                ui.toggle_value(show_ui, "Show UI");
            });

        if *show_ui {
            egui::TopBottomPanel::top("top_panel").show(ctx, |ui| {
            if ui.button("Reset Graph and State").clicked()
                || ui.input(|i| i.key_pressed(egui::Key::R))
            {
                *graph = Graph::default();
            }

            // button to change the theme of the app
            if ui.button("Change theme").clicked() {
                *dark_mode = !*dark_mode;
                if *dark_mode {
                    ctx.set_visuals(egui::Visuals::dark());
                } else {
                    ctx.set_visuals(egui::Visuals::light());
                }
            }

            ui.label("To add a node click the button add Node and then press A where you want to add the node");
            ui.label("To add an edge right click on a first edge then a second");
            ui.label("To move a node around left click on it");
            ui.label("To show the covering tree or the graph you can tick or untick the checkbox");
        });

            egui::SidePanel::left("side_panel").show(ctx, |ui| {
                ui.add(egui::Checkbox::new(
                    min_covering_tree_algorithm,
                    "min or max covering tree",
                ));

                if ui.button("Add every edge to the graph").clicked() {
                    graph.add_every_edge(egui::Stroke::new(1.0, egui::Color32::GREEN));
                }

                ui.separator();

                ui.add(egui::Checkbox::new(show_graph, "Show Graph"));
                ui.add(egui::Checkbox::new(
                    show_covering_tree,
                    "Show covering Tree",
                ));

                graph.add_node_ui(ui);

                ui.separator();

                ui.add(
                    egui::Slider::new(width_cobblestone, 10.0..=100.0).text("Width cobblestone"),
                );
                ui.add(egui::Slider::new(width_image, 10.0..=200.0).text("Width Images"));
                ui.add(egui::Slider::new(width_vertex, 0.0..=40.0).text("Width stroke vertex"));
                ui.add(egui::Slider::new(width_node, 0.0..=40.0).text("Width stroke node"));

                ui.add(egui::Slider::new(node_radius, 0.0..=40.0).text("Radius"));

                // Button to change the first node of the graph to the flower picture
                if ui.button("Change firsts nodes to images").clicked() {
                    graph.set_textures_nodes(
                        *width_image,
                        vec![
                            textures.get("house1").unwrap().texture_id(ctx),
                            textures.get("house2").unwrap().texture_id(ctx),
                            textures.get("house3").unwrap().texture_id(ctx),
                            textures.get("house4").unwrap().texture_id(ctx),
                            textures.get("house5").unwrap().texture_id(ctx),
                        ],
                        vec![
                            textures.get("house1").unwrap().size_vec2(),
                            textures.get("house2").unwrap().size_vec2(),
                            textures.get("house3").unwrap().size_vec2(),
                            textures.get("house4").unwrap().size_vec2(),
                            textures.get("house5").unwrap().size_vec2(),
                        ],
                    );
                }

                // Button to change the first node of the graph to the flower picture
                if ui.button("Change every Vertex to Cobblestone").clicked() {
                    graph.set_textures_vertex(vec![
                        textures.get("cobblestone1").unwrap().texture_id(ctx),
                        textures.get("cobblestone2").unwrap().texture_id(ctx),
                    ]);
                }

                // DEBUG to show the graph
                graph.debug(ui);
                covering_tree.debug(ui);
            });
        }
    }
}
