use nix::sys::signal::kill;
use nix::sys::signal::{self, Signal};
use nix::unistd::Pid;
use std::{env, io};

fn main() {
    // TODO 1: Take the pid as an argument from the command line

    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let input_trim = input.trim().to_owned();
    let input_as_str: i32 = input_trim.parse::<i32>().unwrap();

    // TODO 2: Use the kill function for sending the signal

    kill(Pid::from_raw(input_as_str), Signal::SIGINT).unwrap();
    
}
