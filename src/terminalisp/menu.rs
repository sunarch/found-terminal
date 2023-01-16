// -*- coding: utf-8, vim: expandtab:ts=4 -*-

// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

// dependencies
use inquire::Select;


pub fn tli_menu(title: &str, options: Vec<String>) -> String {
    Select::new(title, options)
        .prompt()
        .unwrap()
}
