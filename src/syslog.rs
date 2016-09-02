use libc::{self, c_int};
use std::ffi::CStr;

pub fn openlog(ident: &'static CStr, option: Option, facility: Facility) {
    unsafe { libc::openlog(ident.as_ptr(), option.bits(), facility.bits()); }
}

pub fn syslog(facility: Facility, level: Level, message: &CStr) {
    unsafe {
        let priority = facility.bits() | level as c_int;
        let format = b"%s\0".as_ptr() as *const i8;
        libc::syslog(priority, format, message.as_ptr());
    }
}

pub fn closelog() {
    unsafe { libc::closelog(); }
}

libc_bitflags! {
    flags Option: c_int {
        LOG_CONS,
        LOG_NDELAY,
        LOG_NOWAIT,
        LOG_ODELAY,
        LOG_PERROR,
        LOG_PID,
    }
}

libc_bitflags! {
    flags Facility: c_int {
        LOG_AUTH,
        LOG_AUTHPRIV,
        LOG_CRON,
        LOG_DAEMON,
        LOG_FTP,
        LOG_KERN,
        LOG_LOCAL0,
        LOG_LOCAL1,
        LOG_LOCAL2,
        LOG_LOCAL3,
        LOG_LOCAL4,
        LOG_LOCAL5,
        LOG_LOCAL6,
        LOG_LOCAL7,
        LOG_LPR,
        LOG_MAIL,
        LOG_NEWS,
        LOG_SYSLOG,
        LOG_USER,
        LOG_UUCP,
    }
}

#[repr(i32)]
pub enum Level {
    LOG_EMERG = libc::LOG_EMERG,
    LOG_ALERT = libc::LOG_ALERT,
    LOG_CRIT = libc::LOG_CRIT,
    LOG_ERR = libc::LOG_ERR,
    LOG_WARNING = libc::LOG_WARNING,
    LOG_NOTICE = libc::LOG_NOTICE,
    LOG_INFO = libc::LOG_INFO,
    LOG_DEBUG = libc::LOG_DEBUG,
}
