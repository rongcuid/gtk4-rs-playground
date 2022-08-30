use custom_button::CustomButton;
use gtk::glib::BindingFlags;
use gtk::subclass::prelude::*;
use gtk::{glib, prelude::*, Align, Orientation};
use gtk::{Application, ApplicationWindow, Button};
const APP_ID: &str = "org.gtk_rs.HelloWorld3";

mod custom_button;

fn main() {
    // Create a new application
    let app = Application::builder().application_id(APP_ID).build();

    // Connect to "activate" signal of `app`
    app.connect_activate(build_ui);

    // Run the application
    app.run();
}

fn build_ui(app: &Application) {
    // Create a button with label and margins
    let button_1 = CustomButton::new();
    let button_2 = CustomButton::new();

    button_1
        .bind_property("number", &button_2, "number")
        .transform_to(|_, value| {
            let number = value.get::<i32>().unwrap();
            let incremented_number = number + 1;
            Some(incremented_number.to_value())
        })
        .transform_from(|_, value| {
            let number = value.get::<i32>().unwrap();
            let decremented_number = number - 1;
            Some(decremented_number.to_value())
        })
        .flags(BindingFlags::BIDIRECTIONAL | BindingFlags::SYNC_CREATE)
        .build();

    // Set up box
    let gtk_box = gtk::Box::builder()
        .margin_top(12)
        .margin_bottom(12)
        .margin_start(12)
        .margin_end(12)
        .valign(Align::Center)
        .halign(Align::Center)
        .spacing(12)
        .orientation(Orientation::Vertical)
        .build();
    gtk_box.append(&button_1);
    gtk_box.append(&button_2);

    // Create a window
    let window = ApplicationWindow::builder()
        .application(app)
        .title("My GTK App")
        .child(&gtk_box)
        .build();

    // Present window
    window.present();
}
