// -*- coding: utf-8, vim: expandtab:ts=4 -*-

// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

// Original version of this file released by Tristram Oaten under CC0 1.0 Universal
// https://github.com/0atman/noboilerplate -> 8 | Building a space station in Rust

// project
use found_terminal::station::station::Station;
use found_terminal::station::components::{Repair, PowerDown};
use found_terminal::journal::journal::Journal;
use found_terminal::terminalisp::menu::tli_menu;


fn main() {
    let mut station = Station::new();

    let mut journal = Journal::new(
        "STATION LOG".to_string(),
        station.name_display()
    );

    loop {
        if !day(&mut station, &mut journal) {
            break
        };
    }

    journal.print();
}

fn day(station: &mut Station, journal: &mut Journal) -> bool {

    journal.add_entry(format!("Mission Day {}", station.mission_day));

    let journal_prompt = format!("Log for Mission Day {}:", station.mission_day);
    loop {
        match journal.prompt_entry(journal_prompt.clone()) {
            Ok(_) => { break; }
            Err(_) => {}
        }
    }

    station.new_day();

    if station.is_shut_down() {
        return false;
    }

    let prompts: Vec<String> = vec![
        "STATUS".to_string(),
        "REPAIR".to_string(),
        "SCIENCE".to_string(),
        "NEW DAY".to_string(),
        "POWER DOWN".to_string()
    ];

    loop {
        let chosen: String;
        match tli_menu("MENU", prompts.clone()) {
            Ok(v) => { chosen = v; },
            Err(_) => { continue; }
        }
        match chosen {
            _ if chosen == prompts[0] => {
                println!("{}", station.status(0, true, true))
            },
            _ if chosen == prompts[1] => {
                station.repair();
                break;
            },
            _ if chosen == prompts[2] => {
                station.science();
                break;
            },
            _ if chosen == prompts[3] => {
                break;
            },
            _ if chosen == prompts[4] => {
                station.power_down();
                break;
            },
            _ => unreachable!(),
        }
    }

    return true;
}
