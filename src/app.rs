use crate::gen::{
    Atmosphere, Cylinder, EngineState, GasMix, Generator, BAR, CCM, CELSIUS, MILLILITER,
};
use eframe::egui::{Color32, Layout, Rounding, Slider, Visuals};
use eframe::{egui, epi};
use egui::plot::{Line, Plot, Value, Values};
use std::f32::consts::PI;

pub struct App {
    visuals: Visuals,

    gen: Generator,

    steps: u32,
}

impl Default for App {
    fn default() -> Self {
        let atmosphere = Atmosphere {
            ambient_pressure: 101325.0,
            ambient_temperature: CELSIUS + 20.0,
            gas: GasMix {
                fuel: 0.0,
                neutral: 1.0 - 0.20946,
                oxidizer: 0.20946,
            } * 0.02896 /* kg/mol */
                * 1.204f32.recip(), /* kg/m³ */
        };
        let pos = -PI;
        Self {
            gen: Generator {
                rate: 80000.0,
                sample_rate: 48000,
                engine: EngineState {
                    position: pos,
                    cylinders: vec![Cylinder::new(
                        pos,
                        0.0,
                        10.0,
                        0.35,
                        50.0 * CCM,
                        0.025,
                        &atmosphere,
                    )],
                    speed: rpm_to_rad(300.0),
                },
                atmosphere,
            },
            visuals: Visuals::default(),
            steps: 100,
        }
    }
}
pub fn rpm_to_rad(rpm: f32) -> f32 {
    const RPM_TO_AV: f32 = std::f32::consts::PI / 30.0;
    rpm * RPM_TO_AV
}
pub fn rad_to_rpm(rad: f32) -> f32 {
    const AV_TO_RPM: f32 = 30.0 / std::f32::consts::PI;
    rad * AV_TO_RPM
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

            let prev_pos = self.gen.engine.position;

            ui.add(
                Slider::new(&mut self.steps, 100..=100000)
                    .step_by(1.0)
                    .clamp_to_range(true)
                    .logarithmic(true),
            );

            Plot::new("my_plot").view_aspect(2.0).show(ui, |plot| {
                let mut pressures = vec![];
                let mut volumes = vec![];
                let mut temperatures = vec![];

                for i in 0..self.steps {
                    //self.gen.rate as u32 / 4 {
                    self.gen.step();
                    pressures.push(Value::new(
                        i,
                        self.gen.engine.cylinders[0].cylinder.pressure / BAR,
                    ));
                    temperatures.push(Value::new(
                        i,
                        self.gen.engine.cylinders[0].cylinder.temperature,
                    ));
                }

                plot.line(Line::new(Values::from_values(pressures)));
                plot.line(Line::new(Values::from_values(volumes)));
                plot.line(Line::new(Values::from_values(temperatures)));
            });
            self.gen.engine.position = prev_pos;
        });
    }
}
