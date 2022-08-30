use std::cell::Cell;

use gtk::{glib, subclass::prelude::*, traits::ButtonExt};



#[derive(Default)]
pub struct CustomButton {
    number: Cell<i32>,
}

#[glib::object_subclass]
impl ObjectSubclass for CustomButton {
    const NAME: &'static str = "MyGtkAppCustomButton";
    type Type = super::CustomButton;
    type ParentType = gtk::Button;
}

impl ObjectImpl for CustomButton {
    fn constructed(&self, obj: &Self::Type) {
        self.parent_constructed(obj);
        obj.set_label(&self.number.get().to_string());
    }
}
impl WidgetImpl for CustomButton {}
impl ButtonImpl for CustomButton {
    fn clicked(&self, button: &Self::Type) {
        self.number.set(self.number.get() + 1);
        button.set_label(&self.number.get().to_string())
    }
}