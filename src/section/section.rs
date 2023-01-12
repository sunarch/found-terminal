// -*- coding: utf-8, vim: expandtab:ts=4 -*-

// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

// dependencies
use rand_derive2::RandGen;
use strum_macros::Display;
use strum_macros::EnumString;


#[derive(Debug, RandGen, Eq, PartialEq)]
pub struct Section {
    pub name: SectionName,
    pub active: bool,
}

#[derive(Debug, RandGen, Display, EnumString)]
#[derive(Eq, PartialEq)]
pub enum SectionName {
    Antenna,
    AstroScience,
    Galley,
    NuclearGenerator,
    RadiationMirrors,
    Sleeping,
    Solar,
    Tracking,
    Transponder,
}
