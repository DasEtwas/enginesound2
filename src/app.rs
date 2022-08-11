use eframe::egui::{Color32, Layout, Rounding, Visuals};
use eframe::{egui, epi};

#[derive(Default)]
pub struct App {
    visuals: Visuals,
}

impl App {
    pub fn new() -> Self {
        Self { ..Self::default() }
    }
}

impl epi::App for App {
    fn name(&self) -> &str {
        "enginesound2"
    }

    fn setup(
        &mut self,
        ctx: &egui::Context,
        _frame: &epi::Frame,
        _storage: Option<&dyn epi::Storage>,
    ) {
        let mut visuals = Visuals::dark();

        visuals.window_rounding = Rounding::same(1.0);
        visuals.faint_bg_color = Color32::from_gray(43);

        ctx.set_visuals(visuals.clone());

        self.visuals = visuals;
    }

    fn update(&mut self, ctx: &egui::Context, _frame: &epi::Frame) {
        egui::TopBottomPanel::top("top_panel").show(ctx, |ui| {
            // The top panel is often a good place for a menu bar:
            egui::menu::bar(ui, |ui| {
                ui.menu_button("File", |_ui| {});

                ui.with_layout(Layout::right_to_left(), |ui| {
                    if let Some(visuals) = self.visuals.light_dark_small_toggle_button(ui) {
                        ctx.set_visuals(visuals.clone());
                        self.visuals = visuals;
                    }
                });
            });
        });
    }
}
