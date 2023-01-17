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
use crate::module::{crew, misc};
use crate::terminalisp::station as tl_station;
use crate::terminalisp::menu::tli_menu;

// module
use crate::section::common::{ModulesContained, Installed};


/* Crew Module Section ========================================================================== */

pub struct CrewModuleSection {
    _name: &'static str,
    _installed: bool,

    pub module_airlock: misc::Airlock,
    pub module_command_module: crew::CommandModule,
    pub module_galley: crew::Galley,
    pub module_life_support: crew::LifeSupport,
    pub module_sleeping_pods: crew::SleepingPods,
    pub module_space_suits: crew::SpaceSuits,
    pub module_temperature_control: misc::TemperatureControl,
    pub module_water_reclamation: crew::WaterReclamation,

    _total_modules: u16,
    _active_modules: u16,
}

impl ModulesContained for CrewModuleSection {
    const MODULES_CONTAINED: u16 = 8;
}

impl CrewModuleSection {
    pub fn new(installed: bool) -> Self {
        let mut section = CrewModuleSection {
            _name: "Crew Module Section",
            _installed: installed,

            module_airlock: misc::Airlock::new(installed),
            module_command_module: crew::CommandModule::new(installed),
            module_galley: crew::Galley::new(installed),
            module_life_support: crew::LifeSupport::new(installed),
            module_sleeping_pods: crew::SleepingPods::new(installed),
            module_space_suits: crew::SpaceSuits::new(installed),
            module_temperature_control: misc::TemperatureControl::new(installed),
            module_water_reclamation: crew::WaterReclamation::new(installed),

            _total_modules: CrewModuleSection::MODULES_CONTAINED,
            _active_modules: 0,
        };

        section.update_active_modules();

        return section;
    }
}

impl Name for CrewModuleSection { fn name(&self) -> String { self._name.to_string() } }
impl Installed for CrewModuleSection { fn installed(&self) -> bool { self._installed } }
impl ModuleCounts for CrewModuleSection {
    fn total_modules(&self) -> u16 { self._total_modules }
    fn active_modules(&self) -> u16 { self._active_modules }
}

impl UpdateModules for CrewModuleSection {
    fn active_module_counts(&self) -> Vec<u16> {
        vec![
            self.module_airlock.active() as u16,
            self.module_command_module.active() as u16,
            self.module_galley.active() as u16,
            self.module_life_support.active() as u16,
            self.module_sleeping_pods.active() as u16,
            self.module_space_suits.active() as u16,
            self.module_temperature_control.active() as u16,
            self.module_water_reclamation.active() as u16
        ]
    }

    fn update_active_modules(&mut self) {
        self._active_modules = self.active_module_sum();
    }
}

impl Status for CrewModuleSection {
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
                self.module_command_module.status(indent + 2),
                self.module_galley.status(indent + 2),
                self.module_life_support.status(indent + 2),
                self.module_sleeping_pods.status(indent + 2),
                self.module_space_suits.status(indent + 2),
                self.module_temperature_control.status(indent + 2),
                self.module_water_reclamation.status(indent + 2)
            ],
            indent
        )
    }
}

impl BreakSomething for CrewModuleSection {
    fn break_something(&mut self) -> Result<String, String> {
        if !self.installed() { return Err("not installed".to_string()); }

        let broken_module: String;

        match thread_rng().gen_range(1..=CrewModuleSection::MODULES_CONTAINED) {
            1 => { broken_module = self.module_airlock.break_something(); },
            2 => { broken_module = self.module_command_module.break_something(); },
            3 => { broken_module = self.module_galley.break_something(); },
            4 => { broken_module = self.module_life_support.break_something(); },
            5 => { broken_module = self.module_sleeping_pods.break_something(); },
            6 => { broken_module = self.module_space_suits.break_something(); },
            7 => { broken_module = self.module_temperature_control.break_something(); },
            8 => { broken_module = self.module_water_reclamation.break_something(); },
            _ => unreachable!(),
        }

        self.update_active_modules();

        return Ok(broken_module);
    }
}

impl Repair for CrewModuleSection {
    fn repairable(&self) -> bool {
        self.installed() && self.active_modules() < self.total_modules()
    }

    fn repair(&mut self) {
        let prompts: Vec<String> = vec![
            self.module_airlock.repair_display(),
            self.module_command_module.repair_display(),
            self.module_galley.repair_display(),
            self.module_life_support.repair_display(),
            self.module_sleeping_pods.repair_display(),
            self.module_space_suits.repair_display(),
            self.module_temperature_control.repair_display(),
            self.module_water_reclamation.repair_display(),
        ];

        let mut options:Vec<String> = vec![];
        if self.module_airlock.repairable()             { options.push(prompts[0].clone()) }
        if self.module_command_module.repairable()      { options.push(prompts[1].clone()) }
        if self.module_galley.repairable()              { options.push(prompts[2].clone()) }
        if self.module_life_support.repairable()        { options.push(prompts[3].clone()) }
        if self.module_sleeping_pods.repairable()       { options.push(prompts[4].clone()) }
        if self.module_space_suits.repairable()         { options.push(prompts[5].clone()) }
        if self.module_temperature_control.repairable() { options.push(prompts[6].clone()) }
        if self.module_water_reclamation.repairable()   { options.push(prompts[7].clone()) }

        let chosen: String = tli_menu("Select module to repair:", options);
        match chosen {
            _ if chosen == prompts[0] => { self.module_airlock.repair(); },
            _ if chosen == prompts[1] => { self.module_command_module.repair(); },
            _ if chosen == prompts[2] => { self.module_galley.repair(); },
            _ if chosen == prompts[3] => { self.module_life_support.repair(); },
            _ if chosen == prompts[4] => { self.module_sleeping_pods.repair(); },
            _ if chosen == prompts[5] => { self.module_space_suits.repair(); },
            _ if chosen == prompts[6] => { self.module_temperature_control.repair(); },
            _ if chosen == prompts[7] => { self.module_water_reclamation.repair(); },
            _ => unreachable!()
        }

        self.update_active_modules();
    }
}

impl PowerDown for CrewModuleSection {
    fn power_down(&mut self) {
        self.module_airlock.power_down();
        self.module_command_module.power_down();
        self.module_galley.power_down();
        self.module_life_support.power_down();
        self.module_sleeping_pods.power_down();
        self.module_space_suits.power_down();
        self.module_temperature_control.power_down();
        self.module_water_reclamation.power_down();

        self.update_active_modules();
    }
}
