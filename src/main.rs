mod page_header;
mod database_header;

use anyhow::{bail, Result};
use std::fs::File;
use std::io::prelude::*;
use page_header::PageHeader;
use crate::database_header::DatabaseHeader;

fn main() -> Result<()> {
    // Parse arguments
    let args = std::env::args().collect::<Vec<_>>();
    match args.len() {
        0 | 1 => bail!("Missing <database path> and <command>"),
        2 => bail!("Missing <command>"),
        _ => {}
    }

    let command = &args[2];
    match command.as_str() {
        ".dbinfo" => {
            let mut file = File::open(&args[1])?;
            let mut header = [0; 100];
            file.read_exact(&mut header)?;
            let db_header = DatabaseHeader::try_from(&header).unwrap();

            println!("database page size: {}", db_header.page_size);

            let mut raw_page_header = [0; 12];
            file.read_exact(&mut raw_page_header)?;
            let page_header = PageHeader::from(&raw_page_header as &[u8]);

            println!("number of cells: {}", page_header.cell_count);
        },
        ".tables" => {

        }
        _ => bail!("Missing or invalid command passed: {}", command),
    }

    Ok(())
}
