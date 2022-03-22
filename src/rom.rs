pub struct InfoHeader<'a> {
    entrypoint: &'a[u8; 0x103 - 0x100], // NOP followed by JP $150. Jumps here after displaying nintendo logo.
    nintendo_logo: &'a[u8; 0x133 - 0x104], // Bitmap of Nintendo Logo.
    title: &'a[u8; 0x143 - 0x134], // Title of the game in uppercase ascii. Filled to 16 bytes.
    manufacturer_code: &'a[u8; 0x142 - 0x13f], // Part of the title in older cartridges. Contains a 4 char uppercase man-code in new cartridges.
    cgb_flag: u8, // 0x143 - Part of the title in older cartridges. In CGB cartridges the upper bit is used to enable CGB functions.
    new_licensee_code: &'a[u8; 0x145 - 0x144], // Two char licensee code. Has the publisher of the game.
    sgb_flag: u8, // 0x146 - Specifies whether the game supports SGB functions.
    cartridge_type: u8, // 0x147 - Specifies which Memory Bank Controller (if any) is used in the cartridge.
    rom_size: u8, // 0x148 - Specifies the ROM Size of the cartridge. Typically calculated as “N such that 32 KiB << N”.
    ram_size: u8, // 0x149 - Specifies the size of the external RAM in the cartridge (if any).
    destination_code: u8, // 0x14A - Specifies if this version of the game is supposed to be sold in Japan, or anywhere else.
    old_licensee_code: u8, // 0x14B - Specifies the games company/publisher code in range $00-FF. A value of $33 signals that the New Licensee Code (in header bytes $0144-0145) is used instead.
    mask_rom_ver: u8, // 0x14C - Specifies the version number of the game. That is usually $00.
    header_checksum: u8, // 0x14D - Contains an 8 bit checksum across the cartridge header bytes $0134-014C.
    global_checksum: &'a[u8; 0x14F - 0x14E] // Contains a 16 bit checksum (upper byte first) across the whole cartridge ROM.
}

pub struct Rom<'a> {
    pub(crate) info_header: InfoHeader<'a>,
}

pub enum MBC {
    NoMBC, // 32 KiB ROM only
    MBC1,
    MBC2,
    MBC3,
    MBC4,
    MBC5,
    MBC6,
    MBC7,
    HuC1,
    // There's some others but i dont care
}
