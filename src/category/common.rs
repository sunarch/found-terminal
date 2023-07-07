// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

// dependencies
use rand::Rng;
use rand::seq::SliceRandom;
use rand::thread_rng;


pub trait SectionsAvailable {
    const SECTIONS_AVAILABLE: u16;
}

pub trait SectionCounts {
    fn total_sections(&self) -> u16;
    fn installed_sections(&self) -> u16;
}

pub fn random_bools(count: u16, min_count: u16, max_count: u16) -> Vec<bool> {
    let mut rng = thread_rng();
    let mut result: Vec<bool> = vec![];
    let mut random_count: u16 = count;

    if min_count < count {
        for _ in 1..=min_count {
            result.push(true)
        }
        random_count -= min_count;
    }

    if max_count < count {
        let extra = count - max_count;
        for _ in 1..=extra {
            result.push(false)
        }
        random_count -= extra;
    }

    for _ in 1..=random_count {
        result.push(rng.gen_bool(0.5));
    }

    result.shuffle(&mut rng);

    return result;
}

pub fn random_positions(min_count: u8, max_count: u8) -> Vec<u8> {
    let count: u8 = thread_rng().gen_range(min_count..max_count);

    let mut positions: Vec<u8> = vec![];
    for _ in 1..=count {
        positions.push(thread_rng().gen_range(1..=max_count));
    }

    return positions;
}
