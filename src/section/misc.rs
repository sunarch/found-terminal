// -*- coding: utf-8, vim: expandtab:ts=4 -*-

// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

// dependencies
use rand::Rng;
use rand::thread_rng;

// project
use crate::station::components::{Name, ModuleCounts,
                                 UpdateModules, Status, BreakSomething, Repair, PowerDown};
use crate::module::common::{Active, StatusModule, BreakModule, RepairModule, PowerDownModule};
use crate::module::{misc, maneuver};
use crate::terminalisp::station as tl_station;
use crate::terminalisp::menu::tli_menu;

// module
use crate::section::common::{ModulesContained, Installed};


/* Cargo Bay Section ============================================================================ */

pub struct CargoBaySection {
    _name: &'static str,
    _installed: bool,

    pub module_airlock: misc::Airlock,
    pub module_cargo_bay: misc::CargoBay,
    pub module_docking_system: maneuver::DockingSystem,

    _total_modules: u16,
    _active_modules: u16,
}

impl ModulesContained for CargoBaySection {
    const MODULES_CONTAINED: u16 = 3;
}

impl CargoBaySection {
    pub fn new(installed: bool) -> Self {
        let mut section = CargoBaySection {
            _name: "Cargo Bay Section",
            _installed: installed,

            module_airlock: misc::Airlock::new(installed),
            module_cargo_bay: misc::CargoBay::new(installed),
            module_docking_system: maneuver::DockingSystem::new(installed),

            _total_modules: CargoBaySection::MODULES_CONTAINED,
            _active_modules: 0,
        };

        section.update_active_modules();

        return section;
    }
}

impl Name for CargoBaySection { fn name(&self) -> String { self._name.to_string() } }
impl Installed for CargoBaySection { fn installed(&self) -> bool { self._installed } }
impl ModuleCounts for CargoBaySection {
    fn total_modules(&self) -> u16 { self._total_modules }
    fn active_modules(&self) -> u16 { self._active_modules }
}

impl UpdateModules for CargoBaySection {
    fn active_module_counts(&self) -> Vec<u16> {
        vec![
            self.module_airlock.active() as u16,
            self.module_cargo_bay.active() as u16,
            self.module_docking_system.active() as u16
        ]
    }

    fn update_active_modules(&mut self) {
        self._active_modules = self.active_module_sum();
    }
}

impl Status for CargoBaySection {
    fn status(&self, indent: u8) -> String {
        tl_station::status(
            String::from("section"),
            true,
            vec![
                String::from(":name")
            ],
            vec![
                format!("\"{}\"", self.name())
            ],
            true,
            String::from(":modules"),
            vec![
                self.module_airlock.status(indent + 2),
                self.module_cargo_bay.status(indent + 2),
                self.module_docking_system.status(indent + 2)
            ],
            indent
        )
    }
}

impl BreakSomething for CargoBaySection {
    fn break_something(&mut self) -> Result<String, String> {
        if !self.installed() { return Err("not installed".to_string()); }

        let broken_module: String;

        match thread_rng().gen_range(1..=CargoBaySection::MODULES_CONTAINED) {
            1 => { broken_module = self.module_airlock.break_something(); },
            2 => { broken_module = self.module_cargo_bay.break_something(); },
            3 => { broken_module = self.module_docking_system.break_something(); },
            _ => unreachable!(),
        }

        self.update_active_modules();

        return Ok(broken_module);
    }
}

impl Repair for CargoBaySection {
    fn repair(&mut self) {
        let prompts: Vec<String> = vec![
            self.module_airlock.repair_display(),
            self.module_cargo_bay.repair_display(),
            self.module_docking_system.repair_display(),
        ];

        let mut options:Vec<String> = vec![];
        if self.module_airlock.active() {
            options.push(prompts[0].clone())
        }
        if self.module_cargo_bay.active() {
            options.push(prompts[1].clone())
        }
        if self.module_docking_system.active() {
            options.push(prompts[2].clone())
        }

        let chosen: String = tli_menu("Select module to repair:", options);
        match chosen {
            _ if chosen == prompts[0] => { self.module_airlock.repair(); },
            _ if chosen == prompts[1] => { self.module_cargo_bay.repair(); },
            _ if chosen == prompts[2] => { self.module_docking_system.repair(); },
            _ => unreachable!()
        }

        self.update_active_modules();
    }
}

impl PowerDown for CargoBaySection {
    fn power_down(&mut self) {
        self.module_airlock.power_down();
        self.module_cargo_bay.power_down();
        self.module_docking_system.power_down();

        self.update_active_modules();
    }
}
