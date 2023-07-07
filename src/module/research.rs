// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

// project
use crate::station::components::Name;

// module
use crate::module::common::{Active, StatusModule, BreakModule, RepairModule, PowerDownModule};

/* Astronomy Lab ================================================================================ */

pub struct AstronomyLab {
    _name: &'static str,
    _active: bool,
}

impl AstronomyLab {
    pub fn new(initial: bool) -> Self {
        AstronomyLab {
            _name: "Astronomy Lab",
            _active: initial,
        }
    }
}

impl Name for AstronomyLab { fn name(&self) -> String { self._name.to_string() } }
impl Active for AstronomyLab {
    fn active(&self) -> bool { self._active }
    fn activate(&mut self) { self._active = true; }
    fn deactivate(&mut self) { self._active = false; }
}

impl StatusModule for AstronomyLab {}
impl BreakModule for AstronomyLab {}
impl RepairModule for AstronomyLab {}
impl PowerDownModule for AstronomyLab {}

/* Greenhouse =================================================================================== */

pub struct Greenhouse {
    _name: &'static str,
    _active: bool,
}

impl Greenhouse {
    pub fn new(initial: bool) -> Self {
        Greenhouse {
            _name: "Greenhouse",
            _active: initial,
        }
    }
}

impl Name for Greenhouse { fn name(&self) -> String { self._name.to_string() } }
impl Active for Greenhouse {
    fn active(&self) -> bool { self._active }
    fn activate(&mut self) { self._active = true; }
    fn deactivate(&mut self) { self._active = false; }
}

impl StatusModule for Greenhouse {}
impl BreakModule for Greenhouse {}
impl RepairModule for Greenhouse {}
impl PowerDownModule for Greenhouse {}

/* Mainframe ==================================================================================== */

pub struct Mainframe {
    _name: &'static str,
    _active: bool,
}

impl Mainframe {
    pub fn new(initial: bool) -> Self {
        Mainframe {
            _name: "Mainframe",
            _active: initial,
        }
    }
}

impl Name for Mainframe { fn name(&self) -> String { self._name.to_string() } }
impl Active for Mainframe {
    fn active(&self) -> bool { self._active }
    fn activate(&mut self) { self._active = true; }
    fn deactivate(&mut self) { self._active = false; }
}

impl StatusModule for Mainframe {}
impl BreakModule for Mainframe {}
impl RepairModule for Mainframe {}
impl PowerDownModule for Mainframe {}

/* Weather Observation ========================================================================== */

pub struct WeatherObservation {
    _name: &'static str,
    _active: bool,
}

impl WeatherObservation {
    pub fn new(initial: bool) -> Self {
        WeatherObservation {
            _name: "Weather Observation",
            _active: initial,
        }
    }
}

impl Name for WeatherObservation { fn name(&self) -> String { self._name.to_string() } }
impl Active for WeatherObservation {
    fn active(&self) -> bool { self._active }
    fn activate(&mut self) { self._active = true; }
    fn deactivate(&mut self) { self._active = false; }
}

impl StatusModule for WeatherObservation {}
impl BreakModule for WeatherObservation {}
impl RepairModule for WeatherObservation {}
impl PowerDownModule for WeatherObservation {}
