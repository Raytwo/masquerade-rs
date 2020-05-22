//use ktgl::feth::v120::memory::kt_aligned_malloc;
use lazy_static::lazy_static;
use skyline::alloc::slice;
use skyline::libc::c_void;
use skyline::{hook, install_hook};
use std::collections::HashMap;
use std::fs::File;
use std::io::Read;
use std::path::Path;
use std::sync::Mutex;
use std::{fs, io};

lazy_static! {
    pub static ref FORGE: Mutex<Forge> = Mutex::new(Forge::new());
}

macro_rules! forge {
    () => {
        FORGE.lock().unwrap()
    };
}

pub struct Forge {
    cache: HashMap<u32, String>,
}

impl Forge {
    pub fn get_path_for_fileid(&self, fileid: u32) -> Option<&String> {
        self.cache.get(&fileid)
    }

    pub fn get_filesize_for_fileid(&self, fileid: u32) -> Option<u64> {
        let cached_path = self.get_path_for_fileid(fileid).unwrap();
        let file = File::open(cached_path).unwrap();
        let metadata = file.metadata().unwrap();

        if metadata.is_dir() {
            return None;
        }

        Some(metadata.len())
    }

    pub fn get_fileids(&mut self) -> Vec<u32> {
        let mut fileids = Vec::new();
        for (fileid, _) in self.cache.iter_mut() {
            fileids.push(*fileid);
        }
        fileids
    }

    /// Called by the forge! macro, do not call this yourself
    fn new() -> Forge {
        Forge {
            cache: HashMap::new(),
        }
    }

    fn try_load(
        &self,
        fileid: u32,
        _out_ptr: *const skyline::libc::c_void, // Unused by our system
        file_content: *const skyline::libc::c_void, // The pointer to the original file in memory
    ) -> Option<*const c_void> {
        // Check if we have a path associated with this FileID. If not, return None.
        let cached_path = self.get_path_for_fileid(fileid)?;
        println!(
            "[Iwai] FileID {} loaded from {}. You should be grateful.",
            fileid, cached_path
        );
        let mut file = File::open(cached_path).ok()?;
        let metadata = file.metadata().ok()?;

        // Make sure we didn't try to open a directory instead
        if metadata.is_dir() {
            return None;
        }

        unsafe {
            // Convert the file content as a slice to write over it
            let content =
                slice::from_raw_parts_mut(file_content as *mut u8, metadata.len() as usize);
            file.read_exact(content).ok()?;

            // Return a pointer to the slice
            Some(content.as_ptr() as *const c_void)
        }
    }
}

// The method used by the game to load a file using a FileID
#[hook(offset = 0x3CB840)]
fn hook_load_with_file_id(
    fileid: u32,
    out_ptr: *const skyline::libc::c_void,
) -> *const skyline::libc::c_void {
    // We call this first so the game does all the memory allocation for us. Sure it's a bit of a waste of performances, but it's easier.
    let file_content = original!()(fileid, out_ptr);
    match forge!().try_load(fileid, out_ptr, file_content) {
        None => {
            // We don't have that FileID in the cache, let the game proceed normally.
            println!("[Iwai] FileID {} loaded.", fileid);
            file_content
        }
        Some(ptr) => ptr, // The file was patched, return its content.
    }
}

// Big thanks to Kolakcc (https://github.com/kolakcc) for his help with the current implementation.
pub fn init_forge() {
    install_hook!(hook_load_with_file_id);

    // we write a custom implementation of a directory tree walker because the implementation does not behave like spec
    // which will make walkdir and friends fail

    fn visit_file(real_path: String, filename: String) {
        let index_options = vec![
            filename.parse::<u32>().ok(),
            filename
                .split("-")
                .nth(0)
                .and_then(|x| x.parse::<u32>().ok()),
        ];
        if let Some(index) = index_options.iter().filter_map(|&x| x).nth(0) {
            println!(
                "[Iwai] Discovered {} => {}. I guess that's fine?",
                index, real_path
            );
            forge!().cache.insert(index, real_path);
        }
    }

    fn visit_dirs(dir: &Path) -> io::Result<()> {
        if dir.is_dir() {
            for entry in fs::read_dir(dir)? {
                let entry = entry?;
                let filename = entry.path(); // this looks wrong but is actually right
                let real_path = format!("{}/{}", dir.to_str().unwrap(), filename.to_str().unwrap());
                let path = Path::new(&real_path);
                if path.is_dir() {
                    visit_dirs(&path)?;
                } else {
                    visit_file(real_path, filename.to_str().unwrap().to_string());
                }
            }
        }
        Ok(())
    }
    visit_dirs(Path::new("sd:/Masquerade/forge")).ok();
}
