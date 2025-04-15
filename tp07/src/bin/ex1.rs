use nix::libc;
use nix::sys::signal::{self, SigHandler, Signal};
use std::thread;
use std::time::Duration;

extern "C" fn handle_signal(signal: libc::c_int) {
    let signal = Signal::try_from(signal).unwrap();
    println!("received signal {}: {}", signal, signal.as_str());

    // TODO 1: For each signal, print the specified letter
    match signal {
        Signal::SIGHUP => print!("a"),
        Signal::SIGINT => print!("b"),
        Signal::SIGQUIT => print!("n"),
        Signal::SIGTRAP => print!("s"),
        Signal::SIGFPE => print!(" "),
        _ => {
        }
    }
}

fn main() {
    let signals = [
        Signal::SIGHUP,  // a
        Signal::SIGINT,  // b
        Signal::SIGQUIT, // n
        Signal::SIGTRAP, // s
        Signal::SIGFPE,  // space
    ];
    let handler = SigHandler::Handler(handle_signal);
    println!("My pid is {}", std::process::id());
    for signal in signals {
        // Registering the handler for each signal
        unsafe { signal::signal(signal, handler) }.unwrap();
    }
    loop {
        thread::sleep(Duration::from_millis(10));
    }
}
