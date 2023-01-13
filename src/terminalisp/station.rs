// -*- coding: utf-8, vim: expandtab:ts=4 -*-

// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

// project
use crate::station::station::Station;

// module
use crate::terminalisp::symbols;

pub fn tl_station_status(station: &Station, show_details: bool, show_sections: bool) {
    print!("(station");

    if show_details {
        println!();
        println!("    {:<12} \"{}\"", ":name", &station.name);
        println!("    {:<12} {}", ":version", &station.version);
    }

    if show_details || show_sections {
        print!("   ");
    }
    print!(" {:<12} {}", ":mission-day", &station.mission_day);

    if show_sections {
        println!();
        println!("    {:<12} (", ":sections");
        for section in &station.sections {
            let status = if section.active { symbols::OK } else { symbols::INACTIVE };
            let section_name = format!("\"{}\"", section.name);
            println!("        (section :name {:<18} :status {})", section_name, status);
        }
        println!("    )");
    }

    if show_details && !show_sections {
        println!();
    }
    println!(")");
}

pub fn tl_until_final_transmission(count: u8) {
    println!("(until-final-transmission {})", count);
}
