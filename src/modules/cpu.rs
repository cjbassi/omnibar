use std::collections::HashMap;

use gtk::prelude::*;
use gtk::Label;
use psutil::cpu;
use strfmt::{strfmt_map, Formatter};

use crate::module::Module;

pub struct Cpu {
    label: Label,
    format: String,

    collector: cpu::CpuPercentCollector,
}

impl Cpu {
    pub fn new(format: &str) -> Self {
        let label = gtk::Label::new(None);
        let format = format.to_string();
        let collector = cpu::CpuPercentCollector::new().unwrap();

        Self {
            label,
            format,
            collector,
        }
    }
}

impl Module for Cpu {
    fn update(&mut self) {
        let percent = self.collector.cpu_percent().unwrap();

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
