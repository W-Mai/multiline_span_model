use std::ops::RangeInclusive;

/// We derive Deserialize/Serialize so we can persist app state on shutdown.
#[derive(Default, Copy, Clone, serde::Deserialize, serde::Serialize)]
#[serde(default)]
pub struct SpanCord {
    start: [f64; 2],
    height: f64,
}

#[derive(serde::Deserialize, serde::Serialize)]
#[serde(default)] // if we add new fields, give them default values when deserializing old state
pub struct App {
    // Example stuff:
    label: String,

    #[serde(skip)] // This how you opt-out of serialization of a field
    value: f32,

    span_rect: SpanCord,
    span_group_width: f32,
}

impl Default for App {
    fn default() -> Self {
        Self {
            // Example stuff:
            label: "Hello World!".to_owned(),
            value: 2.7,

            span_rect: SpanCord {
                start: [20.0, 0.0],
                height: 10.0,
            },
            span_group_width: 100.0,
        }
    }
}

impl App {
    /// Called once before the first frame.
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        // This is also where you can customize the look and feel of egui using
        // `cc.egui_ctx.set_visuals` and `cc.egui_ctx.set_fonts`.

        // Load previous app state (if any).
        // Note that you must enable the `persistence` feature for this to work.
        if let Some(storage) = cc.storage {
            return eframe::get_value(storage, eframe::APP_KEY).unwrap_or_default();
        }

        Default::default()
    }
}

impl eframe::App for App {
    /// Called each time the UI needs repainting, which may be many times per second.
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        // Put your widgets into a `SidePanel`, `TopBottomPanel`, `CentralPanel`, `Window` or `Area`.
        // For inspiration and more examples, go to https://emilk.github.io/egui

        egui::TopBottomPanel::top("top_panel").show(ctx, |ui| {
            // The top panel is often a good place for a menu bar:

            egui::menu::bar(ui, |ui| {
                // NOTE: no File->Quit on web pages!
                let is_web = cfg!(target_arch = "wasm32");
                if !is_web {
                    ui.menu_button("File", |ui| {
                        if ui.button("Quit").clicked() {
                            ctx.send_viewport_cmd(egui::ViewportCommand::Close);
                        }
                    });
                    ui.add_space(16.0);
                }

                egui::widgets::global_dark_light_mode_buttons(ui);
            });
        });

        self.center_panel(ctx);
    }

    /// Called by the frame work to save state before shutdown.
    fn save(&mut self, storage: &mut dyn eframe::Storage) {
        eframe::set_value(storage, eframe::APP_KEY, self);
    }
}

impl App {
    fn center_panel(&mut self, ctx: &egui::Context) {
        egui::CentralPanel::default().show(ctx, |ui| {
            // The central panel the region left after adding TopPanel's and SidePanel's
            ui.heading("Multiline span model");

            ui.separator();

            let plot = egui_plot::Plot::new("my_plot")
                .data_aspect(1.0)
                .allow_zoom(true)
                .allow_drag(true)
                .allow_scroll(true)
                .allow_boxed_zoom(true)
                .allow_double_click_reset(true);

            let tmp_span_cord = SpanCord {
                start: [50.0, 50.0],
                height: 10.0,
            };
            println!("{:?}", self.span_rect.start);
            plot.show(ui, |plot_ui| {
                plot_ui.polygon(
                    egui_plot::Polygon::new(
                        egui_plot::PlotPoints::new(
                            vec![
                                self.span_rect.start,
                                [self.span_group_width as f64, self.span_rect.start[1]],
                                [self.span_group_width as f64, tmp_span_cord.start[1]],
                                tmp_span_cord.start,
                                [tmp_span_cord.start[0], tmp_span_cord.start[1] + tmp_span_cord.height],
                                [0.0, tmp_span_cord.start[1] + tmp_span_cord.height],
                                [0.0, self.span_rect.start[1] + self.span_rect.height],
                                [self.span_rect.start[0], self.span_rect.start[1] + self.span_rect.height],
                                self.span_rect.start,
                            ].iter().map(
                                |p| [p[0], -p[1]]
                            ).collect::<Vec<_>>()
                        )
                    )
                );
            });

            ui.with_layout(egui::Layout::bottom_up(egui::Align::LEFT), |ui| {
                powered_by_egui_and_eframe(ui);
                egui::warn_if_debug_build(ui);
            });
        });
    }
}

fn powered_by_egui_and_eframe(ui: &mut egui::Ui) {
    ui.horizontal(|ui| {
        ui.spacing_mut().item_spacing.x = 0.0;
        ui.label("Powered by ");
        ui.hyperlink_to("egui", "https://github.com/emilk/egui");
        ui.label(" and ");
        ui.hyperlink_to(
            "eframe",
            "https://github.com/emilk/egui/tree/master/crates/eframe",
        );
        ui.label(".");
    });
}