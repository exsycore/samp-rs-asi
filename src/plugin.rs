use windows::core::w;
use windows::Win32::System::LibraryLoader::GetModuleHandleW;

use crate::samp;

pub fn initialize() {
    std::thread::spawn(|| {
        unsafe {
            let mut hsa_value: usize = 0;
            while hsa_value == 0 {
                if let Ok(hsa) = GetModuleHandleW(w!("samp.dll")) {
                    hsa_value = hsa.0 as usize;
                }
                std::thread::sleep(std::time::Duration::from_millis(100));
            }

            match samp::get_samp_version(hsa_value) {
                Ok(version) => {
                    let (chat_offset, _) = version.chat_offset();
                    let chat_ptr_address = (hsa_value + chat_offset) as *mut *mut usize;
            
                    while (*chat_ptr_address).is_null() {
                        std::thread::sleep(std::time::Duration::from_millis(100));
                    }

                    let msg = format!("SA:MP Rust ASI Plugin Loaded Version: {:?}", version);
                    samp::add_message(hsa_value, version, &msg, 0x00FF00FF);
                }
                #[allow(unused)]
                Err(e) => {
                }
            }
        }
    });
}

pub fn uninitialize() {

}