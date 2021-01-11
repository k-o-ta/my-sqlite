use anyhow::Result;
use my_sqlite::db::DbFile;
use std::fs::File;
use std::io::{BufReader, Read};

fn main() -> Result<()> {
    let mut file = File::open("my-dropbox.db3")?;
    let mut buff = BufReader::new(&file);
    let ret = DbFile::from_read(&mut buff)?;
    Ok(())
}
