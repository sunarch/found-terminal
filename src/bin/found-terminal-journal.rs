// -*- coding: utf-8, vim: expandtab:ts=4 -*-

// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

// Original version of this file released by Tristram Oaten under CC0 1.0 Universal
// https://github.com/0atman/noboilerplate -> 8 | Building a space station in Rust

// lib
use std::str::FromStr;

// dependencies
use inquire::Text;
use inquire::Select;

// project
use found_terminal::station::station::Station;
use found_terminal::section::section::SectionName;
use found_terminal::journal::journal::Journal;


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
    journal.add_entry(Text::new(journal_prompt.as_str())
        .prompt()
        .unwrap()
    );

    station.new_day();

    if station.is_shut_down() {
        return false;
    }

    loop {
        match menu(&[
            "STATUS".into(),
            "REPAIR".into(),
            "SCIENCE".into(),
            "NEW DAY".into(),
            "POWER DOWN".into(),
        ]).as_str() {
            "STATUS" => station.status(),
            "REPAIR" => {
                repair(menu(&station.broken_sections()), station);
                break;
            },
            "SCIENCE" => {
                science(menu(&station.working_sections()), station);
                break;
            }
            "NEW DAY" => {break;}
            "POWER DOWN" => {
                station.shut_down();
                break;
            },
            &_ => panic!(),
        }
    }

    return true;
}

fn menu(items: &[String]) -> String {
    Select::new("MENU", items.to_vec())
        .prompt()
        .unwrap()
}

fn repair(broken_section: String, station: &mut Station) {
    let section = SectionName::from_str(
        broken_section.as_str()
    ).unwrap();

    let broken_index = station.sections
        .iter()
        .position(|m| m.name == section)
        .expect("Section not found.");

    station.sections[broken_index].active = true;
}

fn science(_working_section: String, station: &mut Station) {
    station.break_something();
}
