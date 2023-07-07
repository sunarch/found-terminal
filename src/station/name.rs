// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

// dependencies
use rand_derive2::RandGen;
use strum_macros::Display;


#[derive(RandGen, Display)]
pub enum StationName {
    // manga - cyberpunk
    Akira, Avalon,
    Babylon,
    Chrysalis,
    Eden,
    Ronin,
    // U.S. states
    Alabama, Alaska, Arkansas,
    California, Carolina, Colorado, Connecticut,
    Dakota, Delaware,
    Florida,
    Georgia,
    Hawaii, Hampshire,
    Idaho, Illinois, Indiana, Iowa,
    Jersey,
    Kansas, Kentucky,
    Louisiana,
    Maine, Maryland, Massachusetts, Michigan, Minnesota, Mississippi, Missouri, Montana,
    Nebraska, Nevada,
    Ohio, Oklahoma, Oregon,
    Pennsylvania,
    Tennessee, Texas,
    Utah,
    Vermont, Virginia, Washington, Wisconsin, Wyoming,
    // cities
    York,
    // greek mythology
    Aether, Artemis, Athena, Atlas,
    Daedalus,
    Helios, Hemera, Hermes,
    Orion,
    Talos, Titan,
    Uranus,
    // other
    Eisenberg,
    // adjectives
    Dauntless,
    Intrepid,
    Reliant, Resolute,
    Serene,
    Valiant,
    // Shakespeare
    Aaron, Alice, Angus, Ariel,
    Beadle, Bishop,
    Caius, Ceres, Corin,
    Diana, Duncan,
    Helena,
    Julia,
    Marcus, Miranda,
    Oliver,
    Percy, Provost,
    Robin,
    Sentinel, Sentry,
    Titus,
    Viola,
    // astronomy
    Nova,
    Pulsar,
    // people - astronomers
    Copernicus,
    Galilei,
    Kepler,
    Newton,
    Sagan,
    Tyson,
    // people - CS
    Babbage, Backus,
    Conway,
    Dijkstra,
    Hamilton, Hopper,
    Kernighan, Knuth,
    Liskov,
    Rivest, Romero, Rossum,
    Shamir,
    // people - authors
    Asimov, Atwood,
    Bachman, Bradbury,
    Clarke,
    Herbert, Huxley,
    Shelley,
}
