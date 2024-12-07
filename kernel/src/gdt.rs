pub static GDT: &[u64] = &[
    0,
    add_entry(0, 0xFFFFF, 0xA0, 0x9B),
    add_entry(0, 0xFFFFF, 0xA0, 0x93),
];

const fn add_entry(base: u32, limit: u32, access: u8, flag: u8) -> u64
{
    let mut descriptor: u64;
    let base: u64 = base as u64;
    let limit = limit as u64;
    let flag = flag as u64;
    let access = access as u64;

    descriptor  =  limit         & 0x000F0000;
    descriptor |= (flag <<  8)   & 0x0000FF00;
    descriptor |= (access << 16) & 0x00F00000;
    descriptor |= (base >> 16)   & 0x000000FF;
    descriptor |=  base          & 0xFF000000;

    descriptor <<= 32;

    descriptor |= base  << 16;
    descriptor |= limit  & 0x0000FFFF;

    descriptor
}
