// -*- coding: utf-8, vim: expandtab:ts=4 -*-

// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

// project
use crate::station::components::Name;

// module
use crate::module::common::{Active, StatusModule, BreakModule, RepairModule, PowerDownModule};

/* Fusion Component Storage ===================================================================== */

pub struct FusionComponentStorage {
    _name: &'static str,
    _active: bool,
}

impl FusionComponentStorage {
    pub fn new(initial: bool) -> Self {
        FusionComponentStorage {
            _name: "Fusion Component Storage",
            _active: initial,
        }
    }
}

impl Name for FusionComponentStorage { fn name(&self) -> String { self._name.to_string() } }
impl Active for FusionComponentStorage {
    fn active(&self) -> bool { self._active }
    fn activate(&mut self) { self._active = true; }
    fn deactivate(&mut self) { self._active = false; }
}

impl StatusModule for FusionComponentStorage {}
impl BreakModule for FusionComponentStorage {}
impl RepairModule for FusionComponentStorage {}
impl PowerDownModule for FusionComponentStorage {}

/* Fusion Reactor =============================================================================== */

pub struct FusionReactor {
    _name: &'static str,
    _active: bool,
}

impl FusionReactor {
    pub fn new(initial: bool) -> Self {
        FusionReactor {
            _name: "Fusion Reactor",
            _active: initial,
        }
    }
}

impl Name for FusionReactor { fn name(&self) -> String { self._name.to_string() } }
impl Active for FusionReactor {
    fn active(&self) -> bool { self._active }
    fn activate(&mut self) { self._active = true; }
    fn deactivate(&mut self) { self._active = false; }
}

impl StatusModule for FusionReactor {}
impl BreakModule for FusionReactor {}
impl RepairModule for FusionReactor {}
impl PowerDownModule for FusionReactor {}

/* Combustion Turbine Generator ================================================================= */

pub struct CombustionTurbineGenerator {
    _name: &'static str,
    _active: bool,
}

impl CombustionTurbineGenerator {
    pub fn new(initial: bool) -> Self {
        CombustionTurbineGenerator {
            _name: "Combustion Turbine Generator",
            _active: initial,
        }
    }
}

impl Name for CombustionTurbineGenerator { fn name(&self) -> String { self._name.to_string() } }
impl Active for CombustionTurbineGenerator {
    fn active(&self) -> bool { self._active }
    fn activate(&mut self) { self._active = true; }
    fn deactivate(&mut self) { self._active = false; }
}

impl StatusModule for CombustionTurbineGenerator {}
impl BreakModule for CombustionTurbineGenerator {}
impl RepairModule for CombustionTurbineGenerator {}
impl PowerDownModule for CombustionTurbineGenerator {}

/* Fossil Fuel Storage ========================================================================== */

pub struct FossilFuelStorage {
    _name: &'static str,
    _active: bool,
}

impl FossilFuelStorage {
    pub fn new(initial: bool) -> Self {
        FossilFuelStorage {
            _name: "Fossil Fuel Storage",
            _active: initial,
        }
    }
}

impl Name for FossilFuelStorage { fn name(&self) -> String { self._name.to_string() } }
impl Active for FossilFuelStorage {
    fn active(&self) -> bool { self._active }
    fn activate(&mut self) { self._active = true; }
    fn deactivate(&mut self) { self._active = false; }
}

impl StatusModule for FossilFuelStorage {}
impl BreakModule for FossilFuelStorage {}
impl RepairModule for FossilFuelStorage {}
impl PowerDownModule for FossilFuelStorage {}

/* Nuclear Fuel Storage ========================================================================= */

pub struct NuclearFuelStorage {
    _name: &'static str,
    _active: bool,
}

impl NuclearFuelStorage {
    pub fn new(initial: bool) -> Self {
        NuclearFuelStorage {
            _name: "Nuclear Fuel Storage",
            _active: initial,
        }
    }
}

impl Name for NuclearFuelStorage { fn name(&self) -> String { self._name.to_string() } }
impl Active for NuclearFuelStorage {
    fn active(&self) -> bool { self._active }
    fn activate(&mut self) { self._active = true; }
    fn deactivate(&mut self) { self._active = false; }
}

impl StatusModule for NuclearFuelStorage {}
impl BreakModule for NuclearFuelStorage {}
impl RepairModule for NuclearFuelStorage {}
impl PowerDownModule for NuclearFuelStorage {}

/* Nuclear Reactor ============================================================================== */

pub struct NuclearReactor {
    _name: &'static str,
    _active: bool,
}

impl NuclearReactor {
    pub fn new(initial: bool) -> Self {
        NuclearReactor {
            _name: "Nuclear Reactor",
            _active: initial,
        }
    }
}

impl Name for NuclearReactor { fn name(&self) -> String { self._name.to_string() } }
impl Active for NuclearReactor {
    fn active(&self) -> bool { self._active }
    fn activate(&mut self) { self._active = true; }
    fn deactivate(&mut self) { self._active = false; }
}

impl StatusModule for NuclearReactor {}
impl BreakModule for NuclearReactor {}
impl RepairModule for NuclearReactor {}
impl PowerDownModule for NuclearReactor {}

/* Nuclear Waste Storage ======================================================================== */

pub struct NuclearWasteStorage {
    _name: &'static str,
    _active: bool,
}

impl NuclearWasteStorage {
    pub fn new(initial: bool) -> Self {
        NuclearWasteStorage {
            _name: "Nuclear Waste Storage",
            _active: initial,
        }
    }
}

impl Name for NuclearWasteStorage { fn name(&self) -> String { self._name.to_string() } }
impl Active for NuclearWasteStorage {
    fn active(&self) -> bool { self._active }
    fn activate(&mut self) { self._active = true; }
    fn deactivate(&mut self) { self._active = false; }
}

impl StatusModule for NuclearWasteStorage {}
impl BreakModule for NuclearWasteStorage {}
impl RepairModule for NuclearWasteStorage {}
impl PowerDownModule for NuclearWasteStorage {}

/* Radiation Mirrors ============================================================================ */

pub struct RadiationMirrors {
    _name: &'static str,
    _active: bool,
}

impl RadiationMirrors {
    pub fn new(initial: bool) -> Self {
        RadiationMirrors {
            _name: "Radiation Mirrors",
            _active: initial,
        }
    }
}

impl Name for RadiationMirrors { fn name(&self) -> String { self._name.to_string() } }
impl Active for RadiationMirrors {
    fn active(&self) -> bool { self._active }
    fn activate(&mut self) { self._active = true; }
    fn deactivate(&mut self) { self._active = false; }
}

impl StatusModule for RadiationMirrors {}
impl BreakModule for RadiationMirrors {}
impl RepairModule for RadiationMirrors {}
impl PowerDownModule for RadiationMirrors {}

/* Solar Panels ================================================================================= */

pub struct SolarPanels {
    _name: &'static str,
    _active: bool,
}

impl SolarPanels {
    pub fn new(initial: bool) -> Self {
        SolarPanels {
            _name: "Solar Panels",
            _active: initial,
        }
    }
}

impl Name for SolarPanels { fn name(&self) -> String { self._name.to_string() } }
impl Active for SolarPanels {
    fn active(&self) -> bool { self._active }
    fn activate(&mut self) { self._active = true; }
    fn deactivate(&mut self) { self._active = false; }
}

impl StatusModule for SolarPanels {}
impl BreakModule for SolarPanels {}
impl RepairModule for SolarPanels {}
impl PowerDownModule for SolarPanels {}

/* Steam Turbine Generator ====================================================================== */

pub struct SteamTurbineGenerator {
    _name: &'static str,
    _active: bool,
}

impl SteamTurbineGenerator {
    pub fn new(initial: bool) -> Self {
        SteamTurbineGenerator {
            _name: "Steam Turbine Generator",
            _active: initial,
        }
    }
}

impl Name for SteamTurbineGenerator { fn name(&self) -> String { self._name.to_string() } }
impl Active for SteamTurbineGenerator {
    fn active(&self) -> bool { self._active }
    fn activate(&mut self) { self._active = true; }
    fn deactivate(&mut self) { self._active = false; }
}

impl StatusModule for SteamTurbineGenerator {}
impl BreakModule for SteamTurbineGenerator {}
impl RepairModule for SteamTurbineGenerator {}
impl PowerDownModule for SteamTurbineGenerator {}
