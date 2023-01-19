// -*- coding: utf-8, vim: expandtab:ts=4 -*-

// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

// module
use crate::terminalisp::symbols;


pub fn station_header(name: String, version: u8) {
    println!("(station :name {} :version {})", name, version);
}

pub fn station_status(keys: Vec<String>,
                      values: Vec<String>,
                      section_names: Vec<String>,
                      section_statuses: Vec<bool>) {

    println!("(station-status");
    for tuple in keys.iter().zip(values) {
        println!("    :{} {}", tuple.0, tuple.1);
    }
    println!("    :sections (");
    for tuple in section_names.iter().zip(section_statuses) {
        let active = if tuple.1 { symbols::OK } else { symbols::INACTIVE };
        println!("        (section :status {:<9} :name {}", active, tuple.0);
    }
    println!("    )");
    println!(")");
}

pub fn section_failure_none() {
    println!("(section-failure {})", symbols::NONE);
}

pub fn section_to_repair_invalid() {
    println!("(section-to-repair {})", symbols::INVALID);
}
