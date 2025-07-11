// Modules
mod app;

// Imports
use app::App;
use eframe::egui;

// Compiling for native
#[cfg(not(target_arch = "wasm32"))]
fn main() -> eframe::Result {
    let options = eframe::NativeOptions::default();
    eframe::run_native(
        "Window",
        options,
        Box::new(|cc| {
            // Setup App
            cc.egui_ctx.set_visuals(egui::Visuals::light());
            Ok(Box::new(App::default()))
        }),
    )
}

// Compiling for web
#[cfg(target_arch = "wasm32")]
fn main() {
    use eframe::wasm_bindgen::JsCast;
    let web_options = eframe::WebOptions::default();
    wasm_bindgen_futures::spawn_local(async {
        // Grab window
        let document = web_sys::window()
            .expect("No window")
            .document()
            .expect("No document");

        // Find canvas
        let canvas = document
            .get_element_by_id("canvas")
            .expect("Failed to find canvas")
            .dyn_into::<web_sys::HtmlCanvasElement>()
            .expect("canvas was not a HtmlCanvasElement");

        // Start GUI
        let start_result = eframe::WebRunner::new()
            .start(
                canvas,
                web_options,
                Box::new(|cc| {
                    // Setup App
                    cc.egui_ctx.set_visuals(egui::Visuals::light());
                    Ok(Box::new(App::default()))
                }),
            )
            .await;

        // Remove the loading text and spinner:
        if let Some(loading_text) = document.get_element_by_id("loading_text") {
            match start_result {
                Ok(_) => {
                    loading_text.remove();
                }
                Err(error) => {
                    loading_text.set_inner_html(
                        "<p>The app has crashed. See the developer console for details.</p>",
                    );
                    panic!("Failed to start eframe: {error:?}");
                }
            }
        }
    });
}
