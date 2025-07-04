#![warn(clippy::all, rust_2018_idioms)]
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release

// When compiling natively (but not Android):
#[cfg(not(target_os = "android"))]
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

// For Android, we need a stub main function to satisfy the compiler
#[cfg(target_os = "android")]
fn main() {
    // Android uses android_main in lib.rs, this is just a stub
}
