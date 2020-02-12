use gtk::Label;

pub trait Module {
    fn update(&mut self);
    fn get_label(&self) -> &Label;
}
