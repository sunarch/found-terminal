// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

// module
use crate::terminalisp::symbols;


fn journal_entry_status(message: String) {
    println!("(journal-entry-status {})", message);
}

pub fn journal_entry_status_error(error: String) {
    journal_entry_status(format!("{} \"{}\"", symbols::ERROR, error));
}

pub fn journal_entry_status_saved() {
    journal_entry_status(String::from(symbols::SAVED));
}
