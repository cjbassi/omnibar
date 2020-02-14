use std::collections::HashMap;

use gtk::prelude::*;
use gtk::Label;
use psutil::memory;
use strfmt::{strfmt_map, Formatter};

use crate::module::Module;

pub struct Memory {
    label: Label,
    format: String,
}

impl Memory {
    pub fn new(format: &str) -> Self {
        let label = gtk::Label::new(None);
        label.set_widget_name("memory");
        let format = format.to_string();

        Self { label, format }
    }
}

impl Module for Memory {
    fn update(&mut self) {
        let percent = memory::virtual_memory().unwrap().percent();

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
