// Original version of this file released by Tristram Oaten under CC0 1.0 Universal
// https://github.com/0atman/noboilerplate -> 8 | Building a space station in Rust

#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_imports)]
#![allow(unused_mut)]

use strum_macros::Display;
use strum_macros::EnumString;
use std::str::FromStr;
use inquire::Text;
use rand_derive2::RandGen;
use rand::random;
use rand::Rng;
use rand::thread_rng;
use rand::prelude::SliceRandom;
use inquire::Select;

// project
use found_terminal::station::station::Station;
use found_terminal::section::section::SectionName;


fn main() {
    let mut station = Station::new();
    let mut station_log = vec![];

    loop {
        if !day(&mut station, &mut station_log) {
            break
        };
    }

    dbg!(station_log);
}

fn day(station: &mut Station, station_log: &mut Vec<String>) -> bool {
    let days_left = station.days_left();

    if days_left < 1 {
        println!("(END-TRANSMISSION)");
        return false;
    }

    println!("{days_left} UNTIL FINAL TRANSMISSION");

    station_log.push(Text::new("Enter your log:")
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

fn science(working_section: String, station: &mut Station) {
    station.break_something();
}
