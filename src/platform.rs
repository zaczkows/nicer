#[cfg(unix)]
pub fn set_priority(priority: i8) {
    unsafe {
        crate::raw::nice(priority as libc::c_int);
    }
}

pub fn exec_cmd(cmd_params: &[String]) -> bool {
    use std::ffi::CString;
    // Need to keep the copy while working with pointers!
    let cparams: Vec<CString> = cmd_params
        .iter()
        .map(|item| CString::new(item.to_string()).expect("CString failed"))
        .collect();
    let params: Vec<*const libc::c_char> = cparams
        .iter()
        .map(|item| item.as_ptr())
        .chain(std::iter::once(std::ptr::null()))
        .collect();
    let command = params[0];
    let error = unsafe { libc::execvp(command, params.as_ptr()) };
    unsafe {
        libc::perror(std::ptr::null());
    }
    error != -1
}

#[cfg(windows)]
pub fn set_priority(priority: i8) {
    use winapi::um::processthreadsapi::*;
    use winapi::um::winbase::*;
    let prio = if priority > 0 {
        BELOW_NORMAL_PRIORITY_CLASS
    } else if priority == 0 {
        NORMAL_PRIORITY_CLASS
    } else {
        HIGH_PRIORITY_CLASS
    };
    let _ret = unsafe { SetPriorityClass(GetCurrentProcess(), prio) };
}
