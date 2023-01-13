// -*- coding: utf-8, vim: expandtab:ts=4 -*-

// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

// dependencies
use rand::random;
use rand::Rng;
use rand::thread_rng;
use rand_derive2::RandGen;

// project
use crate::section::section::Section;

// module
use crate::station::name::StationName;


#[derive(RandGen)]
pub struct Station {
    pub name: StationName,
    pub version: u8,
    pub mission_day: u16,
    pub sections: Vec<Section>
}

impl Station {
    pub fn new() -> Self {
        Station {
            name: random(),
            version: random(),
            mission_day: 0,
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
        self.increment_mission_day();
        println!("{}", self.mission_day_display());
        self.break_something();
    }

    pub fn break_something(&mut self) {
        let broken_index = thread_rng().gen_range(0..self.sections.len());
        let mut broken_section = &mut self.sections[broken_index];
        if broken_section.active {
            broken_section.active = false;
            println!("(section-failure {})", &broken_section.name);
        } else {
            println!("(sections 'ok)");
        }
    }

    pub fn status(&self) {
        println!("{:-^80}", " [ BEGIN: STATION STATUS ] ");
        println!("{}", &self.name_display());
        println!("{:-^10} {} {:-^10}", "", "[ Sections ]", "");
        for section in &self.sections {
            let active = if section.active {"   OK   "} else {"INACTIVE"};
            println!("[{}] {}", active, section.name);
        }
        println!("{:-^80}", " [ END: STATION STATUS ] ");
    }

    pub fn name_display(&self) -> String {
        return format!("Station \"{}\" v{}", &self.name, &self.version);
    }

    pub fn mission_day_display(&self) -> String {
        let mission_day = self.mission_day;
        return format!("Mission Day {mission_day}");
    }

    fn increment_mission_day(&mut self) {
        self.mission_day = self.mission_day.saturating_add(1);
    }
}
