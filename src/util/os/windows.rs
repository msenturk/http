use winapi::um::fileapi::GetFileAttributesW;
use std::os::windows::fs::MetadataExt;
use std::os::windows::ffi::OsStrExt;
use std::fs::Metadata;
use std::path::Path;


/// Get windows-style attributes for the specified file
///
/// https://docs.microsoft.com/en-gb/windows/win32/fileio/file-attribute-constants
pub fn win32_file_attributes(_: &Metadata, path: &Path) -> u32 {
    let mut buf: Vec<_> = path.as_os_str().encode_wide().collect();
    buf.push(0);

    unsafe { GetFileAttributesW(buf.as_ptr()) }
}


/// `st_dev`-`st_ino`-`st_mtim`
pub fn file_etag(m: &Metadata) -> String {
    format!("{:x}-{}-{}",
            m.volume_serial_number().unwrap_or(0),
            m.file_index().unwrap_or(0),
            m.last_write_time())
}


/// Check if file is marked executable
#[inline(always)]
pub fn file_executable(_: &Metadata) -> bool {
    true
}
