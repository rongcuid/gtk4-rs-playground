use custom_button::CustomButton;
use gtk::glib::BindingFlags;
use gtk::subclass::prelude::*;
use gtk::{glib, prelude::*, Align, Orientation, gio};
use gtk::{Application, ApplicationWindow, Button};
use window::Window;
const APP_ID: &str = "org.gtk_rs.HelloWorld3";

mod custom_button;
mod window;
mod task_object;
mod task_row;

fn main() {
    gio::resources_register_include!("todo_1.gresource").unwrap();

    let app = Application::builder()
    .application_id("org.gtk_rs.Todo1").build();
    app.connect_activate(build_ui);
    app.run();
}

fn build_ui(app: &Application) {
    let window = Window::new(app);
    window.present();
}