use std::collections::HashMap;

use gtk::prelude::*;
use gtk::Label;
use psutil::disk;
use strfmt::{strfmt_map, Formatter};

use crate::module::Module;

pub struct Disk {
    label: Label,
    format: String,

    path: String,
}

impl Disk {
    pub fn new(format: &str, path: &str) -> Self {
        let label = gtk::Label::new(None);
        let format = format.to_string();
        let path = path.to_string();

        Self {
            label,
            format,
            path,
        }
    }
}

impl Module for Disk {
    fn update(&mut self) {
        let percent = disk::disk_usage(&self.path).unwrap().percent();

        let mut vars = HashMap::new();
        vars.insert("percent".to_string(), percent);

        let f = |mut fmt: Formatter| fmt.f32(*vars.get(fmt.key).unwrap());

        self.label
            .set_markup(&strfmt_map(&self.format, &f).unwrap());
    }

    fn get_label(&self) -> &Label {
        &self.label
    }
}
