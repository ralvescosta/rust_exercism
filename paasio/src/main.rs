use std::{fs::File, io};

use paasio::{ReadStats, WriteStats};

fn main() {
    read();
    write();
}

fn read() -> Result<(), io::Error> {
    let a = File::open("path")?;
    let read = ReadStats::new(a);

    Ok(())
}

fn write() -> Result<(), io::Error> {
    let a = File::open("path")?;
    let write = WriteStats::new(a);

    Ok(())
}
