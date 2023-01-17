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
use crate::module::maneuver;
use crate::terminalisp::station as tl_station;
use crate::terminalisp::menu::tli_menu;

// module
use crate::section::common::{ModulesContained, Installed};


/* Basic Maneuver Section ======================================================================= */

pub struct BasicManeuverSection {
    _name: &'static str,
    _installed: bool,

    pub module_reaction_control_system: maneuver::ReactionControlSystem,

    _total_modules: u16,
    _active_modules: u16,
}

impl ModulesContained for BasicManeuverSection {
    const MODULES_CONTAINED: u16 = 1;
}

impl BasicManeuverSection {
    pub fn new(installed: bool) -> Self {
        let mut section = BasicManeuverSection {
            _name: "Basic Maneuver Section",
            _installed: installed,

            module_reaction_control_system: maneuver::ReactionControlSystem::new(installed),

            _total_modules: BasicManeuverSection::MODULES_CONTAINED,
            _active_modules: 0,
        };

        section.update_active_modules();

        return section;
    }
}

impl Name for BasicManeuverSection { fn name(&self) -> String { self._name.to_string() } }
impl Installed for BasicManeuverSection { fn installed(&self) -> bool { self._installed } }
impl ModuleCounts for BasicManeuverSection {
    fn total_modules(&self) -> u16 { self._total_modules }
    fn active_modules(&self) -> u16 { self._active_modules }
}

impl UpdateModules for BasicManeuverSection {
    fn active_module_counts(&self) -> Vec<u16> {
        vec![
            self.module_reaction_control_system.active() as u16
        ]
    }

    fn update_active_modules(&mut self) {
        self._active_modules = self.active_module_sum();
    }
}

impl Status for BasicManeuverSection {
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
                self.module_reaction_control_system.status(indent + 2)
            ],
            indent
        )
    }
}

impl BreakSomething for BasicManeuverSection {
    fn break_something(&mut self) -> Result<String, String> {
        if !self.installed() { return Err("not installed".to_string()); }

        let broken_module: String;

        match thread_rng().gen_range(1..=BasicManeuverSection::MODULES_CONTAINED) {
            1 => { broken_module = self.module_reaction_control_system.break_something(); },
            _ => unreachable!(),
        }

        self.update_active_modules();

        return Ok(broken_module);
    }
}

impl Repair for BasicManeuverSection {
    fn repairable(&self) -> bool {
        self.installed() && self.active_modules() < self.total_modules()
    }

    fn repair(&mut self) {
        let prompts: Vec<String> = vec![
            self.module_reaction_control_system.repair_display(),
        ];

        let mut options:Vec<String> = vec![];
        if self.module_reaction_control_system.repairable() { options.push(prompts[0].clone()) }

        let chosen: String = tli_menu("Select module to repair:", options);
        match chosen {
            _ if chosen == prompts[0] => { self.module_reaction_control_system.repair(); },
            _ => unreachable!()
        }

        self.update_active_modules();
    }
}

impl PowerDown for BasicManeuverSection {
    fn power_down(&mut self) {
        self.module_reaction_control_system.power_down();

        self.update_active_modules();
    }
}

/* Maneuver With Docking Section ================================================================ */

pub struct ManeuverWithDockingSection {
    _name: &'static str,
    _installed: bool,

    pub module_reaction_control_system: maneuver::ReactionControlSystem,
    pub module_docking_system: maneuver::DockingSystem,

    _total_modules: u16,
    _active_modules: u16,
}

impl ModulesContained for ManeuverWithDockingSection {
    const MODULES_CONTAINED: u16 = 2;
}

impl ManeuverWithDockingSection {
    pub fn new(installed: bool) -> Self {
        let mut section = ManeuverWithDockingSection {
            _name: "Maneuver With Docking Section",
            _installed: installed,

            module_reaction_control_system: maneuver::ReactionControlSystem::new(installed),
            module_docking_system: maneuver::DockingSystem::new(installed),

            _total_modules: ManeuverWithDockingSection::MODULES_CONTAINED,
            _active_modules: 0,
        };

        section.update_active_modules();

        return section;
    }
}

impl Name for ManeuverWithDockingSection { fn name(&self) -> String { self._name.to_string() } }
impl Installed for ManeuverWithDockingSection { fn installed(&self) -> bool { self._installed } }
impl ModuleCounts for ManeuverWithDockingSection {
    fn total_modules(&self) -> u16 { self._total_modules }
    fn active_modules(&self) -> u16 { self._active_modules }
}

impl UpdateModules for ManeuverWithDockingSection {
    fn active_module_counts(&self) -> Vec<u16> {
        vec![
            self.module_reaction_control_system.active() as u16,
            self.module_docking_system.active() as u16
        ]
    }

    fn update_active_modules(&mut self) {
        self._active_modules = self.active_module_sum();
    }
}

impl Status for ManeuverWithDockingSection {
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
                self.module_reaction_control_system.status(indent + 2),
                self.module_docking_system.status(indent + 2)
            ],
            indent
        )
    }
}

impl BreakSomething for ManeuverWithDockingSection {
    fn break_something(&mut self) -> Result<String, String> {
        if !self.installed() { return Err("not installed".to_string()); }

        let broken_module: String;

        match thread_rng().gen_range(1..=ManeuverWithDockingSection::MODULES_CONTAINED) {
            1 => { broken_module = self.module_reaction_control_system.break_something(); },
            2 => { broken_module = self.module_docking_system.break_something(); },
            _ => unreachable!(),
        }

        self.update_active_modules();

        return Ok(broken_module);
    }
}

impl Repair for ManeuverWithDockingSection {
    fn repairable(&self) -> bool {
        self.installed() && self.active_modules() < self.total_modules()
    }

    fn repair(&mut self) {
        let prompts: Vec<String> = vec![
            self.module_reaction_control_system.repair_display(),
            self.module_docking_system.repair_display(),
        ];

        let mut options:Vec<String> = vec![];
        if self.module_reaction_control_system.repairable() { options.push(prompts[0].clone()) }
        if self.module_docking_system.repairable()          { options.push(prompts[1].clone()) }

        let chosen: String = tli_menu("Select module to repair:", options);
        match chosen {
            _ if chosen == prompts[0] => { self.module_reaction_control_system.repair(); },
            _ if chosen == prompts[1] => { self.module_docking_system.repair(); },
            _ => unreachable!()
        }

        self.update_active_modules();
    }
}

impl PowerDown for ManeuverWithDockingSection {
    fn power_down(&mut self) {
        self.module_reaction_control_system.power_down();
        self.module_docking_system.power_down();

        self.update_active_modules();
    }
}
