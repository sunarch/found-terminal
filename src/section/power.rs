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
use crate::module::power;
use crate::terminalisp::station as tl_station;
use crate::terminalisp::menu::tli_menu;

// module
use crate::section::common::{ModulesContained, Installed};


/* Fossil Power Section ========================================================================= */

pub struct FossilPowerSection {
    _name: &'static str,
    _installed: bool,

    pub module_combustion_turbine_generator: power::CombustionTurbineGenerator,
    pub module_fossil_fuel_storage: power::FossilFuelStorage,

    _total_modules: u16,
    _active_modules: u16,
}

impl ModulesContained for FossilPowerSection {
    const MODULES_CONTAINED: u16 = 2;
}

impl FossilPowerSection {
    pub fn new(installed: bool) -> Self {
        let mut section = FossilPowerSection {
            _name: "Fossil Power Section",
            _installed: installed,

            module_combustion_turbine_generator: power::CombustionTurbineGenerator::new(installed),
            module_fossil_fuel_storage: power::FossilFuelStorage::new(installed),

            _total_modules: FossilPowerSection::MODULES_CONTAINED,
            _active_modules: 0,
        };

        section.update_active_modules();

        return section;
    }
}

impl Name for FossilPowerSection { fn name(&self) -> String { self._name.to_string() } }
impl Installed for FossilPowerSection { fn installed(&self) -> bool { self._installed } }
impl ModuleCounts for FossilPowerSection {
    fn total_modules(&self) -> u16 { self._total_modules }
    fn active_modules(&self) -> u16 { self._active_modules }
}

impl UpdateModules for FossilPowerSection {
    fn active_module_counts(&self) -> Vec<u16> {
        vec![
            self.module_combustion_turbine_generator.active() as u16,
            self.module_fossil_fuel_storage.active() as u16
        ]
    }

    fn update_active_modules(&mut self) {
        self._active_modules = self.active_module_sum();
    }
}

impl Status for FossilPowerSection {
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
                self.module_combustion_turbine_generator.status(indent + 2),
                self.module_fossil_fuel_storage.status(indent + 2)
            ],
            indent
        )
    }
}

impl BreakSomething for FossilPowerSection {
    fn break_something(&mut self) -> Result<String, String> {
        if !self.installed() { return Err("not installed".to_string()); }

        let broken_module: String;

        match thread_rng().gen_range(1..=FossilPowerSection::MODULES_CONTAINED) {
            1 => { broken_module = self.module_combustion_turbine_generator.break_something(); },
            2 => { broken_module = self.module_fossil_fuel_storage.break_something(); },
            _ => unreachable!(),
        }

        self.update_active_modules();

        return Ok(broken_module);
    }
}

impl Repair for FossilPowerSection {
    fn repair(&mut self) {
        let prompts: Vec<String> = vec![
            self.module_combustion_turbine_generator.repair_display(),
            self.module_fossil_fuel_storage.repair_display(),
        ];

        let mut options:Vec<String> = vec![];
        if self.module_combustion_turbine_generator.active() {
            options.push(prompts[0].clone())
        }
        if self.module_fossil_fuel_storage.active() {
            options.push(prompts[1].clone())
        }

        let chosen: String = tli_menu("Select module to repair:", options);
        match chosen {
            _ if chosen == prompts[0] => { self.module_combustion_turbine_generator.repair(); },
            _ if chosen == prompts[1] => { self.module_fossil_fuel_storage.repair(); },
            _ => unreachable!()
        }

        self.update_active_modules();
    }
}

impl PowerDown for FossilPowerSection {
    fn power_down(&mut self) {
        self.module_combustion_turbine_generator.power_down();
        self.module_fossil_fuel_storage.power_down();

        self.update_active_modules();
    }
}

/* Fusion Power Section ========================================================================= */

pub struct FusionPowerSection {
    _name: &'static str,
    _installed: bool,

    pub module_fusion_reactor: power::FusionReactor,
    pub module_steam_turbine_generator: power::SteamTurbineGenerator,
    pub module_fusion_component_storage: power::FusionComponentStorage,

    _total_modules: u16,
    _active_modules: u16,
}

impl ModulesContained for FusionPowerSection {
    const MODULES_CONTAINED: u16 = 3;
}

impl FusionPowerSection {
    pub fn new(installed: bool) -> Self {
        let mut section = FusionPowerSection {
            _name: "Fusion Power Section",
            _installed: installed,

            module_fusion_reactor: power::FusionReactor::new(installed),
            module_steam_turbine_generator: power::SteamTurbineGenerator::new(installed),
            module_fusion_component_storage: power::FusionComponentStorage::new(installed),

            _total_modules: FusionPowerSection::MODULES_CONTAINED,
            _active_modules: 0,
        };

        section.update_active_modules();

        return section;
    }
}

impl Name for FusionPowerSection { fn name(&self) -> String { self._name.to_string() } }
impl Installed for FusionPowerSection { fn installed(&self) -> bool { self._installed } }
impl ModuleCounts for FusionPowerSection {
    fn total_modules(&self) -> u16 { self._total_modules }
    fn active_modules(&self) -> u16 { self._active_modules }
}

impl UpdateModules for FusionPowerSection {
    fn active_module_counts(&self) -> Vec<u16> {
        vec![
            self.module_fusion_reactor.active() as u16,
            self.module_steam_turbine_generator.active() as u16,
            self.module_fusion_component_storage.active() as u16
        ]
    }

    fn update_active_modules(&mut self) {
        self._active_modules = self.active_module_sum();
    }
}

impl Status for FusionPowerSection {
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
                self.module_fusion_reactor.status(indent + 2),
                self.module_steam_turbine_generator.status(indent + 2),
                self.module_fusion_component_storage.status(indent + 2)
            ],
            indent
        )
    }
}

impl BreakSomething for FusionPowerSection {
    fn break_something(&mut self) -> Result<String, String> {
        if !self.installed() { return Err("not installed".to_string()); }

        let broken_module: String;

        match thread_rng().gen_range(1..=FusionPowerSection::MODULES_CONTAINED) {
            1 => { broken_module = self.module_fusion_reactor.break_something(); },
            2 => { broken_module = self.module_steam_turbine_generator.break_something(); },
            3 => { broken_module = self.module_fusion_component_storage.break_something(); },
            _ => unreachable!(),
        }

        self.update_active_modules();

        return Ok(broken_module);
    }
}

impl Repair for FusionPowerSection {
    fn repair(&mut self) {
        let prompts: Vec<String> = vec![
            self.module_fusion_reactor.repair_display(),
            self.module_steam_turbine_generator.repair_display(),
            self.module_fusion_component_storage.repair_display(),
        ];

        let mut options:Vec<String> = vec![];
        if self.module_fusion_reactor.active() {
            options.push(prompts[0].clone())
        }
        if self.module_steam_turbine_generator.active() {
            options.push(prompts[1].clone())
        }
        if self.module_fusion_component_storage.active() {
            options.push(prompts[2].clone())
        }

        let chosen: String = tli_menu("Select module to repair:", options);
        match chosen {
            _ if chosen == prompts[0] => { self.module_fusion_reactor.repair(); },
            _ if chosen == prompts[1] => { self.module_steam_turbine_generator.repair(); },
            _ if chosen == prompts[2] => { self.module_fusion_component_storage.repair(); },
            _ => unreachable!()
        }

        self.update_active_modules();
    }
}

impl PowerDown for FusionPowerSection {
    fn power_down(&mut self) {
        self.module_fusion_reactor.power_down();
        self.module_steam_turbine_generator.power_down();
        self.module_fusion_component_storage.power_down();

        self.update_active_modules();
    }
}

/* Nuclear Power Section ======================================================================== */

pub struct NuclearPowerSection {
    _name: &'static str,
    _installed: bool,

    pub module_nuclear_fuel_storage: power::NuclearFuelStorage,
    pub module_nuclear_reactor: power::NuclearReactor,
    pub module_steam_turbine_generator: power::SteamTurbineGenerator,
    pub module_nuclear_waste_storage: power::NuclearWasteStorage,

    _total_modules: u16,
    _active_modules: u16,
}

impl ModulesContained for NuclearPowerSection {
    const MODULES_CONTAINED: u16 = 4;
}

impl NuclearPowerSection {
    pub fn new(installed: bool) -> Self {
        let mut section = NuclearPowerSection {
            _name: "Nuclear Power Section",
            _installed: installed,

            module_nuclear_fuel_storage: power::NuclearFuelStorage::new(installed),
            module_nuclear_reactor: power::NuclearReactor::new(installed),
            module_steam_turbine_generator: power::SteamTurbineGenerator::new(installed),
            module_nuclear_waste_storage: power::NuclearWasteStorage::new(installed),

            _total_modules: NuclearPowerSection::MODULES_CONTAINED,
            _active_modules: 0,
        };

        section.update_active_modules();

        return section;
    }
}

impl Name for NuclearPowerSection { fn name(&self) -> String { self._name.to_string() } }
impl Installed for NuclearPowerSection { fn installed(&self) -> bool { self._installed } }
impl ModuleCounts for NuclearPowerSection {
    fn total_modules(&self) -> u16 { self._total_modules }
    fn active_modules(&self) -> u16 { self._active_modules }
}

impl UpdateModules for NuclearPowerSection {
    fn active_module_counts(&self) -> Vec<u16> {
        vec![
            self.module_nuclear_fuel_storage.active() as u16,
            self.module_nuclear_reactor.active() as u16,
            self.module_steam_turbine_generator.active() as u16,
            self.module_nuclear_waste_storage.active() as u16
        ]
    }

    fn update_active_modules(&mut self) {
        self._active_modules = self.active_module_sum();
    }
}

impl Status for NuclearPowerSection {
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
                self.module_nuclear_fuel_storage.status(indent + 2),
                self.module_nuclear_reactor.status(indent + 2),
                self.module_steam_turbine_generator.status(indent + 2),
                self.module_nuclear_waste_storage.status(indent + 2)
            ],
            indent
        )
    }
}

impl BreakSomething for NuclearPowerSection {
    fn break_something(&mut self) -> Result<String, String> {
        if !self.installed() { return Err("not installed".to_string()); }

        let broken_module: String;

        match thread_rng().gen_range(1..=NuclearPowerSection::MODULES_CONTAINED) {
            1 => { broken_module = self.module_nuclear_fuel_storage.break_something(); },
            2 => { broken_module = self.module_nuclear_reactor.break_something(); },
            3 => { broken_module = self.module_steam_turbine_generator.break_something(); },
            4 => { broken_module = self.module_nuclear_waste_storage.break_something(); },
            _ => unreachable!(),
        }

        self.update_active_modules();

        return Ok(broken_module);
    }
}

impl Repair for NuclearPowerSection {
    fn repair(&mut self) {
        let prompts: Vec<String> = vec![
            self.module_nuclear_fuel_storage.repair_display(),
            self.module_nuclear_reactor.repair_display(),
            self.module_steam_turbine_generator.repair_display(),
            self.module_nuclear_waste_storage.repair_display(),
        ];

        let mut options:Vec<String> = vec![];
        if self.module_nuclear_fuel_storage.active() {
            options.push(prompts[0].clone())
        }
        if self.module_nuclear_reactor.active() {
            options.push(prompts[1].clone())
        }
        if self.module_steam_turbine_generator.active() {
            options.push(prompts[2].clone())
        }
        if self.module_nuclear_waste_storage.active() {
            options.push(prompts[3].clone())
        }

        let chosen: String = tli_menu("Select module to repair:", options);
        match chosen {
            _ if chosen == prompts[0] => { self.module_nuclear_fuel_storage.repair(); },
            _ if chosen == prompts[1] => { self.module_nuclear_reactor.repair(); },
            _ if chosen == prompts[2] => { self.module_steam_turbine_generator.repair(); },
            _ if chosen == prompts[3] => { self.module_nuclear_waste_storage.repair(); },
            _ => unreachable!()
        }

        self.update_active_modules();
    }
}

impl PowerDown for NuclearPowerSection {
    fn power_down(&mut self) {
        self.module_nuclear_fuel_storage.power_down();
        self.module_nuclear_reactor.power_down();
        self.module_steam_turbine_generator.power_down();
        self.module_nuclear_waste_storage.power_down();

        self.update_active_modules();
    }
}

/* Radiation Power Section ====================================================================== */

pub struct RadiationPowerSection {
    _name: &'static str,
    _installed: bool,

    pub module_radiation_mirrors: power::RadiationMirrors,

    _total_modules: u16,
    _active_modules: u16,
}

impl ModulesContained for RadiationPowerSection {
    const MODULES_CONTAINED: u16 = 1;
}

impl RadiationPowerSection {
    pub fn new(installed: bool) -> Self {
        let mut section = RadiationPowerSection {
            _name: "Radiation Power Section",
            _installed: installed,

            module_radiation_mirrors: power::RadiationMirrors::new(installed),

            _total_modules: RadiationPowerSection::MODULES_CONTAINED,
            _active_modules: 0,
        };

        section.update_active_modules();

        return section;
    }
}

impl Name for RadiationPowerSection { fn name(&self) -> String { self._name.to_string() } }
impl Installed for RadiationPowerSection { fn installed(&self) -> bool { self._installed } }
impl ModuleCounts for RadiationPowerSection {
    fn total_modules(&self) -> u16 { self._total_modules }
    fn active_modules(&self) -> u16 { self._active_modules }
}

impl UpdateModules for RadiationPowerSection {
    fn active_module_counts(&self) -> Vec<u16> {
        vec![
            self.module_radiation_mirrors.active() as u16
        ]
    }

    fn update_active_modules(&mut self) {
        self._active_modules = self.active_module_sum();
    }
}

impl Status for RadiationPowerSection {
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
                self.module_radiation_mirrors.status(indent + 2)
            ],
            indent
        )
    }
}

impl BreakSomething for RadiationPowerSection {
    fn break_something(&mut self) -> Result<String, String> {
        if !self.installed() { return Err("not installed".to_string()); }

        let broken_module: String;

        match thread_rng().gen_range(1..=RadiationPowerSection::MODULES_CONTAINED) {
            1 => { broken_module = self.module_radiation_mirrors.break_something(); },
            _ => unreachable!(),
        }

        self.update_active_modules();

        return Ok(broken_module);
    }
}

impl Repair for RadiationPowerSection {
    fn repair(&mut self) {
        let prompts: Vec<String> = vec![
            self.module_radiation_mirrors.repair_display(),
        ];

        let mut options:Vec<String> = vec![];
        if self.module_radiation_mirrors.active() {
            options.push(prompts[0].clone())
        }

        let chosen: String = tli_menu("Select module to repair:", options);
        match chosen {
            _ if chosen == prompts[0] => { self.module_radiation_mirrors.repair(); },
            _ => unreachable!()
        }

        self.update_active_modules();
    }
}

impl PowerDown for RadiationPowerSection {
    fn power_down(&mut self) {
        self.module_radiation_mirrors.power_down();

        self.update_active_modules();
    }
}

/* Solar Power Section ========================================================================== */

pub struct SolarPowerSection {
    _name: &'static str,
    _installed: bool,

    pub module_solar_panel: power::SolarPanels,

    _total_modules: u16,
    _active_modules: u16,
}

impl ModulesContained for SolarPowerSection {
    const MODULES_CONTAINED: u16 = 1;
}

impl SolarPowerSection {
    pub fn new(installed: bool) -> Self {
        let mut section = SolarPowerSection {
            _name: "Solar Power Section",
            _installed: installed,

            module_solar_panel: power::SolarPanels::new(installed),

            _total_modules: SolarPowerSection::MODULES_CONTAINED,
            _active_modules: 0,
        };

        section.update_active_modules();

        return section;
    }
}

impl Name for SolarPowerSection { fn name(&self) -> String { self._name.to_string() } }
impl Installed for SolarPowerSection { fn installed(&self) -> bool { self._installed } }
impl ModuleCounts for SolarPowerSection {
    fn total_modules(&self) -> u16 { self._total_modules }
    fn active_modules(&self) -> u16 { self._active_modules }
}

impl UpdateModules for SolarPowerSection {
    fn active_module_counts(&self) -> Vec<u16> {
        vec![
            self.module_solar_panel.active() as u16
        ]
    }

    fn update_active_modules(&mut self) {
        self._active_modules = self.active_module_sum();
    }
}

impl Status for SolarPowerSection {
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
                self.module_solar_panel.status(indent + 2)
            ],
            indent
        )
    }
}

impl BreakSomething for SolarPowerSection {
    fn break_something(&mut self) -> Result<String, String> {
        if !self.installed() { return Err("not installed".to_string()); }

        let broken_module: String;

        match thread_rng().gen_range(1..=SolarPowerSection::MODULES_CONTAINED) {
            1 => { broken_module = self.module_solar_panel.break_something(); },
            _ => unreachable!(),
        }

        self.update_active_modules();

        return Ok(broken_module);
    }
}

impl Repair for SolarPowerSection {
    fn repair(&mut self) {
        let prompts: Vec<String> = vec![
            self.module_solar_panel.repair_display(),
        ];

        let mut options:Vec<String> = vec![];
        if self.module_solar_panel.active() {
            options.push(prompts[0].clone())
        }

        let chosen: String = tli_menu("Select module to repair:", options);
        match chosen {
            _ if chosen == prompts[0] => { self.module_solar_panel.repair(); },
            _ => unreachable!()
        }

        self.update_active_modules();
    }
}

impl PowerDown for SolarPowerSection {
    fn power_down(&mut self) {
        self.module_solar_panel.power_down();

        self.update_active_modules();
    }
}
