#![feature(proc_macro_hygiene)]

use skyline::{hook, hooks, install_hooks};
use std::slice;
#[macro_use]
mod forge;
use crate::forge::FORGE;

// Latest update: 1.0.3
const LINKDATA_ENTRY_COUNT: usize = 7321;

#[repr(C)]
pub struct LinkdataEntry {
    uncompressed_size: u32,
    compressed_size: u32,
    offset: u64,
    unk1: u16,
    compressed: bool,
    padding: u8,
    unk2: u32,
}

/// KTGL::IO::Data::ParseLinkdata
#[hook(offset = 0x3CC370)]
fn hook_initialize_linkdata_table() -> u32 {
    println!("[Chihaya] Intercepted Linkdata parsing...");

    // It is necessary to call the original function in order to let the game populate the table.
    let result = original!()();

    unsafe {
        // Get the LINKDATA table in memory and convert it to a slice for easy editing.

        let text_address = hooks::getRegionAddress(hooks::Region::Text) as *mut u8;
        let array_ptr = text_address.offset(0x22458C8);
        let linkdata_entries =
            slice::from_raw_parts_mut(array_ptr as *mut LinkdataEntry, LINKDATA_ENTRY_COUNT);

        // We need to put it in a separate variable first because it'll die if we use the forge!() macro inside of an iterator.
        let fileids = forge!().get_fileids();

        // Patch the size of every FileID in the LINKDATA table.
        for fileid in fileids {
            let id = fileid as usize;

            let filesize = match forge!().get_filesize_for_fileid(fileid) {
                None => {
                    println!(
                        "[Chihaya] Filesize for FileID {} is 0. Your arcana might be broken.",
                        id
                    );
                    0
                }
                Some(size) => size,
            };

            linkdata_entries[id].uncompressed_size = filesize as _;
            linkdata_entries[id].compressed_size = filesize as _;
            linkdata_entries[id].compressed = false;

            println!("[Chihaya] Successfully edited entry {} in LINKDATA", id);
        }
    }

    println!("[Chihaya] Table patched successfully. Fortune shines upon you!");
    result
}

// Necessary to stop the game from crashing when loading thanks to Skyline using core 3
#[hook(offset = 0x12BB330)]
fn fake_core_number() -> u64 {
    0
}

// TODO: Add this method to skyline-rs
// nn::oe::SetCopyrightVisibility
#[hook(offset = 0x12BB260)]
fn hook_set_copyright_visibility(_is_visible: bool) {
    // Disable copyright image on screenshots because why the hell is this acceptable to begin with.
    original!()(false)
}

#[skyline::main(name = "masquerade")]
pub fn main() {
    println!(
        "Masquerade-rs v{} - Persona 5 Scramble file replacement plugin",
        env!("CARGO_PKG_VERSION")
    );

    install_hooks!(
        fake_core_number,
        hook_set_copyright_visibility,
        hook_initialize_linkdata_table
    );

    forge::init_forge();
}
