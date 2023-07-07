// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

#![allow(dead_code)]

/*
1.1
> (PLAYFILE UNKNOWN-SIGNAL.WAV)
> (END-TRANSMISSION)

1.2
<ERROR: SIGNAL LOST>

1.5
> (PLAY GUIDANCE_COMPUTER.WAV)

1.10
> (CONNECTED)
> (PLAYSTREAM /DEV/NET/ANTARCTICA)

2.1
> (CONNECTION-ESTABLISHED :BAUD 9600 :BITS 8 :SNR 3)
> (SIGNAL LOST)
> (PROCESSING)
> (PLAYSTREAM /DEV/MADDIE/SND-MONO-IN-1)

2.3
> (PLAYSTREAM /dev/random)

2.4
> (PLAYSTREAM /DEV/MADDIE/TTY)

2.5
> (PLAYSTREAM /DEV/MADDIE/MIC0)

2.9
> (LAUGH-MK5 "HAAAAAAA")
> (PLAYSTREAM /DEV/MADDIE/RF0)
> (BATTERY-LEVEL 99%)
*/

fn play() {

}

fn play_file() {

}

fn play_stream() {

}

fn laugh() -> &'static str {
    laugh_mk5()
}

fn laugh_mk5() -> &'static str {
    "HAAAAAAA"
}
