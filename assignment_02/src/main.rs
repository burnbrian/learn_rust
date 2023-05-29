extern crate ferris_says;

use ferris_says::say;
use std::io::{ stdout, BufWriter };

fn main() {
    // message and message width
    let message = "What does the crab say?";
    let width = 40;

    // send args to say function
    let mut writer = BufWriter::new(stdout());
    say(message, width, &mut writer).unwrap();
}