use std::{fs::File, io::{BufReader, Read}};

pub struct RomFile {
    pub prg: Vec<u8>,
    pub chr: Vec<u8>,
}

pub fn read_rom(filename: &str) -> std::io::Result<RomFile> {

    const KB: usize = 1024;
    const HEADER_LEN: usize = 16;
    const PRG_UNIT: usize = 16 * KB;
    const CHR_UNIT: usize = 8 * KB;

    let f = File::open(filename)?;
    let mut reader = BufReader::new(f);

    let mut file = Vec::new();
    reader.read_to_end(&mut file)?;

    let prg_len = PRG_UNIT * (file[4] as usize);
    let chr_len = CHR_UNIT * (file[5] as usize);

    let prg_start = HEADER_LEN;
    let chr_start = prg_start + prg_len;
    let chr_end = chr_start + chr_len;

    let rom_file = RomFile {
        prg: file[prg_start..chr_start].to_owned(),
        chr: file[chr_start..chr_end].to_owned(),
    };

    Ok(rom_file)
}
