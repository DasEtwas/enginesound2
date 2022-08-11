#![forbid(unsafe_code)]
#![warn(clippy::all, rust_2018_idioms)]

// https://github.com/emilk/eframe_template/

fn main() {
    let app = enginesound2::App::default();
    let mut native_options = eframe::NativeOptions::default();
    native_options.drag_and_drop_support = true;
    eframe::run_native(Box::new(app), native_options);
}
