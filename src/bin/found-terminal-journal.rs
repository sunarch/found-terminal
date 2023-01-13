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
    let station_name_display = station.name_display();
    println!("{}", station_name_display);
    println!("{}", station.mission_day_display());

    let mut journal = Journal::new(
        "STATION LOG".to_string(),
        station_name_display
    );

    loop {
        if !day(&mut station, &mut journal) {
            break
        };
    }

    journal.print();
}

fn day(station: &mut Station, journal: &mut Journal) -> bool {
    let days_left = station.days_left();

    if days_left < 1 {
        println!("(END-TRANSMISSION)");
        return false;
    }

    println!("{days_left} UNTIL FINAL TRANSMISSION");

    journal.add_entry(Text::new("Enter your log:")
        .prompt()
        .unwrap()
    );

    match menu(&[
        "NEW DAY".into(),
        "STATUS".into(),
        "POWERDOWN".into()
    ]).as_str() {
        "NEW DAY" => {
            station.new_day();
            match menu(&["REPAIR".into(), "SCIENCE".into()]).as_str() {
                "REPAIR" => {
                    repair(menu(&station.broken_sections()), station);
                    return true;
                },
                "SCIENCE" => {
                    science(menu(&station.working_sections()), station);
                    return true;
                }
                &_ => panic!(),
            }
        },
        "STATUS" => station.status(),
        "POWERDOWN" => return false,
        &_ => panic!("test"),
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
