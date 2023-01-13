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
use crate::terminalisp::station::{tl_station_status,
                                  tl_until_final_transmission,
                                  tl_end_transmission};

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
        let station = Station {
            name: random(),
            version: random(),
            mission_day: 0,
            sections: (0..10)
                .map(|_| random())
                .collect()
        };

        tl_station_status(&station, true, false);

        return station;
    }

    fn days_left(&self) -> u8 {
        self.sections
            .iter()
            .filter(|m| m.active)
            .count()
            as u8
    }

    pub fn is_shut_down(&self) -> bool {
        return self.days_left() < {1 as u8};
    }

    pub fn shut_down(&mut self) {
        for section in &mut self.sections {
            section.active = false;
        }
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
        if self.is_shut_down() {
            tl_end_transmission();
        }
        else {
            tl_station_status(&self, false, false);
            self.break_something();
            tl_until_final_transmission(self.days_left() as u8);
        }
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
        tl_station_status(&self, true, true);
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
