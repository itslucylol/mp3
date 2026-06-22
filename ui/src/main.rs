slint::include_modules!();

fn main() -> Result<(), slint::PlatformError> {
    let ui = MainWindow::new()?;

    // // Handle the button click
    // ui.on_button_clicked({
    //     let ui_handle = ui.as_weak();
    //     move || {
    //         let ui = ui_handle.unwrap();
    //         // Update the Slint property from Rust
    //         ui.set_my_text("Hello from Rust!".into());
    //     }
    // });

    ui.run()
}
