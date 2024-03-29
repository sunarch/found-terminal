// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

// Original version of this file released by Tristram Oaten under CC0 1.0 Universal
// https://github.com/0atman/noboilerplate -> 8 | Building a space station in Rust

// library
use std::str::FromStr;

// dependencies
use inquire::Select;
use rand::{random, Rng, thread_rng};
use rand_derive2::RandGen;
use strum_macros::{Display, EnumString};

// project
use found_terminal::journal::journal::Journal;
use found_terminal::terminalisp::station as tl_station;
use found_terminal::terminalisp::original as tl_original;


#[derive(RandGen)]
struct Station {
    name: Name,
    version: u8,
    sections: Vec<Section>
}

#[derive(RandGen, Display)]
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
            tl_station::section_failure(broken_section.name.to_string())
        } else {
            tl_original::section_failure_none();
        }
    }

    fn status(&self) {
        let mut section_names: Vec<String> = vec![];
        let mut section_statuses: Vec<bool> = vec![];

        for section in &self.sections {
            section_names.push(section.name.to_string());
            section_statuses.push(section.active);
        }

        tl_original::station_status(
            vec![
                String::from("name"),
                String::from("version"),
            ],
            vec![
                format!("\"{}\"", self.name),
                format!("{}", self.version),
            ],
            section_names,
            section_statuses
        );
    }

    fn log_header(&self) -> String {
        return format!("Station \"{}\" v{}", &self.name, &self.version);
    }
}

#[derive(RandGen, Eq, PartialEq)]
struct Section {
    name: SectionName,
    active: bool,
}

#[derive(RandGen, Display, EnumString)]
#[derive(Eq, PartialEq)]
enum SectionName {
    AstroScience,     Solar,       Antenna,
    RadiationMirrors, Sleeping,    NuclearGenerator,
    Galley,           Transponder, Tracking
}


fn main() {
    let mut station = Station::new();

    tl_original::station_header(station.name.to_string(), station.version);

    let mut journal = Journal::new(
        "STATION LOG".to_string(),
        station.log_header()
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
        tl_station::end_transmission();
        return false;
    }

    tl_station::until_final_transmission(days_left as u16);

    let journal_prompt = String::from("Enter your log:");
    loop {
        match journal.prompt_entry(journal_prompt.clone()) {
            Ok(_) => { break; }
            Err(_) => {}
        }
    }

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
    loop {
        match Select::new("MENU", items.to_vec()).prompt() {
            Ok(v) => { return v; }
            Err(_) => { continue; }
        }
    }
}

fn repair(broken_section: String, station: &mut Station) {
    let section = SectionName::from_str(
        broken_section.as_str()
    );

    let section_name: SectionName;
    match section {
        Ok(v) => {
            section_name = v;
        }
        Err(_) => {
            tl_original::section_to_repair_invalid();
            return;
        }
    }

    let broken_index = station.sections
        .iter()
        .position(|m| m.name == section_name)
        .expect("(section 'not-found)");

    station.sections[broken_index].active = true;
}

fn science(_working_section: String, station: &mut Station) {
    station.break_something();
}
