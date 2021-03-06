extern crate winapi;
extern crate kernel32;
extern crate windows_named_pipe;

use std::ffi::CStr;
use std::os::windows::io::{AsRawHandle, FromRawHandle};
use self::winapi::PROCESSENTRY32;
use std::io::{Error, ErrorKind, Result};

pub use self::windows_named_pipe::PipeStream as RawConn;
use discord_ipc;


pub const CHROME_NAME : &str = "chrome.exe";
pub const WINDOWS_PIPE_ADDR: &str = "//./pipe/discord-ipc-0";

impl discord_ipc::Connectable<RawConn> for RawConn {
    fn raw_connect() -> Result<RawConn> {
        RawConn::connect(WINDOWS_PIPE_ADDR)
    }
}

pub fn pid_by_name<S: Into<String>>(name_query: S) -> Result<i32> {
    let name_query = name_query.into();
    let handle: winapi::winnt::HANDLE =
        unsafe { kernel32::CreateToolhelp32Snapshot(winapi::TH32CS_SNAPPROCESS, 0) };
    if handle == winapi::INVALID_HANDLE_VALUE {
        return Err(Error::last_os_error());
    }
    let file = unsafe { ::std::fs::File::from_raw_handle(handle) };
    let handle = file.as_raw_handle();

    let mut proc_ent = unsafe { ::std::mem::uninitialized::<PROCESSENTRY32>() };
    proc_ent.dwSize = ::std::mem::size_of::<winapi::tlhelp32::PROCESSENTRY32>() as u32;

    let first_result = unsafe { kernel32::Process32First(handle, &mut proc_ent) };
    if first_result == 0 {
        return Err(Error::last_os_error());
    }

    while unsafe { kernel32::Process32Next(handle, &mut proc_ent) } != 0 {
        let name = unsafe { CStr::from_ptr(proc_ent.szExeFile.as_ptr()) };

        if name_query == name.to_str().unwrap() {
            return Ok(proc_ent.th32ProcessID as i32);
        }
    }

    let last_error = unsafe { kernel32::GetLastError() };
    if last_error != winapi::ERROR_NO_MORE_FILES {
        return Err(Error::from_raw_os_error(last_error as i32));
    }
    return Err(Error::new(ErrorKind::Other, "Not found"));
}
