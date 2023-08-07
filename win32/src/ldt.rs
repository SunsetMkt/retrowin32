use std::ffi::c_int;

// https://en.wikipedia.org/wiki/Segment_descriptor
// little endian, low bits first
#[repr(C)]
#[allow(non_snake_case)]
#[allow(non_camel_case_types)]
struct LDT_ENTRY {
    LimitLow: u16,
    BaseLow: u16,
    BaseMid: u8,

    /*
    unsigned    Type : 5;
    unsigned    Dpl : 2;
    unsigned    Pres : 1;
    */
    Flags1: u8,
    /*
    unsigned    LimitHi : 4;
    unsigned    Sys : 1;
    unsigned    Reserved_0 : 1;
    unsigned    Default_Big : 1;
    unsigned    Granularity : 1;
    */
    Flags2: u8,

    BaseHi: u8,
}

impl std::fmt::Debug for LDT_ENTRY {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("LDT_ENTRY")
            .field("LimitLow", &self.LimitLow)
            .field("BaseLow", &self.BaseLow)
            .field("BaseMid", &self.BaseMid)
            .field("Type", &(self.Flags1 & 0x1F))
            .field("Dpl", &((self.Flags1 >> 5) & 0x3))
            .field("Pres", &((self.Flags1 >> 7) & 0x1))
            .field("LimitHi", &(self.Flags2 & 0xF))
            .field("Sys", &((self.Flags2 >> 4) & 0x1))
            .field("Default_Big", &((self.Flags2 >> 6) & 0x1))
            .field("Granularity", &((self.Flags2 >> 7) & 0x1))
            .field("BaseHi", &self.BaseHi)
            .finish()
    }
}

extern "C" {
    fn i386_get_ldt(start_sel: c_int, descs: *mut LDT_ENTRY, num_sels: c_int) -> c_int;
    fn i386_set_ldt(start_sel: c_int, descs: *const LDT_ENTRY, num_sels: c_int) -> c_int;
}

pub unsafe fn setup_ldt() {
    let base: u32 = 0;
    let limit: u32 = 0xFFFF_FFFF;
    let limit_pages = limit >> 12;

    // type bits:
    // 10EWA
    //  E: expand-down
    //  W: writeable
    //  A: accessed
    let type_ = 0b11011u8;
    let dpl = 3u8;
    let pres = 1u8;
    let flags1 = (pres << 7) | (dpl << 5) | type_;

    let limit_hi = ((limit_pages >> 16) & 0xF) as u8;
    let sys = 0u8;
    let default_big = 1u8;
    let granularity: u8 = 1u8;
    let flags2: u8 = (granularity << 7) | (default_big << 6) | (sys << 4) | limit_hi;

    let entry = LDT_ENTRY {
        BaseLow: base as u16,
        BaseMid: (base >> 16) as u8,
        BaseHi: (base >> 24) as u8,
        LimitLow: limit_pages as u16,
        Flags1: flags1,
        Flags2: flags2,
    };
    println!("entry: {:x?}", entry);
    let ret = i386_set_ldt(32, &entry, 1);
    println!("ldt: {}", ret);

    let mut entries: [LDT_ENTRY; 256] = std::mem::zeroed();
    let ret = i386_get_ldt(0, &mut entries as *mut LDT_ENTRY, 256);
    println!("existing: {ret}");
    for (i, e) in entries.iter().enumerate() {
        if e.BaseLow == 0 && e.Flags1 == 0 && e.Flags2 == 0 {
            continue;
        }
        println!("{} {:x?}", i, e);
    }
}
