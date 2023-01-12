// -*- coding: utf-8, vim: expandtab:ts=4 -*-

// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use rand_derive2::RandGen;
use strum_macros::Display;
use rand::random;
use rand::Rng;
use rand::thread_rng;

use crate::section::section::Section;


#[derive(Debug, RandGen)]
pub struct Station {
    pub name: StationName,
    pub version: u8,
    pub sections: Vec<Section>
}

#[derive(Debug, RandGen, Display)]
pub enum StationName {
    Akira,     California, Daedalus,
    Eisenberg, Intrepid,   Miranda,
    Nova,      Reliant,    Sagan
}

impl Station {
    pub fn new() -> Self {
        Station {
            name: random(),
            version: random(),
            sections: (0..10)
                .map(|_| random())
                .collect()
        }
    }

    pub fn days_left(&self) -> usize {
        self.sections
            .iter()
            .filter(|m| m.active)
            .count()
    }

    pub fn working_sections(&self) -> Vec<String> {
        self.sections
            .iter()
            .filter(|m| m.active)
            .map(|m| m.name.to_string())
            .collect()
    }

    pub fn broken_sections(&self) -> Vec<String> {
        self.sections
            .iter()
            .filter(|m| !m.active)
            .map(|m| m.name.to_string())
            .collect()
    }

    pub fn new_day(&mut self) {
        self.break_something();
    }

    pub fn break_something(&mut self) {
        let broken_index = thread_rng().gen_range(0..self.sections.len());
        let mut broken_section = &mut self.sections[broken_index];
        if broken_section.active {
            broken_section.active = false;
            println!("(Section-FAILURE {})", &broken_section.name);
        } else {
            println!("(sections OK)");
        }
    }

    pub fn status(&self) {
        dbg!(&self);
    }
}
