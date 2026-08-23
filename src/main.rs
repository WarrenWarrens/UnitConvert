
use eframe::egui;

#[derive(PartialEq, Clone)]
enum DistanceUnit {
    Meters,
    Kilometers,
    Miles,
    Feet,
}

impl std::fmt::Display for DistanceUnit {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            DistanceUnit::Meters => write!(f, "Meters"),
            DistanceUnit::Kilometers => write!(f, "Kilometers"),
            DistanceUnit::Miles => write!(f, "Miles"),
            DistanceUnit::Feet => write!(f, "Feet"),
        }
    }
}

struct ConverterApp {
    input_text: String,
    from_unit: DistanceUnit,
    to_unit: DistanceUnit,
}

impl Default for ConverterApp {
    fn default() -> Self {
        Self {
            input_text: String::new(),
            from_unit: DistanceUnit::Miles,
            to_unit: DistanceUnit::Kilometers,
        }
    }
}

impl eframe::App for ConverterApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("Distance Converter");
            ui.add_space(20.0);

            ui.horizontal(|ui| {
                ui.text_edit_singleline(&mut self.input_text);

                egui::ComboBox::from_id_source("from_unit_dropdown")
                    .selected_text(self.from_unit.to_string())
                    .show_ui(ui, |ui| {
                        ui.selectable_value(&mut self.from_unit, DistanceUnit::Meters, "Meters");
                        ui.selectable_value(&mut self.from_unit, DistanceUnit::Kilometers, "Kilometers");
                        ui.selectable_value(&mut self.from_unit, DistanceUnit::Miles, "Miles");
                        ui.selectable_value(&mut self.from_unit, DistanceUnit::Feet, "Feet");
                    });
            });

            ui.add_space(20.0);

            ui.horizontal(|ui| {

            });
        });
    }
}

fn main() {
    println!("Hello, world!");
}
