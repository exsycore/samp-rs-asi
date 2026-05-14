use std::ffi::CString;

use windows::Win32::System::{
    Diagnostics::Debug::IMAGE_NT_HEADERS32, SystemServices::IMAGE_DOS_HEADER,
};

#[derive(Debug, Clone, Copy, PartialOrd, PartialEq)]
pub enum SampVersion {
    V037R1,
    V037R2,
    V037R3,
    V037R3_1,
    V037R4,
    V037R4_2,
    V037R5,
    V03DLR1,
}

pub fn get_samp_version(base_address: usize) -> Result<SampVersion, &'static str> {
    let entry_point = unsafe {
        let dos_header = *(base_address as *const IMAGE_DOS_HEADER);
        let nt_headers =
            *((base_address + (dos_header.e_lfanew as usize)) as *const IMAGE_NT_HEADERS32);

        nt_headers.OptionalHeader.AddressOfEntryPoint
    };

    match entry_point {
        0x31DF13 => Ok(SampVersion::V037R1),
        0x3195DD => Ok(SampVersion::V037R2),
        0xCC490 => Ok(SampVersion::V037R3),
        0xCC4D0 => Ok(SampVersion::V037R3_1),
        0xCBCD0 => Ok(SampVersion::V037R4),
        0xCBCB0 => Ok(SampVersion::V037R4_2),
        0xCBC90 => Ok(SampVersion::V037R5),
        0xFDB60 => Ok(SampVersion::V03DLR1),
        _ => Err("Unknown SA-MP version."),
    }
}

#[allow(dead_code)]
type AddMessageFn = unsafe extern "thiscall" fn(
    this: *mut usize, 
    color: u32, 
    text: *const i8
);

pub fn add_message(base_address: usize, version: SampVersion, message: &str, color: u32) {
    unsafe {
        let (chat_ptr_offset, func_offset) = match version {
            SampVersion::V037R1 => (0x21A0E4, 0x645A0),
            SampVersion::V037R3 => (0x26E8C8, 0x0000f),
            SampVersion::V037R3_1 => (0x26E8C8, 0x679F0),
            SampVersion::V037R5 => (0x26EB80, 0x68170),
            SampVersion::V03DLR1 => (0x2ACA10, 0x67BE0),
            _ => (0x21A0E4, 0x645A0),
        };
        let chat_ptr_ptr = (base_address + chat_ptr_offset) as *mut *mut usize;
        if chat_ptr_ptr.is_null() || (*chat_ptr_ptr).is_null() { return; }
        let chat_ptr = *chat_ptr_ptr;

        let func_addr = (base_address + func_offset) as *const ();
        
        type AddMessageFn = unsafe extern "thiscall" fn(*mut usize, u32, *const i8);
        let add_message: AddMessageFn = std::mem::transmute(func_addr);

        if let Ok(c_str) = CString::new(message) {
            add_message(chat_ptr, color, c_str.as_ptr());
        }
    }
}
