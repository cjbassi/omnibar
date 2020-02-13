use std::collections::HashMap;

use gtk::prelude::*;
use gtk::Label;
use libpulse_binding::volume::Volume;
use strfmt::{strfmt_map, Formatter};

use crate::module::Module;

pub struct PulseAudio {
    label: Label,
    format: String,
}

impl PulseAudio {
    pub fn new(format: &str) -> Self {
        let label = gtk::Label::new(None);
        let format = format.to_string();

        Self { label, format }
    }
}

impl Module for PulseAudio {
    fn update(&mut self) {
        let percent = Volume::default().print();

        let mut vars = HashMap::new();
        vars.insert("percent".to_string(), percent);

        // let f = |mut fmt: Formatter| fmt.f32(*vars.get(fmt.key).unwrap());

        // self.label
        //     .set_markup(&strfmt_map(&self.format, &f).unwrap());
    }

    fn get_label(&self) -> &Label {
        &self.label
    }
}
