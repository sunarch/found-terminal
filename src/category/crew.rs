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
use crate::section::crew;
use crate::section::common::Installed;
use crate::terminalisp::station as tl_station;
use crate::terminalisp::menu::tli_menu;

// module
use crate::category::common::{SectionsAvailable, random_bools};


pub struct CrewCategory {
    _name: &'static str,

    pub section_crew_module: crew::CrewModuleSection,

    _total_sections: u16,
    _installed_sections: u16,
    _total_modules: u16,
    _active_modules: u16,
}

impl SectionsAvailable for CrewCategory {
    const SECTIONS_AVAILABLE: u16 = 1;
}

impl CrewCategory {
    pub fn new(min_count: u16, max_count: u16) -> Self {
        let installation: Vec<bool> = random_bools(CrewCategory::SECTIONS_AVAILABLE, min_count, max_count);

        let mut section_group = CrewCategory {
            _name: "Crew Category",

            section_crew_module: crew::CrewModuleSection::new(installation[0]),

            _total_sections: CrewCategory::SECTIONS_AVAILABLE,
            _installed_sections: installation.iter().filter(|x| x == &&true).count() as u16,
            _total_modules: 0,
            _active_modules: 0,
        };

        if section_group.section_crew_module.installed() {
            section_group._total_modules += section_group.section_crew_module.total_modules();
        }

        section_group.update_active_modules();

        return section_group;
    }
}

impl Name for CrewCategory { fn name(&self) -> String { self._name.to_string() } }
impl SectionCounts for CrewCategory {
    fn total_sections(&self) -> u16 { self._total_sections }
    fn installed_sections(&self) -> u16 { self._installed_sections }
}
impl ModuleCounts for CrewCategory {
    fn total_modules(&self) -> u16 { self._total_modules }
    fn active_modules(&self) -> u16 { self._active_modules }
}

impl UpdateModules for CrewCategory {
    fn active_module_counts(&self) -> Vec<u16> {
        vec![
            self.section_crew_module.active_modules()
        ]
    }

    fn update_active_modules(&mut self) {
        self._active_modules = self.active_module_sum();
    }
}

impl Status for CrewCategory {
    fn status(&self, indent: u8) -> String {
        let mut modules: Vec<String> = vec![];

        if self.section_crew_module.installed() {
            modules.push(self.section_crew_module.status(indent + 2));
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

impl BreakSomething for CrewCategory {
    fn break_something(&mut self) -> Result<String, String> {
        let broken_module: Result<String, String>;

        match thread_rng().gen_range(1..=CrewCategory::SECTIONS_AVAILABLE) {
            1 => { broken_module = self.section_crew_module.break_something(); },
            _ => unreachable!(),
        }

        self.update_active_modules();

        match broken_module {
            Ok(v) => { Ok(v) },
            Err(e) => { Err(e) },
        }
    }
}

impl Repair for CrewCategory {
    fn repairable(&self) -> bool {
        self.active_modules() < self.total_modules()
    }

    fn repair(&mut self) {
        let prompts: Vec<String> = vec![
            self.section_crew_module.repair_display(),
        ];

        let mut options:Vec<String> = vec![];
        if self.section_crew_module.repairable() { options.push(prompts[0].clone()) }

        let chosen: String = tli_menu("Select section to repair:", options);
        match chosen {
            _ if chosen == prompts[0] => { self.section_crew_module.repair(); },
            _ => unreachable!()
        }

        self.update_active_modules();
    }
}

impl PowerDown for CrewCategory {
    fn power_down(&mut self) {
        self.section_crew_module.power_down();

        self.update_active_modules();
    }
}
