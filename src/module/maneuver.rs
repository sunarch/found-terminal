// -*- coding: utf-8, vim: expandtab:ts=4 -*-

// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

// project
use crate::station::components::Name;

// module
use crate::module::common::{Active, StatusModule, BreakModule, RepairModule, PowerDownModule};

/* Docking System =============================================================================== */

pub struct DockingSystem {
    _name: &'static str,
    _active: bool,
}

impl DockingSystem {
    pub fn new(initial: bool) -> Self {
        DockingSystem {
            _name: "Docking System",
            _active: initial,
        }
    }
}

impl Name for DockingSystem { fn name(&self) -> String { self._name.to_string() } }
impl Active for DockingSystem {
    fn active(&self) -> bool { self._active }
    fn activate(&mut self) { self._active = true; }
    fn deactivate(&mut self) { self._active = false; }
}

impl StatusModule for DockingSystem {}
impl BreakModule for DockingSystem {}
impl RepairModule for DockingSystem {}
impl PowerDownModule for DockingSystem {}

/* Reaction Control System ====================================================================== */

pub struct ReactionControlSystem {
    _name: &'static str,
    _active: bool,
}

impl ReactionControlSystem {
    pub fn new(initial: bool) -> Self {
        ReactionControlSystem {
            _name: "Reaction Control System",
            _active: initial,
        }
    }
}

impl Name for ReactionControlSystem { fn name(&self) -> String { self._name.to_string() } }
impl Active for ReactionControlSystem {
    fn active(&self) -> bool { self._active }
    fn activate(&mut self) { self._active = true; }
    fn deactivate(&mut self) { self._active = false; }
}

impl StatusModule for ReactionControlSystem {}
impl BreakModule for ReactionControlSystem {}
impl RepairModule for ReactionControlSystem {}
impl PowerDownModule for ReactionControlSystem {}
