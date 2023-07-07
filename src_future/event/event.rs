// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

#![allow(dead_code)]

// dependencies
use rand_derive2::RandGen;
use strum_macros::Display;
use strum_macros::EnumString;


#[derive(RandGen, Display, EnumString)]
#[derive(Eq, PartialEq)]
pub enum Event {
    AModuleComesBack, // A module comes back!
    DangerousSolarFlare, // Dangerous solar flare
    DeepSpacePulse, // Deep space pulse
    DeepSpaceWhaleSound, // Deep-space whale sound!?
    EarthIsDead, // Earth is DEAD
    MessageFromEarth, // Message from Earth!
    MoonBaseSignal, // Moon base signal!
    Paradox,
    PowerSurgeKillsAModule, // Power surge kills a module
    ReConnectionToAnOldSatelliteNetwork, // Re-connection to an old satellite network
    NullEvent, // Reroll
    TotalPowerFailure, // Total power failure - all modules die early
}
