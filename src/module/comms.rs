// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

// project
use crate::station::components::Name;

// module
use crate::module::common::{Active, StatusModule, BreakModule, RepairModule, PowerDownModule};

/* Antenna ====================================================================================== */

pub struct Antenna {
    _name: &'static str,
    _active: bool,
}

impl Antenna {
    pub fn new(initial: bool) -> Self {
        Antenna {
            _name: "Antenna",
            _active: initial,
        }
    }
}

impl Name for Antenna { fn name(&self) -> String { self._name.to_string() } }
impl Active for Antenna {
    fn active(&self) -> bool { self._active }
    fn activate(&mut self) { self._active = true; }
    fn deactivate(&mut self) { self._active = false; }
}

impl StatusModule for Antenna {}
impl BreakModule for Antenna {}
impl RepairModule for Antenna {}
impl PowerDownModule for Antenna {}

/* Tracking ===================================================================================== */

pub struct Tracking {
    _name: &'static str,
    _active: bool,
}

impl Tracking {
    pub fn new(initial: bool) -> Self {
        Tracking {
            _name: "Tracking",
            _active: initial,
        }
    }
}

impl Name for Tracking { fn name(&self) -> String { self._name.to_string() } }
impl Active for Tracking {
    fn active(&self) -> bool { self._active }
    fn activate(&mut self) { self._active = true; }
    fn deactivate(&mut self) { self._active = false; }
}

impl StatusModule for Tracking {}
impl BreakModule for Tracking {}
impl RepairModule for Tracking {}
impl PowerDownModule for Tracking {}

/* Transponder ================================================================================== */

pub struct Transponder {
    _name: &'static str,
    _active: bool,
}

impl Transponder {
    pub fn new(initial: bool) -> Self {
        Transponder {
            _name: "Transponder",
            _active: initial,
        }
    }
}

impl Name for Transponder { fn name(&self) -> String { self._name.to_string() } }
impl Active for Transponder {
    fn active(&self) -> bool { self._active }
    fn activate(&mut self) { self._active = true; }
    fn deactivate(&mut self) { self._active = false; }
}

impl StatusModule for Transponder {}
impl BreakModule for Transponder {}
impl RepairModule for Transponder {}
impl PowerDownModule for Transponder {}
