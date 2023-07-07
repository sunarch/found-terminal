// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

// project
use crate::station::components::Name;

// module
use crate::module::common::{Active, StatusModule, BreakModule, RepairModule, PowerDownModule};

/* Command Module =============================================================================== */

pub struct CommandModule {
    _name: &'static str,
    _active: bool,
}

impl CommandModule {
    pub fn new(initial: bool) -> Self {
        CommandModule {
            _name: "Command Module",
            _active: initial,
        }
    }
}

impl Name for CommandModule { fn name(&self) -> String { self._name.to_string() } }
impl Active for CommandModule {
    fn active(&self) -> bool { self._active }
    fn activate(&mut self) { self._active = true; }
    fn deactivate(&mut self) { self._active = false; }
}

impl StatusModule for CommandModule {}
impl BreakModule for CommandModule {}
impl RepairModule for CommandModule {}
impl PowerDownModule for CommandModule {}

/* Galley ======================================================================================= */

pub struct Galley {
    _name: &'static str,
    _active: bool,
}

impl Galley {
    pub fn new(initial: bool) -> Self {
        Galley {
            _name: "Galley",
            _active: initial,
        }
    }
}

impl Name for Galley { fn name(&self) -> String { self._name.to_string() } }
impl Active for Galley {
    fn active(&self) -> bool { self._active }
    fn activate(&mut self) { self._active = true; }
    fn deactivate(&mut self) { self._active = false; }
}

impl StatusModule for Galley {}
impl BreakModule for Galley {}
impl RepairModule for Galley {}
impl PowerDownModule for Galley {}

/* Life Support ================================================================================= */

pub struct LifeSupport {
    _name: &'static str,
    _active: bool,
}

impl LifeSupport {
    pub fn new(initial: bool) -> Self {
        LifeSupport {
            _name: "Life Support",
            _active: initial,
        }
    }
}

impl Name for LifeSupport { fn name(&self) -> String { self._name.to_string() } }
impl Active for LifeSupport {
    fn active(&self) -> bool { self._active }
    fn activate(&mut self) { self._active = true; }
    fn deactivate(&mut self) { self._active = false; }
}

impl StatusModule for LifeSupport {}
impl BreakModule for LifeSupport {}
impl RepairModule for LifeSupport {}
impl PowerDownModule for LifeSupport {}

/* Sleeping Pods ================================================================================ */

pub struct SleepingPods {
    _name: &'static str,
    _active: bool,
}

impl SleepingPods {
    pub fn new(initial: bool) -> Self {
        SleepingPods {
            _name: "Sleeping Pods",
            _active: initial,
        }
    }
}

impl Name for SleepingPods { fn name(&self) -> String { self._name.to_string() } }
impl Active for SleepingPods {
    fn active(&self) -> bool { self._active }
    fn activate(&mut self) { self._active = true; }
    fn deactivate(&mut self) { self._active = false; }
}

impl StatusModule for SleepingPods {}
impl BreakModule for SleepingPods {}
impl RepairModule for SleepingPods {}
impl PowerDownModule for SleepingPods {}

/* Space Suits ================================================================================== */

pub struct SpaceSuits {
    _name: &'static str,
    _active: bool,
}

impl SpaceSuits {
    pub fn new(initial: bool) -> Self {
        SpaceSuits {
            _name: "Space Suits",
            _active: initial,
        }
    }
}

impl Name for SpaceSuits { fn name(&self) -> String { self._name.to_string() } }
impl Active for SpaceSuits {
    fn active(&self) -> bool { self._active }
    fn activate(&mut self) { self._active = true; }
    fn deactivate(&mut self) { self._active = false; }
}

impl StatusModule for SpaceSuits {}
impl BreakModule for SpaceSuits {}
impl RepairModule for SpaceSuits {}
impl PowerDownModule for SpaceSuits {}

/* Water Reclamation ============================================================================ */


pub struct WaterReclamation {
    _name: &'static str,
    _active: bool,
}

impl WaterReclamation {
    pub fn new(initial: bool) -> Self {
        WaterReclamation {
            _name: "Water Reclamation",
            _active: initial,
        }
    }
}

impl Name for WaterReclamation { fn name(&self) -> String { self._name.to_string() } }
impl Active for WaterReclamation {
    fn active(&self) -> bool { self._active }
    fn activate(&mut self) { self._active = true; }
    fn deactivate(&mut self) { self._active = false; }
}

impl StatusModule for WaterReclamation {}
impl BreakModule for WaterReclamation {}
impl RepairModule for WaterReclamation {}
impl PowerDownModule for WaterReclamation {}
