pub mod signal;

// These bits all seem to be from c2rust...
// See https://pubs.opengroup.org/onlinepubs/7908799/xsh/systypes.h.html.

// Ok, so libc thinks that size_t is usize which totally makes sense but does
// not jive at all with the output from c2rust. Changing this would not be a
// refactor for the faint of heart.
pub type size_t = libc::c_ulong;

// These don't seem to exist in libc.
pub type smallint = libc::c_schar;
pub type uoff_t = libc::c_ulong;
pub type __syscall_slong_t = libc::c_long;
pub type __compar_fn_t =
  Option<unsafe extern "C" fn(_: *const libc::c_void, _: *const libc::c_void) -> libc::c_int>;

// See http://man7.org/linux/man-pages/man3/getpwnam.3.html.
#[derive(Copy, Clone)]
#[repr(C)]
pub struct passwd {
  pub pw_name: *mut libc::c_char,
  pub pw_passwd: *mut libc::c_char,
  pub pw_uid: libc::uid_t,
  pub pw_gid: libc::gid_t,
  pub pw_gecos: *mut libc::c_char,
  pub pw_dir: *mut libc::c_char,
  pub pw_shell: *mut libc::c_char,
}

// See https://www.mkssoftware.com/docs/man5/struct_group.5.asp.
#[derive(Copy, Clone)]
#[repr(C)]
pub struct group {
  pub gr_name: *mut libc::c_char,
  pub gr_passwd: *mut libc::c_char,
  pub gr_gid: libc::gid_t,
  pub gr_mem: *mut *mut libc::c_char,
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct termios {
  pub c_iflag: libc::tcflag_t,
  pub c_oflag: libc::tcflag_t,
  pub c_cflag: libc::tcflag_t,
  pub c_lflag: libc::tcflag_t,
  pub c_line: libc::cc_t,
  pub c_cc: [libc::cc_t; 32],
  pub c_ispeed: libc::speed_t,
  pub c_ospeed: libc::speed_t,
}

// See http://www.delorie.com/djgpp/doc/libc/libc_495.html.
#[derive(Copy, Clone)]
#[repr(C)]
pub struct winsize {
  pub ws_row: libc::c_ushort,
  pub ws_col: libc::c_ushort,
  pub ws_xpixel: libc::c_ushort,
  pub ws_ypixel: libc::c_ushort,
}

// ... end c2rust stuff

// Defined in libbb.h
#[derive(Copy, Clone)]
#[repr(C)]
pub struct bb_uidgid_t {
  pub uid: libc::uid_t,
  pub gid: libc::gid_t,
}

// TODO: probably not as readable as a rust tuple.
/* In this form code with pipes is much more readable */
#[derive(Copy, Clone)]
#[repr(C)]
pub struct fd_pair {
  pub rd: libc::c_int,
  pub wr: libc::c_int,
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct md5_ctx_t {
  pub wbuffer: [u8; 64], /* always correctly aligned for uint64_t */
  pub process_block: Option<unsafe extern "C" fn(_: *mut md5_ctx_t) -> ()>,
  pub total64: u64,   /* must be directly before hash[] */
  pub hash: [u32; 8], /* 4 elements for md5, 5 for sha1, 8 for sha256 */
}
pub type sha1_ctx_t = md5_ctx_t;
pub type sha256_ctx_t = md5_ctx_t;

#[derive(Copy, Clone)]
#[repr(C)]
pub struct sha512_ctx_t {
  pub total64: [u64; 2], /* must be directly before hash[] */
  pub hash: [u64; 8],
  pub wbuffer: [u8; 128], /* always correctly aligned for uint64_t */
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct sha3_ctx_t {
  pub state: [u64; 25],
  pub bytes_queued: libc::c_uint,
  pub input_block_bytes: libc::c_uint,
}
// ... end libbb.h stuff
