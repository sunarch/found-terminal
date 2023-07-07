// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

pub const STATION_PACKET_ID : [char; 8] = ['S', 't', 'a', 't', 'i', 'o', 'n', '\0'];

pub const PORT_ADDRESS_MIN : u16 = 0;
pub const PORT_ADDRESS_MAX : u16 = 32767;

pub const DEFAULT_BROADCAST_IP_2 : &str = "2.255.255.255";
pub const DEFAULT_BROADCAST_IP_10 : &str = "10.255.255.255";

pub const DEFAULT_IP_PORT : u16 = 0x1936;
