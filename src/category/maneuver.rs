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
use crate::section::maneuver;
use crate::section::common::Installed;
use crate::terminalisp::station as tl_station;
use crate::terminalisp::menu::tli_menu;

// module
use crate::category::common::{SectionsAvailable, random_bools};


pub struct ManeuverCategory {
    _name: &'static str,

    pub section_basic_maneuver: maneuver::BasicManeuverSection,
    pub section_maneuver_with_docking: maneuver::ManeuverWithDockingSection,

    _total_sections: u16,
    _installed_sections: u16,
    _total_modules: u16,
    _active_modules: u16,
}

impl SectionsAvailable for ManeuverCategory {
    const SECTIONS_AVAILABLE: u16 = 2;
}

impl ManeuverCategory {
    pub fn new(min_count: u16, max_count: u16) -> Self {
        let installation: Vec<bool> = random_bools(ManeuverCategory::SECTIONS_AVAILABLE, min_count, max_count);

        let mut section_group = ManeuverCategory {
            _name: "Maneuver Category",

            section_basic_maneuver: maneuver::BasicManeuverSection::new(installation[0]),
            section_maneuver_with_docking: maneuver::ManeuverWithDockingSection::new(installation[1]),

            _total_sections: ManeuverCategory::SECTIONS_AVAILABLE,
            _installed_sections: installation.iter().filter(|x| x == &&true).count() as u16,
            _total_modules: 0,
            _active_modules: 0,
        };

        if section_group.section_basic_maneuver.installed() {
            section_group._total_modules += section_group.section_basic_maneuver.total_modules();
        }
        if section_group.section_maneuver_with_docking.installed() {
            section_group._total_modules += section_group.section_maneuver_with_docking.total_modules();
        }

        section_group.update_active_modules();

        return section_group;
    }
}

impl Name for ManeuverCategory { fn name(&self) -> String { self._name.to_string() } }
impl SectionCounts for ManeuverCategory {
    fn total_sections(&self) -> u16 { self._total_sections }
    fn installed_sections(&self) -> u16 { self._installed_sections }
}
impl ModuleCounts for ManeuverCategory {
    fn total_modules(&self) -> u16 { self._total_modules }
    fn active_modules(&self) -> u16 { self._active_modules }
}

impl UpdateModules for ManeuverCategory {
    fn active_module_counts(&self) -> Vec<u16> {
        vec![
            self.section_basic_maneuver.active_modules(),
            self.section_maneuver_with_docking.active_modules()
        ]
    }

    fn update_active_modules(&mut self) {
        self._active_modules = self.active_module_sum();
    }
}

impl Status for ManeuverCategory {
    fn status(&self, indent: u8) -> String {
        let mut modules: Vec<String> = vec![];

        if self.section_basic_maneuver.installed() {
            modules.push(self.section_basic_maneuver.status(indent + 2));
        }
        if self.section_maneuver_with_docking.installed() {
            modules.push(self.section_maneuver_with_docking.status(indent + 2));
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

impl BreakSomething for ManeuverCategory {
    fn break_something(&mut self) -> Result<String, String> {
        let broken_module: Result<String, String>;

        match thread_rng().gen_range(1..=ManeuverCategory::SECTIONS_AVAILABLE) {
            1 => { broken_module = self.section_basic_maneuver.break_something(); },
            2 => { broken_module = self.section_maneuver_with_docking.break_something(); },
            _ => unreachable!(),
        }

        self.update_active_modules();

        match broken_module {
            Ok(v) => { Ok(v) },
            Err(e) => { Err(e) },
        }
    }
}

impl Repair for ManeuverCategory {
    fn repairable(&self) -> bool {
        self.active_modules() < self.total_modules()
    }

    fn repair(&mut self) {
        let prompts: Vec<String> = vec![
            self.section_basic_maneuver.repair_display(),
            self.section_maneuver_with_docking.repair_display(),
        ];

        let mut options:Vec<String> = vec![];
        if self.section_basic_maneuver.repairable()        { options.push(prompts[0].clone()) }
        if self.section_maneuver_with_docking.repairable() { options.push(prompts[1].clone()) }

        let chosen: String;
        match tli_menu("Select section to repair:", options) {
            Ok(v) => { chosen = v; },
            Err(_) => { return; }
        }
        match chosen {
            _ if chosen == prompts[0] => { self.section_basic_maneuver.repair(); },
            _ if chosen == prompts[1] => { self.section_maneuver_with_docking.repair(); },
            _ => unreachable!()
        }

        self.update_active_modules();
    }
}

impl PowerDown for ManeuverCategory {
    fn power_down(&mut self) {
        self.section_basic_maneuver.power_down();
        self.section_maneuver_with_docking.power_down();

        self.update_active_modules();
    }
}
