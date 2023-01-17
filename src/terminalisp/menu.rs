// -*- coding: utf-8, vim: expandtab:ts=4 -*-

// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

// project
use crate::terminalisp::station::menu_error;

// dependencies
use inquire::Select;


pub fn tli_menu(title: &str, options: Vec<String>) -> Result<String, String> {
    match Select::new(title, options).prompt() {
        Ok(v) =>  { Ok(v) },
        Err(e) => {
            menu_error(e.to_string());
            return Err(String::from("Menu error"));
        }
    }
}
