pub const MODES_CHECKSUM_TABLE: [u32; 112] = [
    0x3935ea, 0x1c9af5, 0xf1b77e, 0x78dbbf, 0xc397db, 0x9e31e9, 0xb0e2f0, 0x587178, 0x2c38bc,
    0x161c5e, 0x0b0e2f, 0xfa7d13, 0x82c48d, 0xbe9842, 0x5f4c21, 0xd05c14, 0x682e0a, 0x341705,
    0xe5f186, 0x72f8c3, 0xc68665, 0x9cb936, 0x4e5c9b, 0xd8d449, 0x939020, 0x49c810, 0x24e408,
    0x127204, 0x093902, 0x049c81, 0xfdb444, 0x7eda22, 0x3f6d11, 0xe04c8c, 0x702646, 0x381323,
    0xe3f395, 0x8e03ce, 0x4701e7, 0xdc7af7, 0x91c77f, 0xb719bb, 0xa476d9, 0xadc168, 0x56e0b4,
    0x2b705a, 0x15b82d, 0xf52612, 0x7a9309, 0xc2b380, 0x6159c0, 0x30ace0, 0x185670, 0x0c2b38,
    0x06159c, 0x030ace, 0x018567, 0xff38b7, 0x80665f, 0xbfc92b, 0xa01e91, 0xaff54c, 0x57faa6,
    0x2bfd53, 0xea04ad, 0x8af852, 0x457c29, 0xdd4410, 0x6ea208, 0x375104, 0x1ba882, 0x0dd441,
    0xf91024, 0x7c8812, 0x3e4409, 0xe0d800, 0x706c00, 0x383600, 0x1c1b00, 0x0e0d80, 0x0706c0,
    0x038360, 0x01c1b0, 0x00e0d8, 0x00706c, 0x003836, 0x001c1b, 0xfff409, 0x000000, 0x000000,
    0x000000, 0x000000, 0x000000, 0x000000, 0x000000, 0x000000, 0x000000, 0x000000, 0x000000,
    0x000000, 0x000000, 0x000000, 0x000000, 0x000000, 0x000000, 0x000000, 0x000000, 0x000000,
    0x000000, 0x000000, 0x000000, 0x000000,
];

pub const AIS_CHARSET: &[u8] =
    "?ABCDEFGHIJKLMNOPQRSTUVWXYZ????? ???????????????0123456789??????".as_bytes();

pub const FIX_ERRORS: bool = true;
pub const AGGRESSIVE: bool = false;
pub const MODES_LONG_MSG_BITS: usize = 112;
pub const MODES_ICAO_CACHE_LEN: u32 = 1024;
pub const MODES_ICAO_CACHE_TTL: u32 = 60;

pub struct Message {
    pub msg: [u8; MODES_LONG_MSG_BITS],
}

impl Default for Message {
    fn default() -> Self {
        Self { msg: [0; 112] }
    }
}

pub enum Unit {
    METERS,
    FEET,
}

impl Default for Unit {
    fn default() -> Self {
        Unit::FEET
    }
}

pub struct IcaoCache {
    pub icao_cache: Vec<u32>,
}

impl Default for IcaoCache {
    fn default() -> Self {
        Self {
            icao_cache: vec![0; (MODES_ICAO_CACHE_LEN * 2) as usize],
        }
    }
}

#[derive(Default)]
pub struct ModeSMessage {
    pub msg: Message,
    pub msgbits: usize,
    pub msgtype: u8,
    pub crcok: bool,
    pub crc: u32,
    pub errorbit: i8,
    pub aa1: u8,
    pub aa2: u8,
    pub aa3: u8,
    pub phase_corrected: i32,

    pub ca: u8,

    pub metype: i32,
    pub mesub: i32,
    pub heading_is_valid: i32,
    pub heading: i32,
    pub aircraft_type: i32,
    pub fflag: i32,
    pub tflag: i32,
    pub raw_latitude: i32,
    pub raw_longitude: i32,
    pub flight: [u8; 9],
    pub ew_dir: i32,
    pub ew_velocity: i32,
    pub ns_dir: i32,
    pub ns_velocity: i32,
    pub vert_rate_source: i32,
    pub vert_rate_sign: i32,
    pub vert_rate: i32,
    pub velocity: i32,

    pub fs: i32,
    pub dr: i32,
    pub um: i32,
    pub squawka: i32,

    pub altitude: i32,
    pub unit: Unit,
    pub icao_cache: IcaoCache,
}
