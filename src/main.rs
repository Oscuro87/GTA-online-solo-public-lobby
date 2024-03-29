use std::{time::Duration};
use std::io::{stdin, stdout, Read, Write};
use winproc::Process;

enum Errors {
    NoProcess = 1,
    CantAttach = 2,
}

impl Into<i32> for Errors {
    fn into(self) -> i32 {
        self as i32
    }
}

fn pause() {
    let mut stdout = stdout();
    let mut empty_buffer: &mut [u8] = &mut [0];
    stdout.write("Press any key...".as_bytes()).unwrap();
    stdout.flush().unwrap();
    stdin().read(&mut empty_buffer).unwrap(); // Blocking read
}

fn get_gta5_pid() -> u32 {
    match Process::from_name("GTA5.exe") {
        Ok(proc) => proc.id(),
        Err(_err) => {
            println!("Cannot find any running GTA5 processes. You must start GTA5 and go to a public lobby.");
            pause();
            std::process::exit(Errors::NoProcess.into());
        }
    }
}

fn main() {
    let pid: u32 = get_gta5_pid();

    let gta_v = match remoteprocess::Process::new(pid) {
        Ok(x) => x,
        Err(_) => {
            println!("Error: Cannot attach to the GTA5 process.\nTry again or report the bug here: https://github.com/Oscuro87/gtao-solo-lobby/issues/new.");
            pause();
            std::process::exit(Errors::CantAttach.into());
        }
    };

    println!("Process GTA5.exe opened.");
    println!("DO NOT CLOSE THIS WINDOW  OR YOUR GAME WILL HANG FOREVER.");
    println!("Process GTA5.exe suspended for 10 seconds... (Game will freeze for 10 secs)");

    let _lock = gta_v.lock().expect("Cannot lock (suspend) GTA5.exe!");
    // We are controlling GTA5's process from here.
    let duration: Duration = Duration::from_millis(250);
    for i in 0..10 {
        print!("{}... ", 10 - i);
        stdout().flush().unwrap();
        std::thread::sleep(duration);
    }
    // Lock is auto released here as the lock is being destroyed.
}
