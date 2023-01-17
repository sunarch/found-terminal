// -*- coding: utf-8, vim: expandtab:ts=4 -*-

// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

// project
use crate::station::components::Name;
use crate::terminalisp::station as tl_station;
use crate::terminalisp::symbols;


pub trait Active: Name {
    fn active(&self) -> bool;
    fn activate(&mut self);
    fn deactivate(&mut self);
}

pub trait StatusModule: Active {
    fn status(&self, indent: u8) -> String {
        let status = if self.active() { symbols::OK } else { symbols::INACTIVE };

        tl_station::status(
            String::from("module"),
            true,
            vec![
                String::from(":name"),
                String::from(":status")
            ],
            vec![
                format!("\"{}\"", self.name()),
                String::from(status)
            ],
            false,
            String::from(""),
            vec![],
            indent
        )
    }
}

pub trait BreakModule: Active {
    fn break_something(&mut self) -> String {
        self.deactivate();
        return self.name().to_string();
    }
}

pub trait RepairModule: Active {
    fn repairable(&self) -> bool {
        !self.active()
    }

    fn repair(&mut self) -> String {
        self.activate();
        return self.name().to_string();
    }

    fn repair_display(&self) -> String {
        return self.name().to_string();
    }
}

pub trait PowerDownModule: Active {
    fn power_down(&mut self) {
        self.deactivate();
    }
}
