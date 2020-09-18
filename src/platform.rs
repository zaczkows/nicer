#[cfg(unix)]
pub fn set_priority(priority: i8) {
    unsafe {
        crate::raw::nice(priority as libc::c_int);
    }
}

#[cfg(unix)]
pub fn exec_cmd(cmd_params: &Vec<String>) -> bool {
    use std::ffi::CString;
    // Need to keep the copy while working with pointers!
    let cparams: Vec<CString> = cmd_params
        .iter()
        .map(|item| CString::new(item.to_string()).expect("CString failed"))
        .collect();
    let mut params: Vec<*const libc::c_char> = cparams.iter().map(|item| item.as_ptr()).collect();
    params.push(std::ptr::null());
    let command = params[0];
    let error = unsafe { libc::execvp(command, params.as_ptr()) };
    unsafe {
        libc::perror(std::ptr::null());
    }
    error != -1
}

#[cfg(windows)]
use winapi;
