// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

// dependencies
use byteorder::{ByteOrder, LittleEndian};


pub fn op_code_little_endian(op_code: u16) -> [u8; 2] {
    let mut op_code_buf : [u8; 2] = [0; 2];
    LittleEndian::write_u16(&mut op_code_buf, op_code);

    return op_code_buf;
}
