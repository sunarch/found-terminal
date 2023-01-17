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
use crate::module::{research, misc};
use crate::terminalisp::station as tl_station;
use crate::terminalisp::menu::tli_menu;

// module
use crate::section::common::{ModulesContained, Installed};


/* Astronomy Section ============================================================================ */

pub struct AstronomySection {
    _name: &'static str,
    _installed: bool,

    pub module_astronomy_lab: research::AstronomyLab,
    pub module_mainframe: research::Mainframe,

    _total_modules: u16,
    _active_modules: u16,
}

impl ModulesContained for AstronomySection {
    const MODULES_CONTAINED: u16 = 2;
}

impl AstronomySection {
    pub fn new(installed: bool) -> Self {
        let mut section = AstronomySection {
            _name: "Astronomy Section",
            _installed: installed,

            module_astronomy_lab: research::AstronomyLab::new(installed),
            module_mainframe: research::Mainframe::new(installed),

            _total_modules: AstronomySection::MODULES_CONTAINED,
            _active_modules: 0,
        };

        section.update_active_modules();

        return section;
    }
}

impl Name for AstronomySection { fn name(&self) -> String { self._name.to_string() } }
impl Installed for AstronomySection { fn installed(&self) -> bool { self._installed } }
impl ModuleCounts for AstronomySection {
    fn total_modules(&self) -> u16 { self._total_modules }
    fn active_modules(&self) -> u16 { self._active_modules }
}

impl UpdateModules for AstronomySection {
    fn active_module_counts(&self) -> Vec<u16> {
        vec![
            self.module_astronomy_lab.active() as u16,
            self.module_mainframe.active() as u16
        ]
    }

    fn update_active_modules(&mut self) {
        self._active_modules = self.active_module_sum();
    }
}

impl Status for AstronomySection {
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
                self.module_astronomy_lab.status(indent + 2),
                self.module_mainframe.status(indent + 2)
            ],
            indent
        )
    }
}

impl BreakSomething for AstronomySection {
    fn break_something(&mut self) -> Result<String, String> {
        if !self.installed() { return Err("not installed".to_string()); }

        let broken_module: String;

        match thread_rng().gen_range(1..=AstronomySection::MODULES_CONTAINED) {
            1 => { broken_module = self.module_astronomy_lab.break_something(); },
            2 => { broken_module = self.module_mainframe.break_something(); },
            _ => unreachable!(),
        }

        self.update_active_modules();

        return Ok(broken_module);
    }
}

impl Repair for AstronomySection {
    fn repairable(&self) -> bool {
        self.installed() && self.active_modules() < self.total_modules()
    }

    fn repair(&mut self) {
        let prompts: Vec<String> = vec![
            self.module_astronomy_lab.repair_display(),
            self.module_mainframe.repair_display(),
        ];

        let mut options:Vec<String> = vec![];
        if self.module_astronomy_lab.repairable() { options.push(prompts[0].clone()) }
        if self.module_mainframe.repairable()     { options.push(prompts[1].clone()) }

        let chosen: String = tli_menu("Select module to repair:", options);
        match chosen {
            _ if chosen == prompts[0] => { self.module_astronomy_lab.repair(); },
            _ if chosen == prompts[0] => { self.module_mainframe.repair(); },
            _ => unreachable!()
        }
    }
}

impl PowerDown for AstronomySection {
    fn power_down(&mut self) {
        self.module_astronomy_lab.power_down();
        self.module_mainframe.power_down();
    }
}

/* Greenhouse Section =========================================================================== */

pub struct GreenhouseSection {
    _name: &'static str,
    _installed: bool,

    pub module_greenhouse: research::Greenhouse,
    pub module_mainframe: research::Mainframe,
    pub module_airlock: misc::Airlock,
    pub module_temperature_control: misc::TemperatureControl,

    _total_modules: u16,
    _active_modules: u16,
}

impl ModulesContained for GreenhouseSection {
    const MODULES_CONTAINED: u16 = 4;
}

impl GreenhouseSection {
    pub fn new(installed: bool) -> Self {
        let mut section = GreenhouseSection {
            _name: "Greenhouse Section",
            _installed: installed,

            module_greenhouse: research::Greenhouse::new(installed),
            module_mainframe: research::Mainframe::new(installed),
            module_airlock: misc::Airlock::new(installed),
            module_temperature_control: misc::TemperatureControl::new(installed),

            _total_modules: GreenhouseSection::MODULES_CONTAINED,
            _active_modules: 0,
        };

        section.update_active_modules();

        return section;
    }
}

impl Name for GreenhouseSection { fn name(&self) -> String { self._name.to_string() } }
impl Installed for GreenhouseSection { fn installed(&self) -> bool { self._installed } }
impl ModuleCounts for GreenhouseSection {
    fn total_modules(&self) -> u16 { self._total_modules }
    fn active_modules(&self) -> u16 { self._active_modules }
}

impl UpdateModules for GreenhouseSection {
    fn active_module_counts(&self) -> Vec<u16> {
        vec![
            self.module_greenhouse.active() as u16,
            self.module_mainframe.active() as u16,
            self.module_airlock.active() as u16,
            self.module_temperature_control.active() as u16
        ]
    }

    fn update_active_modules(&mut self) {
        self._active_modules = self.active_module_sum();
    }
}

impl Status for GreenhouseSection {
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
                self.module_greenhouse.status(indent + 2),
                self.module_mainframe.status(indent + 2),
                self.module_airlock.status(indent + 2),
                self.module_temperature_control.status(indent + 2)
            ],
            indent
        )
    }
}

impl BreakSomething for GreenhouseSection {
    fn break_something(&mut self) -> Result<String, String> {
        if !self.installed() { return Err("not installed".to_string()); }

        let broken_module: String;

        match thread_rng().gen_range(1..=GreenhouseSection::MODULES_CONTAINED) {
            1 => { broken_module = self.module_greenhouse.break_something(); },
            2 => { broken_module = self.module_mainframe.break_something(); },
            3 => { broken_module = self.module_airlock.break_something(); },
            4 => { broken_module = self.module_temperature_control.break_something(); },
            _ => unreachable!(),
        }

        self.update_active_modules();

        return Ok(broken_module);
    }
}

impl Repair for GreenhouseSection {
    fn repairable(&self) -> bool {
        self.installed() && self.active_modules() < self.total_modules()
    }

    fn repair(&mut self) {
        let prompts: Vec<String> = vec![
            self.module_greenhouse.repair_display(),
            self.module_mainframe.repair_display(),
            self.module_airlock.repair_display(),
            self.module_temperature_control.repair_display(),
        ];

        let mut options:Vec<String> = vec![];
        if self.module_greenhouse.repairable()          { options.push(prompts[0].clone()) }
        if self.module_mainframe.repairable()           { options.push(prompts[1].clone()) }
        if self.module_airlock.repairable()             { options.push(prompts[2].clone()) }
        if self.module_temperature_control.repairable() { options.push(prompts[3].clone()) }

        let chosen: String = tli_menu("Select module to repair:", options);
        match chosen {
            _ if chosen == prompts[0] => { self.module_greenhouse.repair(); },
            _ if chosen == prompts[1] => { self.module_mainframe.repair(); },
            _ if chosen == prompts[2] => { self.module_airlock.repair(); },
            _ if chosen == prompts[3] => { self.module_temperature_control.repair(); },
            _ => unreachable!()
        }
    }
}

impl PowerDown for GreenhouseSection {
    fn power_down(&mut self) {
        self.module_greenhouse.power_down();
        self.module_mainframe.power_down();
        self.module_airlock.power_down();
        self.module_temperature_control.power_down();
    }
}

/* Weather Observation Section ================================================================== */

pub struct WeatherObservationSection {
    _name: &'static str,
    _installed: bool,

    pub module_weather_observation: research::WeatherObservation,
    pub module_mainframe: research::Mainframe,

    _total_modules: u16,
    _active_modules: u16,
}

impl ModulesContained for WeatherObservationSection {
    const MODULES_CONTAINED: u16 = 2;
}

impl WeatherObservationSection {
    pub fn new(installed: bool) -> Self {
        let mut section = WeatherObservationSection {
            _name: "Weather Observation Section",
            _installed: installed,

            module_weather_observation: research::WeatherObservation::new(installed),
            module_mainframe: research::Mainframe::new(installed),

            _total_modules: WeatherObservationSection::MODULES_CONTAINED,
            _active_modules: 0,
        };

        section.update_active_modules();

        return section;
    }
}

impl Name for WeatherObservationSection { fn name(&self) -> String { self._name.to_string() } }
impl Installed for WeatherObservationSection { fn installed(&self) -> bool { self._installed } }
impl ModuleCounts for WeatherObservationSection {
    fn total_modules(&self) -> u16 { self._total_modules }
    fn active_modules(&self) -> u16 { self._active_modules }
}

impl UpdateModules for WeatherObservationSection {
    fn active_module_counts(&self) -> Vec<u16> {
        vec![
            self.module_weather_observation.active() as u16,
            self.module_mainframe.active() as u16
        ]
    }

    fn update_active_modules(&mut self) {
        self._active_modules = self.active_module_sum();
    }
}

impl Status for WeatherObservationSection {
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
                self.module_weather_observation.status(indent + 2),
                self.module_mainframe.status(indent + 2)
            ],
            indent
        )
    }
}

impl BreakSomething for WeatherObservationSection {
    fn break_something(&mut self) -> Result<String, String> {
        if !self.installed() { return Err("not installed".to_string()); }

        let broken_module: String;

        match thread_rng().gen_range(1..=WeatherObservationSection::MODULES_CONTAINED) {
            1 => { broken_module = self.module_weather_observation.break_something(); },
            2 => { broken_module = self.module_mainframe.break_something(); },
            _ => unreachable!(),
        }

        self.update_active_modules();

        return Ok(broken_module);
    }
}

impl Repair for WeatherObservationSection {
    fn repairable(&self) -> bool {
        self.installed() && self.active_modules() < self.total_modules()
    }

    fn repair(&mut self) {
        let prompts: Vec<String> = vec![
            self.module_weather_observation.repair_display(),
            self.module_mainframe.repair_display(),
        ];

        let mut options:Vec<String> = vec![];
        if self.module_weather_observation.repairable() { options.push(prompts[0].clone()) }
        if self.module_mainframe.repairable()           { options.push(prompts[1].clone()) }

        let chosen: String = tli_menu("Select module to repair:", options);
        match chosen {
            _ if chosen == prompts[0] => { self.module_weather_observation.repair(); },
            _ if chosen == prompts[1] => { self.module_mainframe.repair(); },
            _ => unreachable!()
        }

        self.update_active_modules();
    }
}

impl PowerDown for WeatherObservationSection {
    fn power_down(&mut self) {
        self.module_weather_observation.power_down();
        self.module_mainframe.power_down();

        self.update_active_modules();
    }
}
