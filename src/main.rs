use eframe::egui;

fn main() -> eframe::Result {
    let options = eframe::NativeOptions::default();
    eframe::run_native(
        "Window",
        options,
        Box::new(|cc| {
            // Setup App
            cc.egui_ctx.set_visuals(egui::Visuals::light());
            let app = App { age: 0 };
            Ok(Box::new(app))
        }),
    )
}

struct App {
    age: usize,
}

impl eframe::App for App {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("Window");
            ui.add(egui::Slider::new(&mut self.age, 0..=100).text("age"));
            if ui.button("Increment").clicked() {
                self.age += 1;
            }
        });
    }
}
