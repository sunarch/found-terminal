// -*- coding: utf-8, vim: expandtab:ts=4 -*-

// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

// dependencies
use rand::Rng;
use rand::thread_rng;

// project
use crate::station::components::{Name, SectionCounts, ModuleCounts,
                                 UpdateModules, Status, BreakSomething, Repair, PowerDown};
use crate::section::comms;
use crate::section::common::Installed;
use crate::terminalisp::station as tl_station;
use crate::terminalisp::menu::tli_menu;


// module
use crate::category::common::{SectionsAvailable, random_bools};


pub struct CommsCategory {
    _name: &'static str,

    pub section_antenna: comms::AntennaSection,
    pub section_tracking: comms::TrackingSection,
    pub section_transponder: comms::TransponderSection,

    _total_sections: u16,
    _installed_sections: u16,
    _total_modules: u16,
    _active_modules: u16,
}

impl SectionsAvailable for CommsCategory {
    const SECTIONS_AVAILABLE: u16 = 3;
}

impl CommsCategory {
    pub fn new(min_count: u16, max_count: u16) -> Self {
        let installation: Vec<bool> = random_bools(CommsCategory::SECTIONS_AVAILABLE, min_count, max_count);

        let mut section_group = CommsCategory {
            _name: "Comms Category",

            section_antenna: comms::AntennaSection::new(installation[0]),
            section_tracking: comms::TrackingSection::new(installation[1]),
            section_transponder: comms::TransponderSection::new(installation[2]),

            _total_sections: CommsCategory::SECTIONS_AVAILABLE,
            _installed_sections: installation.iter().filter(|x| x == &&true).count() as u16,
            _total_modules: 0,
            _active_modules: 0,
        };

        if section_group.section_antenna.installed() {
            section_group._total_modules += section_group.section_antenna.total_modules();
        }
        if section_group.section_tracking.installed() {
            section_group._total_modules += section_group.section_tracking.total_modules();
        }
        if section_group.section_transponder.installed() {
            section_group._total_modules += section_group.section_transponder.total_modules();
        }

        section_group.update_active_modules();

        return section_group;
    }
}

impl Name for CommsCategory { fn name(&self) -> String { self._name.to_string() } }
impl SectionCounts for CommsCategory {
    fn total_sections(&self) -> u16 { self._total_sections }
    fn installed_sections(&self) -> u16 { self._installed_sections }
}
impl ModuleCounts for CommsCategory {
    fn total_modules(&self) -> u16 { self._total_modules }
    fn active_modules(&self) -> u16 { self._active_modules }
}

impl UpdateModules for CommsCategory {
    fn active_module_counts(&self) -> Vec<u16> {
        vec![
            self.section_antenna.active_modules(),
            self.section_tracking.active_modules(),
            self.section_transponder.active_modules()
        ]
    }

    fn update_active_modules(&mut self) {
        self._active_modules = self.active_module_sum();
    }
}

impl Status for CommsCategory {
    fn status(&self, indent: u8) -> String {
        let mut modules: Vec<String> = vec![];

        if self.section_antenna.installed() {
            modules.push(self.section_antenna.status(indent + 2));
        }
        if self.section_tracking.installed() {
            modules.push(self.section_tracking.status(indent + 2));
        }
        if self.section_transponder.installed() {
            modules.push(self.section_transponder.status(indent + 2));
        }

        tl_station::status(
            String::from("category"),
            true,
            vec![
                String::from(":name"),
                String::from(":installed-sections"),
                String::from(":total-modules"),
                String::from(":active-modules")
            ],
            vec![
                format!("\"{}\"", self.name()),
                format!("{}", self.installed_sections()),
                format!("{}", self.total_modules()),
                format!("{}", self.active_modules())
            ],
            true,
            String::from(":sections"),
            modules,
            indent
        )
    }
}

impl BreakSomething for CommsCategory {
    fn break_something(&mut self) -> Result<String, String> {
        let broken_module: Result<String, String>;

        match thread_rng().gen_range(1..=CommsCategory::SECTIONS_AVAILABLE) {
            1 => { broken_module = self.section_antenna.break_something(); },
            2 => { broken_module = self.section_tracking.break_something(); },
            3 => { broken_module = self.section_transponder.break_something(); },
            _ => unreachable!(),
        }

        self.update_active_modules();

        match broken_module {
            Ok(v) => { Ok(v) },
            Err(e) => { Err(e) },
        }
    }
}

impl Repair for CommsCategory {
    fn repairable(&self) -> bool {
        self.active_modules() < self.total_modules()
    }

    fn repair(&mut self) {
        let prompts: Vec<String> = vec![
            self.section_antenna.repair_display(),
            self.section_tracking.repair_display(),
            self.section_transponder.repair_display(),
        ];

        let mut options:Vec<String> = vec![];
        if self.section_antenna.repairable()     { options.push(prompts[0].clone()) }
        if self.section_tracking.repairable()    { options.push(prompts[1].clone()) }
        if self.section_transponder.repairable() { options.push(prompts[2].clone()) }

        let chosen: String;
        match tli_menu("Select section to repair:", options) {
            Ok(v) => { chosen = v; },
            Err(_) => { return; }
        }
        match chosen {
            _ if chosen == prompts[0] => { self.section_antenna.repair(); },
            _ if chosen == prompts[1] => { self.section_tracking.repair(); },
            _ if chosen == prompts[2] => { self.section_transponder.repair(); },
            _ => unreachable!()
        }

        self.update_active_modules();
    }
}

impl PowerDown for CommsCategory {
    fn power_down(&mut self) {
        self.section_antenna.power_down();
        self.section_tracking.power_down();
        self.section_transponder.power_down();

        self.update_active_modules();
    }
}
