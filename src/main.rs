use std::thread;
use std::time::Duration;

// This macro includes the generated Rust code from your .slint file
slint::include_modules!();

fn main() -> Result<(), slint::PlatformError> {
    let ui = AppWindow::new()?;
    let ui_weak = ui.as_weak();

    // Spawn a background thread to handle the typing animation
    thread::spawn(move || {
        let full_text = "Hello World!";
        let mut current_text = String::new();

        // Optional: Wait a moment for the window to actually open before typing
        thread::sleep(Duration::from_millis(500));

        for ch in full_text.chars() {
            current_text.push(ch);
            
            // Clone the weak reference for the event loop closure
            let ui_clone = ui_weak.clone();
            let text_to_send = current_text.clone();

            // Safely update the UI from the background thread
            let _ = slint::invoke_from_event_loop(move || {
                if let Some(ui) = ui_clone.upgrade() {
                    ui.set_message(text_to_send.into());
                }
            });

            // Control the typing speed (150 milliseconds per letter)
            thread::sleep(Duration::from_millis(150));
        }
    });

    // Run the main event loop (this blocks the main thread)
    ui.run()
}