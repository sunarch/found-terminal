// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

// project
use crate::station::components::Name;

// module
use crate::module::common::{Active, StatusModule, BreakModule, RepairModule, PowerDownModule};

/* Airlock ====================================================================================== */


pub struct Airlock {
    _name: &'static str,
    _active: bool,
}

impl Airlock {
    pub fn new(initial: bool) -> Self {
        Airlock {
            _name: "Airlock",
            _active: initial,
        }
    }
}

impl Name for Airlock { fn name(&self) -> String { self._name.to_string() } }
impl Active for Airlock {
    fn active(&self) -> bool { self._active }
    fn activate(&mut self) { self._active = true; }
    fn deactivate(&mut self) { self._active = false; }
}

impl StatusModule for Airlock {}
impl BreakModule for Airlock {}
impl RepairModule for Airlock {}
impl PowerDownModule for Airlock {}

/* Cargo Bay ==================================================================================== */

pub struct CargoBay {
    _name: &'static str,
    _active: bool,
}

impl CargoBay {
    pub fn new(initial: bool) -> Self {
        CargoBay {
            _name: "Cargo Bay",
            _active: initial,
        }
    }
}

impl Name for CargoBay { fn name(&self) -> String { self._name.to_string() } }
impl Active for CargoBay {
    fn active(&self) -> bool { self._active }
    fn activate(&mut self) { self._active = true; }
    fn deactivate(&mut self) { self._active = false; }
}

impl StatusModule for CargoBay {}
impl BreakModule for CargoBay {}
impl RepairModule for CargoBay {}
impl PowerDownModule for CargoBay {}

/* Temperature Control ========================================================================== */

pub struct TemperatureControl {
    _name: &'static str,
    _active: bool,
}

impl TemperatureControl {
    pub fn new(initial: bool) -> Self {
        TemperatureControl {
            _name: "Temperature Control",
            _active: initial,
        }
    }
}

impl Name for TemperatureControl { fn name(&self) -> String { self._name.to_string() } }
impl Active for TemperatureControl {
    fn active(&self) -> bool { self._active }
    fn activate(&mut self) { self._active = true; }
    fn deactivate(&mut self) { self._active = false; }
}

impl StatusModule for TemperatureControl {}
impl BreakModule for TemperatureControl {}
impl RepairModule for TemperatureControl {}
impl PowerDownModule for TemperatureControl {}
