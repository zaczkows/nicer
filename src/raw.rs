#[cfg(unix)]
extern "C" {
    pub fn nice(inc: libc::c_int) -> libc::c_int;
}
