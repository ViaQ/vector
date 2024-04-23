use super::super::*;
use libc::*;

extern "C" {
    #[cfg(ossl300)]
    pub fn OSSL_PARAM_construct_uint(key: *const c_char, buf: *mut c_uint) -> OSSL_PARAM;
    #[cfg(ossl300)]
    pub fn OSSL_PARAM_construct_end() -> OSSL_PARAM;
}
