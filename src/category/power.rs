// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

// dependencies
use rand::Rng;
use rand::thread_rng;

// project
use crate::station::components::{Name, SectionCounts, ModuleCounts,
                                 UpdateModules, Status, BreakSomething, Repair, PowerDown};
use crate::section::power;
use crate::section::common::Installed;
use crate::terminalisp::station as tl_station;
use crate::terminalisp::menu::tli_menu;

// module
use crate::category::common::{SectionsAvailable, random_bools};


pub struct PowerCategory {
    _name: &'static str,

    pub section_fossil_power: power::FossilPowerSection,
    pub section_fusion_power: power::FusionPowerSection,
    pub section_nuclear_power: power::NuclearPowerSection,
    pub section_radiation_power: power::RadiationPowerSection,
    pub section_solar_power: power::SolarPowerSection,

    _total_sections: u16,
    _installed_sections: u16,
    _total_modules: u16,
    _active_modules: u16,
}

impl SectionsAvailable for PowerCategory {
    const SECTIONS_AVAILABLE: u16 = 5;
}

impl PowerCategory {
    pub fn new(min_count: u16, max_count: u16) -> Self {
        let installation: Vec<bool> = random_bools(PowerCategory::SECTIONS_AVAILABLE, min_count, max_count);

        let mut section_group = PowerCategory {
            _name: "Power Category",

            section_fossil_power: power::FossilPowerSection::new(installation[0]),
            section_fusion_power: power::FusionPowerSection::new(installation[1]),
            section_nuclear_power: power::NuclearPowerSection::new(installation[2]),
            section_radiation_power: power::RadiationPowerSection::new(installation[3]),
            section_solar_power: power::SolarPowerSection::new(installation[4]),

            _total_sections: PowerCategory::SECTIONS_AVAILABLE,
            _installed_sections: installation.iter().filter(|x| x == &&true).count() as u16,
            _total_modules: 0,
            _active_modules: 0,
        };

        if section_group.section_fossil_power.installed() {
            section_group._total_modules += section_group.section_fossil_power.total_modules();
        }
        if section_group.section_fusion_power.installed() {
            section_group._total_modules += section_group.section_fusion_power.total_modules();
        }
        if section_group.section_nuclear_power.installed() {
            section_group._total_modules += section_group.section_nuclear_power.total_modules();
        }
        if section_group.section_radiation_power.installed() {
            section_group._total_modules += section_group.section_radiation_power.total_modules();
        }
        if section_group.section_solar_power.installed() {
            section_group._total_modules += section_group.section_solar_power.total_modules();
        }

        section_group.update_active_modules();

        return section_group;
    }
}

impl Name for PowerCategory { fn name(&self) -> String { self._name.to_string() } }
impl SectionCounts for PowerCategory {
    fn total_sections(&self) -> u16 { self._total_sections }
    fn installed_sections(&self) -> u16 { self._installed_sections }
}
impl ModuleCounts for PowerCategory {
    fn total_modules(&self) -> u16 { self._total_modules }
    fn active_modules(&self) -> u16 { self._active_modules }
}

impl UpdateModules for PowerCategory {
    fn active_module_counts(&self) -> Vec<u16> {
        vec![
            self.section_fossil_power.active_modules(),
            self.section_fusion_power.active_modules(),
            self.section_nuclear_power.active_modules(),
            self.section_radiation_power.active_modules(),
            self.section_solar_power.active_modules()
        ]
    }

    fn update_active_modules(&mut self) {
        self._active_modules = self.active_module_sum();
    }
}

impl Status for PowerCategory {
    fn status(&self, indent: u8) -> String {
        let mut modules: Vec<String> = vec![];

        if self.section_fossil_power.installed() {
            modules.push(self.section_fossil_power.status(indent + 2));
        }
        if self.section_fusion_power.installed() {
            modules.push(self.section_fusion_power.status(indent + 2));
        }
        if self.section_nuclear_power.installed() {
            modules.push(self.section_nuclear_power.status(indent + 2));
        }
        if self.section_radiation_power.installed() {
            modules.push(self.section_radiation_power.status(indent + 2));
        }
        if self.section_solar_power.installed() {
            modules.push(self.section_solar_power.status(indent + 2));
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

impl BreakSomething for PowerCategory {
    fn break_something(&mut self) -> Result<String, String> {
        let broken_module: Result<String, String>;

        match thread_rng().gen_range(1..=PowerCategory::SECTIONS_AVAILABLE) {
            1 => { broken_module = self.section_fossil_power.break_something(); },
            2 => { broken_module = self.section_fusion_power.break_something(); },
            3 => { broken_module = self.section_nuclear_power.break_something(); },
            4 => { broken_module = self.section_radiation_power.break_something(); },
            5 => { broken_module = self.section_solar_power.break_something(); },
            _ => unreachable!(),
        }

        self.update_active_modules();

        match broken_module {
            Ok(v) => { Ok(v) },
            Err(e) => { Err(e) },
        }
    }
}

impl Repair for PowerCategory {
    fn repairable(&self) -> bool {
        self.active_modules() < self.total_modules()
    }

    fn repair(&mut self) {
        let prompts: Vec<String> = vec![
            self.section_fossil_power.repair_display(),
            self.section_fusion_power.repair_display(),
            self.section_nuclear_power.repair_display(),
            self.section_radiation_power.repair_display(),
            self.section_solar_power.repair_display(),
        ];

        let mut options:Vec<String> = vec![];
        if self.section_fossil_power.repairable()    { options.push(prompts[0].clone()) }
        if self.section_fusion_power.repairable()    { options.push(prompts[1].clone()) }
        if self.section_nuclear_power.repairable()   { options.push(prompts[2].clone()) }
        if self.section_radiation_power.repairable() { options.push(prompts[3].clone()) }
        if self.section_solar_power.repairable()     { options.push(prompts[4].clone()) }

        let chosen: String;
        match tli_menu("Select section to repair:", options) {
            Ok(v) => { chosen = v; },
            Err(_) => { return; }
        }
        match chosen {
            _ if chosen == prompts[0] => { self.section_fossil_power.repair(); },
            _ if chosen == prompts[1] => { self.section_fusion_power.repair(); },
            _ if chosen == prompts[2] => { self.section_nuclear_power.repair(); },
            _ if chosen == prompts[3] => { self.section_radiation_power.repair(); },
            _ if chosen == prompts[4] => { self.section_solar_power.repair(); },
            _ => unreachable!()
        }

        self.update_active_modules();
    }
}

impl PowerDown for PowerCategory {
    fn power_down(&mut self) {
        self.section_fossil_power.power_down();
        self.section_fusion_power.power_down();
        self.section_nuclear_power.power_down();
        self.section_radiation_power.power_down();
        self.section_solar_power.power_down();

        self.update_active_modules();
    }
}
