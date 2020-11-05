use aes::*;
use rayon::prelude::*;
use std::fmt;
// use std::io::{self, Write};

/* GET round 10 key from
c = 0x605b749eb9f8f10ba9c9ff632b1ab3e0
c' = 0xb45b749eb9f8f143a9c967632b00b3e0
c' = 0x60dc749eaef8f10ba9c9ff722b1a2fe0
c' = 0x605b039eb9b9f10b02c9ff632b1ab376
c' = 0x605b74a0b9f8700ba91fff63c31ab3e0
c' = 0x265b749eb9f8f1cfa9c90e632b3fb3e0
c' = 0x6071749eddf8f10ba9c9ff0a2b1a56e0
c' = 0x605be59eb9e4f10b96c9ff632b1ab366
c' = 0x605b74f4b9f8b50ba970ff63cc1ab3e0
c' = 0x7b5b749eb9f8f1b0a9c9eb632ba3b3e0
c' = 0x607a749e25f8f10ba9c9ffce2b1aeee0
c' = 0x605b569eb978f10b6ec9ff632b1ab341
 */

enum Ratio {
    R2113([GF256;16]),
    R3211([GF256;16]),
    R1321([GF256;16]),
    R1132([GF256;16]),
    RNone,
}

impl Ratio {
    fn is_some(&self) -> bool {
        match self {
            RNone => false,
            _ => true,
        }
    }

    fn get_key(&self) -> Option<&[GF256;16]> {
        match self {
            R2113(key) => Some(key),
            R3211(key) => Some(key),
            R1321(key) => Some(key),
            R1132(key) => Some(key),
            RNone => None,
        }
    }
}

impl fmt::Display for Ratio {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            R2113(key) => write!(f, "R2113(\n{})", array2string(key)),
            R3211(key) => write!(f, "R3211(\n{})", array2string(key)),
            R1321(key) => write!(f, "R1321(\n{})", array2string(key)),
            R1132(key) => write!(f, "R1132(\n{})", array2string(key)),
            RNone => write!(f, "RNone"),
        }
    }
}

use Ratio::*;

fn main() {
    let g2 = GF256::new(2);
    let g3 = GF256::new(3);

    let c_raw = [
        0x60, 0xb9, 0xa9, 0x2b,
        0x5b, 0xf8, 0xc9, 0x1a,
        0x74, 0xf1, 0xff, 0xb3,
        0x9e, 0x0b, 0x63, 0xe0,
    ];
    let c = GF256::from_u8array(&c_raw).unwrap();

    let c_wrongs_raw = [
        [
            0xb4, 0xb9, 0xa9, 0x2b,
            0x5b, 0xf8, 0xc9, 0x00,
            0x74, 0xf1, 0x67, 0xb3,
            0x9e, 0x43, 0x63, 0xe0,
        ],
        [
            0x60, 0xae, 0xa9, 0x2b,
            0xdc, 0xf8, 0xc9, 0x1a,
            0x74, 0xf1, 0xff, 0x2f,
            0x9e, 0x0b, 0x72, 0xe0,
        ],
        [
            0x60, 0xb9, 0x02, 0x2b,
            0x5b, 0xb9, 0xc9, 0x1a,
            0x03, 0xf1, 0xff, 0xb3,
            0x9e, 0x0b, 0x63, 0x76,
        ],
        [
            0x60, 0xb9, 0xa9, 0xc3,
            0x5b, 0xf8, 0x1f, 0x1a,
            0x74, 0x70, 0xff, 0xb3,
            0xa0, 0x0b, 0x63, 0xe0,
        ],
        [
            0x26, 0xb9, 0xa9, 0x2b,
            0x5b, 0xf8, 0xc9, 0x3f,
            0x74, 0xf1, 0x0e, 0xb3,
            0x9e, 0xcf, 0x63, 0xe0,
        ],
        [
            0x60, 0xdd, 0xa9, 0x2b,
            0x71, 0xf8, 0xc9, 0x1a,
            0x74, 0xf1, 0xff, 0x56,
            0x9e, 0x0b, 0x0a, 0xe0,
        ],
        [
            0x60, 0xb9, 0x96, 0x2b,
            0x5b, 0xe4, 0xc9, 0x1a,
            0xe5, 0xf1, 0xff, 0xb3,
            0x9e, 0x0b, 0x63, 0x66,
        ],
        [
            0x60, 0xb9, 0xa9, 0xcc,
            0x5b, 0xf8, 0x70, 0x1a,
            0x74, 0xb5, 0xff, 0xb3,
            0xf4, 0x0b, 0x63, 0xe0,
        ],
        [
            0x7b, 0xb9, 0xa9, 0x2b,
            0x5b, 0xf8, 0xc9, 0xa3,
            0x74, 0xf1, 0xeb, 0xb3,
            0x9e, 0xb0, 0x63, 0xe0,
        ],
        [
            0x60, 0x25, 0xa9, 0x2b,
            0x7a, 0xf8, 0xc9, 0x1a,
            0x74, 0xf1, 0xff, 0xee,
            0x9e, 0x0b, 0xce, 0xe0,
        ],
        [
            0x60, 0xb9, 0x6e, 0x2b,
            0x5b, 0x78, 0xc9, 0x1a,
            0x56, 0xf1, 0xff, 0xb3,
            0x9e, 0x0b, 0x63, 0x41,
        ]
    ];

    /*
    let c_wrongs_raw = [
        [
            0xb4, 0xb9, 0xa9, 0x2b,
            0x5b, 0xf8, 0xc9, 0x00,
            0x74, 0xf1, 0x67, 0xb3,
            0x9e, 0x43, 0x63, 0xe0,
        ],
        [
            0x60, 0xae, 0xa9, 0x2b,
            0xdc, 0xf8, 0xc9, 0x1a,
            0x74, 0xf1, 0xff, 0x2f,
            0x9e, 0x0b, 0x72, 0xe0,
        ],
    ];
     */

    let mut key_cands: Vec<Vec<[GF256;16]>> = vec![
        vec![], vec![], vec![], vec![]
    ];

    'main:for (c_w_index, c_w_raw) in c_wrongs_raw.iter().enumerate() {

        /*
        let p = format!("start: {}\n", c_w_index);
        io::stderr().write_all(p.as_bytes()).unwrap();
        */

        let c_w = GF256::from_u8array(c_w_raw).unwrap();

        let error_indexs = if c[0] != c_w[0] {
            [0, 7, 10, 13]
        } else if c[1] != c_w[1] {
            [1, 4, 11, 14]
        } else if c[2] != c_w[2] {
            [2, 5, 8, 15]
        } else if c[3] != c_w[3] {
            [3, 6, 9, 12]
        } else {
            println!("There are no Error. continue.");
            continue;
        };

        // 議論: このチェックは誤りかもしれない(一致する可能性もある...?) 要検証
        for i in 0..4 {
            if c[error_indexs[i]] == c_w[error_indexs[i]] {
                println!("Something wrong with Error. continue.");
                continue 'main;
            }
        }

        // let hit_keys = (0x11111111u64..=0x111111ffu64)
        let part_keys = (0x0000u64..=0xffffu64)
            .into_par_iter()
            .map(|key_base| {
                let mut c_state = [GF256::new(0);16];
                let mut w_state = [GF256::new(0);16];
                for i in 0..16 {
                    c_state[i] = c[i];
                    w_state[i] = c_w[i];
                }

                let mut key = [GF256::new(0);16];
                let k0 = (key_base & 0x00ffu64) as u8;
                key[error_indexs[0]] = GF256::new(k0);
                let k1 = ((key_base & 0xff00u64) / 0x0100u64) as u8;
                key[error_indexs[1]] = GF256::new(k1);

                add_round_key(&mut c_state, &key);
                inv_shift_rows(&mut c_state);
                inv_sub_bytes(&mut c_state);
                add_round_key(&mut w_state, &key);
                inv_shift_rows(&mut w_state);
                inv_sub_bytes(&mut w_state);

                let mut diff = [GF256::new(0);16];
                for i in 0..16 {
                    diff[i] = c_state[i] ^ w_state[i];
                }

                let r = error_indexs[0];
                let a = diff[r] / g2 == diff[4+r];
                let b = diff[r] / g3 == diff[4+r] / g2;
                let c = diff[r] == diff[4+r] / g3;
                let d = diff[r] == diff[4+r];
                if a {
                    R2113(key)
                } else if b {
                    R3211(key)
                } else if c {
                    R1321(key)
                } else if d {
                    R1132(key)
                } else {
                    RNone
                }
            })
            .filter(|v| v.is_some())
            .collect::<Vec<_>>();

        /*
        for pk in part_keys.iter() {
            println!("{}", pk);
        }
        */

        /*
        let p = format!("part_key end\np_len: {}\n", part_keys.len());
        io::stderr().write_all(p.as_bytes()).unwrap();
        */

        let hit_keys = part_keys
            .into_iter()
            .map(|rpk| {
                (0x0000u64..=0xffffu64)
                    .into_par_iter()
                    .map(|key_base| {
                        let pk = rpk.get_key().unwrap();
                        let mut c_state = [GF256::new(0);16];
                        let mut w_state = [GF256::new(0);16];
                        for i in 0..16 {
                            c_state[i] = c[i];
                            w_state[i] = c_w[i];
                        }

                        let mut key = [GF256::new(0);16];
                        let k0 = pk[error_indexs[0]];
                        key[error_indexs[0]] = k0;
                        let k1 = pk[error_indexs[1]];
                        key[error_indexs[1]] = k1;
                        let k2 = (key_base & 0x00ffu64) as u8;
                        key[error_indexs[2]] = GF256::new(k2);
                        let k3 = ((key_base & 0xff00u64) / 0x0100u64) as u8;
                        key[error_indexs[3]] = GF256::new(k3);

                        add_round_key(&mut c_state, &key);
                        inv_shift_rows(&mut c_state);
                        inv_sub_bytes(&mut c_state);
                        add_round_key(&mut w_state, &key);
                        inv_shift_rows(&mut w_state);
                        inv_sub_bytes(&mut w_state);

                        let mut diff = [GF256::new(0);16];
                        for i in 0..16 {
                            diff[i] = c_state[i] ^ w_state[i];
                        }

                        let r = error_indexs[0];
                        match rpk {
                            R2113(_) => if diff[r] / g2 == diff[r+4] &&
                                diff[r+4] == diff[r+8] &&
                                diff[r+8] == diff[r+12] / g3 {
                                Some(key)
                            } else {
                                None
                            },
                            R3211(_) => if diff[r] / g3 == diff[r+4] / g2 &&
                                diff[r+4] /g2 == diff[r+8] &&
                                diff[r+8] == diff[r+12] {
                                Some(key)
                            } else {
                                None
                            },
                            R1321(_) => if diff[r] == diff[r+4] / g3 &&
                                diff[r+4] / g3 == diff[r+8] / g2 &&
                                diff[r+8] / g2 == diff[r+12] {
                                Some(key)
                            } else {
                                None
                            },
                            R1132(_) => if diff[r] == diff[r+4] &&
                                diff[r+4] == diff[r+8] / g3 &&
                                diff[r+8] / g3 == diff[r+12] / g2 {
                                Some(key)
                            } else {
                                None
                            },
                            _ => None, // don't work
                        }
                    })
                    .filter(|v| v.is_some())
                    .map(|v| v.unwrap())
                    .collect::<Vec<[GF256;16]>>()
            })
            .flatten()
            .collect::<Vec<[GF256;16]>>();

        /*
        let p = format!("hit_key end\nh_len: {}\n", hit_keys.len());
        io::stderr().write_all(p.as_bytes()).unwrap();
        */

        let dest = &mut key_cands[error_indexs[0]];
        *dest = if dest.len() == 0 {
            hit_keys
        } else {
            hit_keys
                .into_iter()
                .filter(|key| {
                    dest.iter().any(|k| k == key)
                })
                .collect::<Vec<_>>()
        };

        // 最小の鍵の個数を求める
        if key_cands.iter().all(|ks| ks.len() == 1) {
            println!("Necessary Keys Num: {}", c_w_index+1);
            break 'main;
        }
    }

    let mut _num = 0;
    let mut res = [GF256::new(0);16];

    for (_ind, hit_keys) in key_cands.into_iter().enumerate() {
        // println!("$$$$ {} $$$$", _ind);
        for key in hit_keys.into_iter() {
            // dump_array(&key, "key");
            for i in 0..16 {
                res[i] ^= key[i];
            }
            _num += 1;
        }
    }

    /*
    println!("len: {}", num);
    let p = format!("len: {}\n", num);
    io::stderr().write_all(p.as_bytes()).unwrap();
     */

    dump_array(&res, "round 10 key");
}
