// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

// dependencies
use inquire::Text;

// project
use crate::terminalisp::journal::*;


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

    pub fn prompt_entry(&mut self, prompt: String) -> Result<String, String> {
        match Text::new(prompt.as_str()).prompt() {
            Ok(v) =>  {
                self.add_entry(v);
                journal_entry_status_saved();
                Ok(String::from("Journal entry success."))
            },
            Err(e) => {
                journal_entry_status_error(e.to_string());
                Err(String::from("Journal entry error."))
            }
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
