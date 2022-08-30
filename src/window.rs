use gtk::gio;
use gtk::glib;
use gtk::glib::clone;
use gtk::glib::Object;
use gtk::prelude::*;
use gtk::subclass::prelude::ObjectSubclassIsExt;
use gtk::Application;
use gtk::NoSelection;
use gtk::SignalListItemFactory;

use crate::task_object::TaskObject;
use crate::task_row::TaskRow;

glib::wrapper! {
    pub struct Window(ObjectSubclass<imp::Window>)
        @extends gtk::ApplicationWindow, gtk::Window, gtk::Widget,
        @implements gio::ActionGroup, gio::ActionMap, gtk::Accessible, gtk::Buildable,
        gtk::ConstraintTarget, gtk::Native, gtk::Root, gtk::ShortcutManager;
}

impl Window {
    pub fn new(app: &Application) -> Self {
        Object::new(&[("application", app)]).unwrap()
    }
    fn tasks(&self) -> gio::ListStore {
        self.imp().tasks.borrow().clone().unwrap()
    }
    fn setup_tasks(&self) {
        let model = gio::ListStore::new(TaskObject::static_type());
        self.imp().tasks.replace(Some(model));
        let selection_model = NoSelection::new(Some(&self.tasks()));
        self.imp().tasks_list.set_model(Some(&selection_model));
    }
    fn setup_callbacks(&self) {
        self.imp()
            .entry
            .connect_activate(clone!(@weak self as window => move |_| {
                window.new_task();
            }));
        self.imp()
            .entry
            .connect_icon_release(clone!(@weak self as window => move |_,_| {
                window.new_task();
            }));
    }
    fn new_task(&self) {
        let buffer = self.imp().entry.buffer();
        let content = buffer.text();
        if content.is_empty() {
            return;
        }
        buffer.set_text("");
        let task = TaskObject::new(false, content);
        self.tasks().append(&task);
    }
    fn setup_factory(&self) {
        let factory = SignalListItemFactory::new();
        factory.connect_setup(move |_, list_item| {
            let task_row = TaskRow::new();
            list_item.set_child(Some(&task_row));
        });
        factory.connect_bind(move |_, list_item| {
            let task_object = list_item.item().unwrap().downcast::<TaskObject>().unwrap();
            let task_row = list_item.child().unwrap().downcast::<TaskRow>().unwrap();
            task_row.bind(&task_object);
        });
        factory.connect_unbind(move |_, list_item| {
            let task_row = list_item.child().unwrap().downcast::<TaskRow>().unwrap();
            task_row.unbind();
        });
        self.imp().tasks_list.set_factory(Some(&factory));
    }
}

mod imp {
    use std::cell::RefCell;

    use gtk::{
        gio,
        glib::{self, subclass::InitializingObject},
        prelude::InitializingWidgetExt,
        subclass::prelude::*,
        CompositeTemplate, Entry, ListView, TemplateChild,
    };

    #[derive(CompositeTemplate, Default)]
    #[template(resource = "/org/gtk_rs/Todo1/window.ui")]
    pub struct Window {
        #[template_child]
        pub entry: TemplateChild<Entry>,
        #[template_child]
        pub tasks_list: TemplateChild<ListView>,
        pub tasks: RefCell<Option<gio::ListStore>>,
    }

    #[glib::object_subclass]
    impl ObjectSubclass for Window {
        const NAME: &'static str = "TodoWindow";
        type Type = super::Window;
        type ParentType = gtk::ApplicationWindow;

        fn class_init(klass: &mut Self::Class) {
            klass.bind_template();
        }

        fn instance_init(obj: &InitializingObject<Self>) {
            obj.init_template();
        }
    }

    impl ObjectImpl for Window {
        fn constructed(&self, obj: &Self::Type) {
            self.parent_constructed(obj);

            obj.setup_tasks();
            obj.setup_callbacks();
            obj.setup_factory();
        }
    }

    impl WidgetImpl for Window {}
    impl WindowImpl for Window {}
    impl ApplicationWindowImpl for Window {}
}
