use std::{
    fs::{self, File, OpenOptions},
    io::{self, Read, Write},
    path::Path,
    process
};

const LOCK_FILE: &str = concat!("/var/run/quip", "quip.pid");

pub(crate) fn check_lock() -> bool {
    let lock_file_path = Path::new(LOCK_FILE);
    let mut pass: bool = false;

    if lock_file_path.exists() {

        match read(lock_file_path) {
            Ok(existing_pid) => { println!("PID found: {}", existing_pid); }
            Err(e) => { println!("Failed to read lock file: {}", e); }
        }
    } else {
        println!("No lock found.");
        pass = true;
    }
    
    pass
}

pub(crate) fn lock() {

     match create(Path::new(LOCK_FILE), process::id()) {
         Ok(_) => { println!("Locked. {} written to {}", process::id(), LOCK_FILE); }
         Err(e) => {
             eprintln!("Failed to lock: {}. Exiting.", e);
             process::exit(1);
         }
     }
}

fn read(path: &Path) -> io::Result<u32> {
    let mut file = File::open(path)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    contents.trim().parse::<u32>().map_err(|_| {
        io::Error::new(
            io::ErrorKind::InvalidData,
            "Could not parse PID from lock file",
        )
    })
}

fn create(path: &Path, pid: u32) -> io::Result<()> {
    let mut file = OpenOptions::new()
        .write(true)
        .create_new(true)
        .open(path)?;

    file.write_all(pid.to_string().as_bytes())?;
    file.flush()?;
    Ok(())
}

fn remove(path: &Path) -> io::Result<()> {
    fs::remove_file(path)?;
    println!("Unlocked {}.", path.display());
    Ok(())
}