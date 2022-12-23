#![allow(unused)]

use std::{env, fs::{File, OpenOptions}, io::{Read, Seek, SeekFrom, Write}, path::{Path, PathBuf}};
use anyhow::{Result, anyhow, Context};

const ELF_MAGIC: [u8; 4] = [0x7f, 0x45, 0x4c, 0x46];

const ELF_IDENT_SIZE: u8 = 16;
const ELF_IDENT_PAD_INDEX: u8 = 7;
const ELF_PADDING_SIZE: u8 = ELF_IDENT_SIZE - ELF_IDENT_PAD_INDEX;

fn main() -> Result<()> {
    let mut args = env::args().skip(1);
    if args.len() != 2 {
        println!("Syntax: markelf <input file> <string>");
        return Ok(());
    }

    let mut file = OpenOptions::new()
        .read(true)
        .write(true)
        .append(false)
        .truncate(false)
        .open(args.next().unwrap())?;

    let string = args.next().unwrap();
    let string_bytes = string.as_bytes();

    if string_bytes.len() > ELF_PADDING_SIZE as usize {
        return Err(anyhow!("Error: String exceeds size of ELF identification padding bytes ({} bytes)", ELF_PADDING_SIZE));
    }
    
    {
        let mut buffer: [u8; 4] = [0; 4];
        file.read_exact(&mut buffer)?;
        if buffer != ELF_MAGIC {
            return Err(anyhow!("Error: Not an ELF object file"));
        }
    }

    file.seek(SeekFrom::Start(ELF_IDENT_PAD_INDEX as u64));
    file.write_all(string_bytes);

    Ok(())
}