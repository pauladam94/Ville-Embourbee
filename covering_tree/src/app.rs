use egui;
use graph::graph::Graph;
use graph::node::Node;

pub struct App {
    graph: Graph,
    show_graph: bool,
    graph_stroke_vertex: egui::Stroke,
    graph_stroke_node: egui::Stroke,

    covering_tree: Graph,
    show_covering_tree: bool,
    covering_tree_stroke_vertex: egui::Stroke,
    covering_tree_stroke_node: egui::Stroke,

    new_node: Node,
    show_new_node: bool,
    new_node_stroke: egui::Stroke,

    width_node: f32,
    width_vertex: f32,
    radius: f32,

    min_covering_tree_algorithm: bool,

    show_ui: bool,

    dark_mode: bool,

    textures: Vec<egui_extras::RetainedImage>,
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
        let cobblestone = egui_extras::RetainedImage::from_image_bytes(
            "../data/house6.png",
            include_bytes!("../../data/cobblestone.png"),
        )
        .unwrap();
        let textures = vec![house1, house2, house3, house4, house5, cobblestone];
        Self {
            graph: Graph::default(),
            show_graph: true,
            graph_stroke_vertex: egui::Stroke::new(2.0, egui::Color32::GREEN),
            graph_stroke_node: egui::Stroke::new(2.0, egui::Color32::GREEN),

            covering_tree: Graph::default(),
            show_covering_tree: false,
            covering_tree_stroke_vertex: egui::Stroke::new(2.0, egui::Color32::BLUE),
            covering_tree_stroke_node: egui::Stroke::new(2.0, egui::Color32::BLUE),

            new_node: Node::new_circle_node(
                None,
                egui::Pos2::new(200.0, 400.0),
                egui::Stroke::new(2.0, egui::Color32::BLUE),
            ),
            show_new_node: false,
            new_node_stroke: egui::Stroke::new(2.0, egui::Color32::LIGHT_BLUE),

            width_node: 10.0,
            width_vertex: 4.0,
            radius: 20.0,

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
            graph_stroke_vertex,
            graph_stroke_node,

            covering_tree,
            show_covering_tree,
            covering_tree_stroke_vertex,
            covering_tree_stroke_node,

            new_node,
            show_new_node,
            new_node_stroke,

            width_node,
            width_vertex,
            radius,

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
        graph_stroke_vertex.width = *width_vertex;
        graph_stroke_node.width = *width_node;
        covering_tree_stroke_vertex.width = *width_vertex;
        covering_tree_stroke_node.width = *width_node;
        new_node_stroke.width = *width_node;

        // every node is update with the same radius for the two graph
        graph.set_radius_nodes(*radius);
        new_node.set_radius(*radius);

        egui::CentralPanel::default().show(ctx, |ui| {
            // The central panel the region left after adding TopPanel's and SidePanel's
            egui::warn_if_debug_build(ui);

            // Draw the App
            if *show_graph {
                graph.draw(ui, *graph_stroke_vertex);
            }
            if *show_covering_tree {
                covering_tree.draw(ui, *covering_tree_stroke_vertex);
            }
            if *show_new_node {
                new_node.draw(ui);
            }

            // Handle graph events
            let events = ui.input(|i| i.clone().events);
            for event in events.iter() {
                if *show_graph || *show_covering_tree {
                    graph.update(event);
                }
                if *show_new_node {
                    new_node.follow_mouse(event)
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
            ui.label("To add an edge right click on a fist edge then a second");
            ui.label("To move a node around left click on it");
            ui.label("To show the covering tree or the graph you can tick or untick the checkbox");
        });

            egui::SidePanel::left("side_panel").show(ctx, |ui| {
                if *show_new_node && ui.input(|i| i.key_pressed(egui::Key::A)) {
                    graph.add_node(new_node.pos(), egui::Stroke::new(2.0, egui::Color32::GREEN));
                }

                ui.add(egui::Checkbox::new(
                    min_covering_tree_algorithm,
                    "min or max covering tree",
                ));

                if ui.button("Add every edge to the graph").clicked() {
                    graph.add_every_edge();
                }

                ui.separator();

                ui.add(egui::Checkbox::new(show_graph, "Show Graph"));
                ui.add(egui::Checkbox::new(
                    show_covering_tree,
                    "Show covering Tree",
                ));

                ui.add(egui::Checkbox::new(show_new_node, "Add New Node"));

                ui.separator();

                ui.add(egui::Slider::new(width_vertex, 0.0..=40.0).text("Width stroke vertex"));
                ui.add(egui::Slider::new(width_node, 0.0..=40.0).text("Width stroke node"));

                ui.add(egui::Slider::new(radius, 0.0..=40.0).text("Radius"));

                // Button to change the first node of the graph to the flower picture
                if ui.button("Change first node to flower").clicked() {
                    // size of texture[0]

                    for (i, texture) in textures
                        .iter()
                        .enumerate()
                        .take(std::cmp::min(graph.nodes.len(), textures.len()))
                    {
                        let size = texture.size_vec2();
                        let alpha: f32 = 200. / size.x;
                        let pos_node = graph.nodes[i].pos();
                        graph.nodes[i].set_drawable_image(
                            pos_node,
                            size * alpha,
                            texture.texture_id(ctx),
                        );
                    }
                }

                // DEBUG to show the graph
                // graph.debug(ui);
            });
        }
    }
}
