// -*- coding: utf-8, vim: expandtab:ts=4 -*-

// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

pub trait Name {
    fn name(&self) -> String;
}

pub trait SectionCounts {
    fn total_sections(&self) -> u16;
    fn installed_sections(&self) -> u16;
}

pub trait ModuleCounts: Name {
    fn total_modules(&self) -> u16;
    fn active_modules(&self) -> u16;

    fn repair_display(&self) -> String {
        format!("{} ({}/{})",
                self.name(),
                self.active_modules(),
                self.total_modules())
    }
}

pub trait UpdateModules {
    fn active_module_counts(&self) -> Vec<u16>;
    fn update_active_modules(&mut self);

    fn active_module_sum(&self) -> u16 {
        self.active_module_counts()
            .iter()
            .fold(0, |acc, e| acc + e)
    }
}

pub trait Status {
    fn status(&self, indent: u8) -> String;
}

pub trait BreakSomething {
    fn break_something(&mut self) -> Result<String, String>;
}

pub trait Repair {
    fn repairable(&self) -> bool;
    fn repair(&mut self);
}

pub trait PowerDown {
    fn power_down(&mut self);
}
