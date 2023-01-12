// -*- coding: utf-8, vim: expandtab:ts=4 -*-

// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

pub struct Journal {
    header: String,
    title: String,
    log: Vec<String>,
}

impl Journal {
    pub fn new(header: String, title: String) -> Self {
        Journal {
            header,
            title,
            log: vec![],
        }
    }

    pub fn add_entry(&mut self, text: String) {
        let _ = &self.log.push(text);
    }

    pub fn print(&self) {
        println!("{:-<80}", "");
        println!("{}: {}", &self.header, &self.title);
        println!("{:-<80}", "");
        for item in &self.log {
            println!("{}", item);
            println!("{:-<80}", "");
        }
    }
}
