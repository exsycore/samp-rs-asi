use std::thread;
use windows::Win32::{
    Foundation::{HMODULE},
    System::{
        LibraryLoader::DisableThreadLibraryCalls,
        SystemServices::{DLL_PROCESS_ATTACH, DLL_PROCESS_DETACH},
    },
};

pub mod plugin;
pub mod samp;

#[unsafe(no_mangle)]
extern "system" fn DllMain(instance: HMODULE, reason: u32, _reserved: *mut core::ffi::c_void) -> bool {
    match reason {
        DLL_PROCESS_ATTACH => {
            unsafe {
                let _ = DisableThreadLibraryCalls(instance).unwrap();
            }
            thread::spawn(|| {
                plugin::initialize();
            });
        }
        DLL_PROCESS_DETACH => {
            plugin::uninitialize();
        }
        _ => {}
    }
    true
}