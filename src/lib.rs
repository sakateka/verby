#![warn(clippy::all, rust_2018_idioms)]

mod app;
pub use app::Verby;

// Android entry point
#[cfg(target_os = "android")]
#[no_mangle]
fn android_main(app: winit::platform::android::activity::AndroidApp) {
    // Log to android output
    android_logger::init_once(
        android_logger::Config::default().with_max_level(log::LevelFilter::Info),
    );

    let options = eframe::NativeOptions {
        android_app: Some(app),
        persist_window: true,
        // Try Android cache directory instead of files directory
        persistence_path: Some(std::path::PathBuf::from(
            "/data/data/com.example.verby/files/verby_data",
        )),
        ..Default::default()
    };
    eframe::run_native(
        "Verby",
        options,
        Box::new(|cc| Ok(Box::new(crate::Verby::new(cc)))),
    )
    .unwrap()
}
