use gtk::gio;

fn main() {
    gio::compile_resources(
        "resources",
        "resources/resources.gresource.xml",
        "todo_1.gresource"
    );
}