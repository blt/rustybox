use crate::librb::procps_status_t;
use crate::librb::size_t;
use libc;
use libc::closedir;
use libc::dirent;
use libc::free;
use libc::getpid;
use libc::opendir;
use libc::printf;
use libc::readdir;
use libc::sprintf;
use libc::DIR;

pub type C2RustUnnamed = libc::c_uint;
pub const PSSCAN_TASKS: C2RustUnnamed = 4194304;
pub const PSSCAN_RUIDGID: C2RustUnnamed = 2097152;
pub const PSSCAN_NICE: C2RustUnnamed = 1048576;
pub const PSSCAN_CPU: C2RustUnnamed = 524288;
pub const PSSCAN_START_TIME: C2RustUnnamed = 262144;
pub const PSSCAN_CONTEXT: C2RustUnnamed = 0;
pub const PSSCAN_ARGVN: C2RustUnnamed = 65536;
pub const PSSCAN_SMAPS: C2RustUnnamed = 32768;
pub const PSSCAN_TTY: C2RustUnnamed = 16384;
pub const PSSCAN_UTIME: C2RustUnnamed = 8192;
pub const PSSCAN_STIME: C2RustUnnamed = 4096;
pub const PSSCAN_RSS: C2RustUnnamed = 2048;
pub const PSSCAN_VSZ: C2RustUnnamed = 1024;
pub const PSSCAN_STATE: C2RustUnnamed = 512;
pub const PSSCAN_EXE: C2RustUnnamed = 256;
pub const PSSCAN_ARGV0: C2RustUnnamed = 128;
pub const PSSCAN_COMM: C2RustUnnamed = 32;
pub const PSSCAN_UIDGID: C2RustUnnamed = 16;
pub const PSSCAN_SID: C2RustUnnamed = 8;
pub const PSSCAN_PGID: C2RustUnnamed = 4;
pub const PSSCAN_PPID: C2RustUnnamed = 2;
pub const PSSCAN_PID: C2RustUnnamed = 1;

/*
 * Mini lsof implementation for busybox
 *
 * Copyright (C) 2012 by Sven Oliver 'SvOlli' Moll <svolli@svolli.de>
 *
 * Licensed under GPLv2, see file LICENSE in this source tree.
 */
//config:config LSOF
//config:	bool "lsof (3.4 kb)"
//config:	default y
//config:	help
//config:	Show open files in the format of:
//config:	PID <TAB> /path/to/executable <TAB> /path/to/opened/file
//applet:IF_LSOF(APPLET(lsof, BB_DIR_USR_BIN, SUID_DROP))
//kbuild:lib-$(CONFIG_LSOF) += lsof.o
//usage:#define lsof_trivial_usage
//usage:       ""
//usage:#define lsof_full_usage "\n\n"
//usage:       "Show all open files"
/*
 * Examples of "standard" lsof output:
 *
 * COMMAND    PID USER   FD   TYPE             DEVICE     SIZE       NODE NAME
 * init         1 root  cwd    DIR                8,5     4096          2 /
 * init         1 root  rtd    DIR                8,5     4096          2 /
 * init         1 root  txt    REG                8,5   872400      63408 /app/busybox-1.19.2/busybox
 * rpc.portm 1064 root  mem    REG                8,5    43494      47451 /app/glibc-2.11/lib/libnss_files-2.11.so
 * rpc.portm 1064 root    3u  IPv4               2178                 UDP *:111
 * rpc.portm 1064 root    4u  IPv4               1244                 TCP *:111 (LISTEN)
 * runsvdir  1116 root    0r   CHR                1,3                1214 /dev/null
 * runsvdir  1116 root    1w   CHR                1,3                1214 /dev/null
 * runsvdir  1116 root    2w   CHR                1,3                1214 /dev/null
 * runsvdir  1116 root    3r   DIR                8,6     1560      58359 /.local/var/service
 * gpm       1128 root    4u  unix 0xffff88007c09ccc0                1302 /dev/gpmctl
 */
#[no_mangle]
pub unsafe extern "C" fn lsof_main(
  mut _argc: libc::c_int,
  mut _argv: *mut *mut libc::c_char,
) -> libc::c_int {
  let mut proc_0: *mut procps_status_t = std::ptr::null_mut();
  loop {
    proc_0 = crate::libbb::procps::procps_scan(
      proc_0,
      PSSCAN_PID as libc::c_int | PSSCAN_EXE as libc::c_int,
    );
    if proc_0.is_null() {
      break;
    }
    let mut name: [libc::c_char; 35] = [0; 35];
    let mut baseofs: libc::c_uint = 0;
    let mut d_fd: *mut DIR = std::ptr::null_mut();
    let mut fdlink: *mut libc::c_char = std::ptr::null_mut::<libc::c_char>();
    let mut entry: *mut dirent = std::ptr::null_mut();
    if getpid() as libc::c_uint == (*proc_0).pid {
      continue;
    }
    baseofs = sprintf(
      name.as_mut_ptr(),
      b"/proc/%u/fd/\x00" as *const u8 as *const libc::c_char,
      (*proc_0).pid,
    ) as libc::c_uint;
    d_fd = opendir(name.as_mut_ptr());
    if !d_fd.is_null() {
      loop {
        entry = readdir(d_fd);
        if entry.is_null() {
          break;
        }
        /* Skip entries '.' and '..' (and any hidden file) */
        if (*entry).d_name[0] as libc::c_int == '.' as i32 {
          continue;
        }
        crate::libbb::safe_strncpy::safe_strncpy(
          name.as_mut_ptr().offset(baseofs as isize),
          (*entry).d_name.as_mut_ptr(),
          10i32 as size_t,
        );
        fdlink = crate::libbb::xreadlink::xmalloc_readlink(name.as_mut_ptr());
        if !fdlink.is_null() {
          printf(
            b"%d\t%s\t%s\n\x00" as *const u8 as *const libc::c_char,
            (*proc_0).pid,
            (*proc_0).exe,
            fdlink,
          );
          free(fdlink as *mut libc::c_void);
        }
      }
      closedir(d_fd);
    }
  }
  return 0;
}
