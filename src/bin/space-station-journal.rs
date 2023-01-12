// -*- coding: utf-8, vim: expandtab:ts=4 -*-

// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

// Original version of this file released by Tristram Oaten under CC0 1.0 Universal
// https://github.com/0atman/noboilerplate -> 8 | Building a space station in Rust

#![allow(dead_code)]

use strum_macros::Display;
use strum_macros::EnumString;
use std::str::FromStr;
use inquire::Text;
use rand_derive2::RandGen;
use rand::random;
use rand::Rng;
use rand::thread_rng;
use inquire::Select;

use found_terminal::journal::journal::Journal;


#[derive(Debug, RandGen)]
struct Station {
    name: Name,
    version: u8,
    sections: Vec<Section>
}

#[derive(Debug, RandGen, Display)]
enum Name {
    Akira,     California, Daedalus,
    Eisenberg, Intrepid,   Miranda,
    Nova,      Reliant,    Sagan
}

impl Station {
    fn new() -> Self {
        Station {
            name: random(),
            version: random(),
            sections: (0..10)
                .map(|_| random())
                .collect()
        }
    }

    fn days_left(&self) -> usize {
        self.sections
            .iter()
            .filter(|m| m.active)
            .count()
    }

    fn working_sections(&self) -> Vec<String> {
        self.sections
            .iter()
            .filter(|m| m.active)
            .map(|m| m.name.to_string())
            .collect()
    }

    fn broken_sections(&self) -> Vec<String> {
        self.sections
            .iter()
            .filter(|m| !m.active)
            .map(|m| m.name.to_string())
            .collect()
    }

    fn new_day(&mut self) {
        self.break_something();
    }

    fn break_something(&mut self) {
        let broken_index = thread_rng().gen_range(0..self.sections.len());
        let mut broken_section = &mut self.sections[broken_index];
        if broken_section.active {
            broken_section.active = false;
            println!("(Section-FAILURE {})", &broken_section.name);
        } else {
            println!("(sections OK)");
        }
    }

    fn status(&self) {
        dbg!(&self);
    }

    fn name_display(&self) -> String {
        return format!("Station \"{}\" v{}", &self.name, &self.version);
    }
}

#[derive(Debug, RandGen, Eq, PartialEq)]
struct Section {
    name: SectionName,
    active: bool,
}

#[derive(Debug, RandGen, Display, EnumString)]
#[derive(Eq, PartialEq)]
enum SectionName {
    AstroScience,     Solar,       Antenna,
    RadiationMirrors, Sleeping,    NuclearGenerator,
    Galley,           Transponder, Tracking
}


fn main() {
    let mut station = Station::new();
    let station_name_display = station.name_display();
    println!("{}", station_name_display);

    let mut journal = Journal::new(
        "STATION LOG".to_string(),
        station_name_display
    );

    loop {
        if !day(&mut station, &mut journal) {
            break;
        }
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
