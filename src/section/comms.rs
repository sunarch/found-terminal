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
use crate::module::comms;
use crate::terminalisp::station as tl_station;
use crate::terminalisp::menu::tli_menu;

// module
use crate::section::common::{ModulesContained, Installed};


/* Antenna Section ============================================================================== */

pub struct AntennaSection {
    _name: &'static str,
    _installed: bool,

    pub module_antenna: comms::Antenna,

    _total_modules: u16,
    _active_modules: u16,
}

impl ModulesContained for AntennaSection {
    const MODULES_CONTAINED: u16 = 1;
}

impl AntennaSection {
    pub fn new(installed: bool) -> Self {
        let mut section = AntennaSection {
            _name: "Antenna Section",
            _installed: installed,

            module_antenna: comms::Antenna::new(installed),

            _total_modules: AntennaSection::MODULES_CONTAINED,
            _active_modules: 0,
        };

        section.update_active_modules();

        return section;
    }
}

impl Name for AntennaSection { fn name(&self) -> String { self._name.to_string() } }
impl Installed for AntennaSection { fn installed(&self) -> bool { self._installed } }
impl ModuleCounts for AntennaSection {
    fn total_modules(&self) -> u16 { self._total_modules }
    fn active_modules(&self) -> u16 { self._active_modules }
}

impl UpdateModules for AntennaSection {
    fn active_module_counts(&self) -> Vec<u16> {
        vec![
            self.module_antenna.active() as u16
        ]
    }

    fn update_active_modules(&mut self) {
        self._active_modules = self.active_module_sum();
    }
}

impl Status for AntennaSection {
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
                self.module_antenna.status(indent + 2)
            ],
            indent
        )
    }
}

impl BreakSomething for AntennaSection {
    fn break_something(&mut self) -> Result<String, String> {
        if !self.installed() { return Err("not installed".to_string()); }

        let broken_module: String;

        match thread_rng().gen_range(1..=AntennaSection::MODULES_CONTAINED) {
            1 => { broken_module = self.module_antenna.break_something(); },
            _ => unreachable!(),
        }

        self.update_active_modules();

        return Ok(broken_module);
    }
}

impl Repair for AntennaSection {
    fn repairable(&self) -> bool {
        self.installed() && self.active_modules() < self.total_modules()
    }

    fn repair(&mut self) {
        let prompts: Vec<String> = vec![
            self.module_antenna.repair_display(),
        ];

        let mut options:Vec<String> = vec![];
        if self.module_antenna.repairable() { options.push(prompts[0].clone()) }

        let chosen: String = tli_menu("Select module to repair:", options);
        match chosen {
            _ if chosen == prompts[0] => { self.module_antenna.repair(); },
            _ => unreachable!()
        }

        self.update_active_modules();
    }
}

impl PowerDown for AntennaSection {
    fn power_down(&mut self) {
        self.module_antenna.power_down();

        self.update_active_modules();
    }
}

/* Tracking Section ============================================================================= */

pub struct TrackingSection {
    _name: &'static str,
    _installed: bool,

    pub module_tracking: comms::Tracking,

    _total_modules: u16,
    _active_modules: u16,
}

impl ModulesContained for TrackingSection {
    const MODULES_CONTAINED: u16 = 1;
}

impl TrackingSection {
    pub fn new(installed: bool) -> Self {
        let mut section = TrackingSection {
            _name: "Tracking Section",
            _installed: installed,

            module_tracking: comms::Tracking::new(installed),

            _total_modules: TrackingSection::MODULES_CONTAINED,
            _active_modules: 0,
        };

        section.update_active_modules();

        return section;
    }
}

impl Name for TrackingSection { fn name(&self) -> String { self._name.to_string() } }
impl Installed for TrackingSection { fn installed(&self) -> bool { self._installed } }
impl ModuleCounts for TrackingSection {
    fn total_modules(&self) -> u16 { self._total_modules }
    fn active_modules(&self) -> u16 { self._active_modules }
}

impl UpdateModules for TrackingSection {
    fn active_module_counts(&self) -> Vec<u16> {
        vec![
            self.module_tracking.active() as u16
        ]
    }

    fn update_active_modules(&mut self) {
        self._active_modules = self.active_module_sum();
    }
}

impl Status for TrackingSection {
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
                self.module_tracking.status(indent + 2)
            ],
            indent
        )
    }
}

impl BreakSomething for TrackingSection {
    fn break_something(&mut self) -> Result<String, String> {
        if !self.installed() { return Err("not installed".to_string()); }

        let broken_module: String;

        match thread_rng().gen_range(1..=TrackingSection::MODULES_CONTAINED) {
            1 => { broken_module = self.module_tracking.break_something(); },
            _ => unreachable!(),
        }

        self.update_active_modules();

        return Ok(broken_module);
    }
}

impl Repair for TrackingSection {
    fn repairable(&self) -> bool {
        self.installed() && self.active_modules() < self.total_modules()
    }

    fn repair(&mut self) {
        let prompts: Vec<String> = vec![
            self.module_tracking.repair_display(),
        ];

        let mut options:Vec<String> = vec![];
        if self.module_tracking.repairable() { options.push(prompts[0].clone()) }

        let chosen: String = tli_menu("Select module to repair:", options);
        match chosen {
            _ if chosen == prompts[0] => { self.module_tracking.repair(); },
            _ => unreachable!()
        }

        self.update_active_modules();
    }
}

impl PowerDown for TrackingSection {
    fn power_down(&mut self) {
        self.module_tracking.power_down();

        self.update_active_modules();
    }
}

/* Transponder Section ========================================================================== */

pub struct TransponderSection {
    _name: &'static str,
    _installed: bool,

    pub module_transponder: comms::Transponder,

    _total_modules: u16,
    _active_modules: u16,
}

impl ModulesContained for TransponderSection {
    const MODULES_CONTAINED: u16 = 1;
}

impl TransponderSection {
    pub fn new(installed: bool) -> Self {
        let mut section = TransponderSection {
            _name: "Transponder Section",
            _installed: installed,

            module_transponder: comms::Transponder::new(installed),

            _total_modules: TransponderSection::MODULES_CONTAINED,
            _active_modules: 0,
        };

        section.update_active_modules();

        return section;
    }
}

impl Name for TransponderSection { fn name(&self) -> String { self._name.to_string() } }
impl Installed for TransponderSection { fn installed(&self) -> bool { self._installed } }
impl ModuleCounts for TransponderSection {
    fn total_modules(&self) -> u16 { self._total_modules }
    fn active_modules(&self) -> u16 { self._active_modules }
}

impl UpdateModules for TransponderSection {
    fn active_module_counts(&self) -> Vec<u16> {
        vec![
            self.module_transponder.active() as u16
        ]
    }

    fn update_active_modules(&mut self) {
        self._active_modules = self.active_module_sum();
    }
}

impl Status for TransponderSection {
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
                self.module_transponder.status(indent + 2)
            ],
            indent
        )
    }
}

impl BreakSomething for TransponderSection {
    fn break_something(&mut self) -> Result<String, String> {
        if !self.installed() { return Err("not installed".to_string()); }

        let broken_module: String;

        match thread_rng().gen_range(1..=TransponderSection::MODULES_CONTAINED) {
            1 => { broken_module = self.module_transponder.break_something(); },
            _ => unreachable!(),
        }

        self.update_active_modules();

        return Ok(broken_module);
    }
}

impl Repair for TransponderSection {
    fn repairable(&self) -> bool {
        self.installed() && self.active_modules() < self.total_modules()
    }

    fn repair(&mut self) {
        let prompts: Vec<String> = vec![
            self.module_transponder.repair_display(),
        ];

        let mut options:Vec<String> = vec![];
        if self.module_transponder.repairable() { options.push(prompts[0].clone()) }

        let chosen: String = tli_menu("Select module to repair:", options);
        match chosen {
            _ if chosen == prompts[0] => { self.module_transponder.repair(); },
            _ => unreachable!()
        }

        self.update_active_modules();
    }
}

impl PowerDown for TransponderSection {
    fn power_down(&mut self) {
        self.module_transponder.power_down();

        self.update_active_modules();
    }
}
