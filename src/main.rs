#![warn(clippy::all, rust_2018_idioms)]
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release

// When compiling natively (but not Android):
#[cfg(not(any(target_arch = "wasm32", target_os = "android")))]
fn main() -> eframe::Result<()> {
    env_logger::init(); // Log to stderr (if you run with `RUST_LOG=debug`).

    let native_options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([400.0, 300.0])
            .with_min_inner_size([300.0, 220.0])
            .with_icon(
                // NOTE: Adding an icon is optional
                eframe::icon_data::from_png_bytes(&include_bytes!("../assets/icon-256.png")[..])
                    .unwrap(),
            ),
        ..Default::default()
    };
    eframe::run_native(
        "verby",
        native_options,
        Box::new(|cc| Ok(Box::new(verby::Verby::new(cc)))),
    )
}

// When compiling to web using trunk:
#[cfg(target_arch = "wasm32")]
fn main() {
    use wasm_bindgen::JsCast;

    // Redirect `log` message to `console.log` and friends:
    eframe::WebLogger::init(log::LevelFilter::Debug).ok();

    let web_options = eframe::WebOptions::default();

    wasm_bindgen_futures::spawn_local(async {
        let start_result = eframe::WebRunner::new()
            .start(
                web_sys::window()
                    .and_then(|win| win.document())
                    .and_then(|doc| {
                        doc.get_element_by_id("the_canvas_id")
                            .map(|element| element.dyn_into::<web_sys::HtmlCanvasElement>())
                    })
                    .and_then(|canvas| canvas.ok())
                    .expect("hardcode it"),
                web_options,
                Box::new(|cc| Ok(Box::new(verby::Verby::new(cc)))),
            )
            .await;

        if let Err(e) = start_result {
            panic!("Failed to start eframe: {:?}", e);
        }
    });
}

// For Android, we need a stub main function to satisfy the compiler
#[cfg(target_os = "android")]
fn main() {
    // Android uses android_main in lib.rs, this is just a stub
}
