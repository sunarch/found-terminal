// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

// dependencies
use bincode;
use serde_derive::{Serialize, Deserialize};

// project
use crate::protocol::common;
use crate::protocol::constants::STATION_PACKET_ID;
use crate::protocol::op_codes::{OP_PING};


#[derive(Serialize, Deserialize, Copy, Clone)]
#[repr(C, packed)]
struct PingPacket {
    id: [char; 8],
    op_code: [u8; 2],
    prot_ver_hi: u8,
    prot_ver_lo: u8,
    flags: u8,
    diag_priority: u8,
    target_port_address_top_hi: u8,
    target_port_address_top_lo: u8,
    target_port_address_bottom_hi: u8,
    target_port_address_bottom_lo: u8,
}

pub fn compose() -> Vec<u8> {

    let packet = PingPacket {
        id: STATION_PACKET_ID,
        op_code: common::op_code_little_endian(OP_PING),
        prot_ver_hi: 0,
        prot_ver_lo: 14,
        flags: 0,
        diag_priority: 0,
        target_port_address_top_hi: 0,
        target_port_address_top_lo: 0,
        target_port_address_bottom_hi: 0xFF,
        target_port_address_bottom_lo: 0xFF,
    };
    let encoded: Vec<u8> = bincode::serialize(&packet).unwrap();
    // let decoded: Entity = bincode::deserialize(&encoded[..]).unwrap();

    return encoded;
}
