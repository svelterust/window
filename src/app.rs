use eframe::egui;

#[derive(Default)]
pub struct App {
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
