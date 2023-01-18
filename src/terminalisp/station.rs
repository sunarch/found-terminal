// -*- coding: utf-8, vim: expandtab:ts=4 -*-

// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

// module
use crate::terminalisp::symbols;


pub fn status(header: String,
              show_fields: bool,
              keys: Vec<String>,
              values: Vec<String>,
              show_inner: bool,
              inner_key: String,
              inner_values: Vec<String>,
              indent: u8) -> String {

    let mut result: String = String::new();

    for _ in 1..=indent { result.push_str("    ") }
    result.push_str(format!("({}", header).as_str());
    if show_inner { result.push('\n'); }

    if show_fields {
        for tuple in keys.iter().zip(values) {
            if show_inner { for _ in 1..=indent+1 { result.push_str("    ") } }
            if !show_inner { result.push(' '); }
            result.push_str(format!("{} {}", tuple.0, tuple.1).as_str());
            if show_inner { result.push('\n'); }
        }
    }

    if show_inner {
        for _ in 1..=indent+1 { result.push_str("    ") }
        result.push_str(format!("{} (\n", inner_key).as_str());

        for inner_value in inner_values.iter() {
            result.push_str(inner_value.as_str());
        }

        for _ in 1..=indent+1 { result.push_str("    ") }
        result.push_str(")\n");
    }

    if show_inner { for _ in 1..=indent { result.push_str("    ") } }
    result.push_str(")\n");

    return result;
}

pub fn sections_ok() {
    println!("(sections {})", symbols::OK);
}

pub fn section_failure(name: String) {
    println!("(section-failure \"{name}\")");
}

pub fn until_final_transmission(count: u16) {
    println!("(until-final-transmission {count})");
}

pub fn end_transmission() {
    println!("(end-transmission)");
}
