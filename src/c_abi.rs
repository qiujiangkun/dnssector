use constants::*;
use libc::{c_int, c_void};
use parsed_packet::*;
use edns_iterator::*;
use question_iterator::*;
use response_iterator::*;
use rr_iterator::*;
use std::convert::From;
use std::ptr;

const ABI_VERSION: u64 = 0x1;

unsafe extern "C" fn flags(parsed_packet: *const ParsedPacket) -> u32 {
    (*parsed_packet).flags()
}

unsafe extern "C" fn set_flags(parsed_packet: *mut ParsedPacket, flags: u32) {
    (*parsed_packet).set_flags(flags)
}

unsafe extern "C" fn rcode(parsed_packet: *const ParsedPacket) -> u8 {
    (*parsed_packet).rcode()
}

unsafe extern "C" fn set_rcode(parsed_packet: *mut ParsedPacket, rcode: u8) {
    (*parsed_packet).set_rcode(rcode)
}

unsafe extern "C" fn opcode(parsed_packet: *const ParsedPacket) -> u8 {
    (*parsed_packet).opcode()
}

unsafe extern "C" fn set_opcode(parsed_packet: *mut ParsedPacket, opcode: u8) {
    (*parsed_packet).set_opcode(opcode)
}

const SECTION_ITERATOR_MAGIC: u64 = 0x08af4e661b92fda3;

#[repr(C)]
pub struct SectionIterator {
    magic: u64,
    section: Section,
    it: *mut c_void,
}

unsafe extern "C" fn iter_answer(
    parsed_packet: *mut ParsedPacket,
    cb: unsafe extern "C" fn(ctx: *mut c_void, section_iterator: *const SectionIterator) -> bool,
    ctx: *mut c_void,
) {
    let mut it = (*parsed_packet).into_iter_answer();
    while let Some(mut item) = it {
        let section_iterator = SectionIterator {
            magic: SECTION_ITERATOR_MAGIC,
            section: Section::Answer,
            it: &mut item as *mut _ as *mut _,
        };
        if (cb)(ctx, &section_iterator as *const _ as *const _) {
            break;
        }
        it = item.next();
    }
}

unsafe extern "C" fn iter_nameservers(
    parsed_packet: *mut ParsedPacket,
    cb: unsafe extern "C" fn(ctx: *mut c_void, section_iterator: *const SectionIterator) -> bool,
    ctx: *mut c_void,
) {
    let mut it = (*parsed_packet).into_iter_nameservers();
    while let Some(mut item) = it {
        let section_iterator = SectionIterator {
            magic: SECTION_ITERATOR_MAGIC,
            section: Section::NameServers,
            it: &mut item as *mut _ as *mut _,
        };
        if (cb)(ctx, &section_iterator as *const _ as *const _) {
            break;
        }
        it = item.next();
    }
}

unsafe extern "C" fn iter_additional(
    parsed_packet: *mut ParsedPacket,
    cb: unsafe extern "C" fn(ctx: *mut c_void, section_iterator: *const SectionIterator) -> bool,
    ctx: *mut c_void,
) {
    let mut it = (*parsed_packet).into_iter_additional();
    while let Some(mut item) = it {
        let section_iterator = SectionIterator {
            magic: SECTION_ITERATOR_MAGIC,
            section: Section::Additional,
            it: &mut item as *mut _ as *mut _,
        };
        if (cb)(ctx, &section_iterator as *const _ as *const _) {
            break;
        }
        it = item.next();
    }
}

unsafe extern "C" fn iter_edns(
    parsed_packet: *mut ParsedPacket,
    cb: unsafe extern "C" fn(ctx: *mut c_void, section_iterator: *const EdnsIterator) -> bool,
    ctx: *mut c_void,
) {
    let mut it = (*parsed_packet).into_iter_edns();
    while let Some(mut item) = it {
        let section_iterator = SectionIterator {
            magic: SECTION_ITERATOR_MAGIC,
            section: Section::Edns,
            it: &mut item as *mut _ as *mut _,
        };
        if (cb)(ctx, &section_iterator as *const _ as *const _) {
            break;
        }
        it = item.next();
    }
}

unsafe extern "C" fn rr_type(section_iterator: &mut SectionIterator) -> u16 {
    assert_eq!(section_iterator.magic, SECTION_ITERATOR_MAGIC);
    match section_iterator.section {
        Section::Question => (&*(section_iterator.it as *mut QuestionIterator)).rr_type(),
        Section::Answer | Section::NameServers | Section::Additional => {
            (&*(section_iterator.it as *mut ResponseIterator)).rr_type()
        }
        _ => panic!("rr_type() called on a record with no type"),
    }
}

/// C wrappers to the internal API
#[repr(C)]
pub struct FnTable {
    pub abi_version: u64,
    pub flags: unsafe extern "C" fn(parsed_packet: *const ParsedPacket) -> u32,
    pub set_flags: unsafe extern "C" fn(parsed_packet: *mut ParsedPacket, flags: u32),
    pub rcode: unsafe extern "C" fn(parsed_packet: *const ParsedPacket) -> u8,
    pub set_rcode: unsafe extern "C" fn(parsed_packet: *mut ParsedPacket, rcode: u8),
    pub opcode: unsafe extern "C" fn(parsed_packet: *const ParsedPacket) -> u8,
    pub set_opcode: unsafe extern "C" fn(parsed_packet: *mut ParsedPacket, opcode: u8),
    pub iter_answer:
        unsafe extern "C" fn(parsed_packet: *mut ParsedPacket,
                             cb: unsafe extern "C" fn(ctx: *mut c_void,
                                                      section_iterator: *const SectionIterator)
                                                      -> bool,
                             *mut c_void),
    pub iter_nameservers:
        unsafe extern "C" fn(parsed_packet: *mut ParsedPacket,
                             cb: unsafe extern "C" fn(ctx: *mut c_void,
                                                      section_iterator: *const SectionIterator)
                                                      -> bool,
                             *mut c_void),
    pub iter_additional:
        unsafe extern "C" fn(parsed_packet: *mut ParsedPacket,
                             cb: unsafe extern "C" fn(ctx: *mut c_void,
                                                      section_iterator: *const SectionIterator)
                                                      -> bool,
                             *mut c_void),
    pub iter_edns:
        unsafe extern "C" fn(parsed_packet: *mut ParsedPacket,
                             cb: unsafe extern "C" fn(ctx: *mut c_void,
                                                      section_iterator: *const EdnsIterator)
                                                      -> bool,
                             *mut c_void),
    pub rr_type: unsafe extern "C" fn(section_iterator: &mut SectionIterator) -> u16,
}

pub fn fn_table() -> FnTable {
    FnTable {
        abi_version: ABI_VERSION,
        flags,
        set_flags,
        rcode,
        set_rcode,
        opcode,
        set_opcode,
        iter_answer,
        iter_nameservers,
        iter_additional,
        iter_edns,
        rr_type,
    }
}
