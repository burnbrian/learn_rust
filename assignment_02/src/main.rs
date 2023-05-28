extern crate ferris_says;                                                       // import external crate

use ferris_says::say;                                                           // required for 'say'
use std::io::{ stdout, BufWriter };                                             // required to oneline 'say'

fn main() {
    let message = "What does the crab say?";                                    // message goes here
    let width = 40;                                                             // set width of message

    let mut writer = BufWriter::new(stdout());                                  // preferred for repeat writes
    say(message, width, &mut writer).unwrap();                                  // send args to fsays
}