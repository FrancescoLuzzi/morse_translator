use std::fs::{File, OpenOptions};
use std::io::{self, BufRead, BufReader};

pub fn get_reader(arg: &str) -> Box<dyn BufRead> {
    match arg {
        "-" => Box::new(io::stdin().lock()),
        "" => Box::new(io::stdin().lock()),
        file_name => Box::new(BufReader::new(
            OpenOptions::new().read(true).open(file_name).unwrap(),
        )),
    }
}

// hack from https://github.com/rust-lang/rust/issues/72802#issuecomment-1101996578
pub fn get_writer(arg: &str) -> io::Result<File> {
    match arg {
        "-" | "" => {
            let lock = io::stdout();
            #[cfg(any(target_family = "unix", target_family = "wasm"))]
            unsafe {
                use std::os::unix::io::{AsRawFd, FromRawFd};
                Ok(std::fs::File::from_raw_fd(lock.as_raw_fd()))
            }

            #[cfg(target_family = "windows")]
            unsafe {
                use std::os::windows::io::{AsRawHandle, FromRawHandle};
                Ok(std::fs::File::from_raw_handle(lock.as_raw_handle()))
            }
        }
        file_name => OpenOptions::new()
            .write(true)
            .create(true)
            .truncate(true)
            .open(file_name),
    }
}
