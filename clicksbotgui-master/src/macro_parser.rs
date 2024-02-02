//#![windows_subsystem = "windows"]

use std::fs;
use std::fs::File;
use std::io::Read;

pub fn parse_zbf(
    clicks_vec: &mut Vec<u64>,
    releases_vec: &mut Vec<u64>,
    fps_val: &mut f32,
    name: String,
) {
    let mut file = File::open(name).unwrap();
    let mut buf = [0u8; 4];

    file.read(&mut buf).unwrap();
    let delta = f32::from_le_bytes(buf);

    file.read(&mut buf).unwrap();
    let speedhack = f32::from_le_bytes(buf);

    let fps: f32 = 1.0 / delta / speedhack;

    let mut clicks: Vec<u64> = Vec::new();
    let mut releases: Vec<u64> = Vec::new();

    let mut buf2 = [0; 2]; // a buffer 2 bytes big, for hold/player1

    loop {
        if file.read(&mut buf).unwrap() != 4 {
            // if we didnt read 4 bytes
            // no more stuff to read, exit
            break;
        }
        let frame = u32::from_le_bytes(buf);
        if file.read(&mut buf2).unwrap() != 2 {
            break;
        }
        let hold = buf2[0] == 0x31;

        if hold {
            clicks.push(frame as u64);
        } else {
            releases.push(frame as u64);
        }
    }

    clicks.dedup();
    releases.dedup();

    *clicks_vec = clicks;
    *releases_vec = releases;
    *fps_val = fps;

    //println!("{:?}", clicks_vec);
    //println!("{:?}", releases_vec);
}

pub fn parse_txt(
    clicks_vec: &mut Vec<u64>,
    releases_vec: &mut Vec<u64>,
    clicks_path: &str,
    releases_path: &str,
) {
    let mut _clicks: Vec<u64> = Vec::new();
    let mut _releases: Vec<u64> = Vec::new();

    let clicks_read = fs::read_to_string(clicks_path).unwrap();
    let releases_read = fs::read_to_string(releases_path).unwrap();

    let clicks_string = clicks_read.trim_end_matches("\n");
    let releases_string = releases_read.trim_end_matches("\n");

    _clicks = clicks_string
        .split("\n")
        .map(|x| x.trim().parse::<u64>().unwrap())
        .collect();

    _releases = releases_string
        .split("\n")
        .map(|x| x.trim().parse::<u64>().unwrap())
        .collect();

    _clicks.dedup();
    _releases.dedup();

    *clicks_vec = _clicks;
    *releases_vec = _releases;

    //println!("{:?}", clicks_vec);
    //println!("{:?}", releases_vec);
}
