use libc;

/* * applets.h - a listing of all busybox applets.
*
* If you write a new applet, you need to add an entry to this list to make
* busybox aware of it.
*/
/*
name  - applet name as it is typed on command line
help  - applet name, converted to C (ether-wake: help = ether_wake)
main  - corresponding <applet>_main to call (bzcat: main = bunzip2)
l     - location to install link to: [/usr]/[s]bin
s     - suid type:
        SUID_REQUIRE: will complain if busybox isn't suid
        and is run by non-root (applet_main() will not be called at all)
        SUID_DROP: will drop suid prior to applet_main()
        SUID_MAYBE: neither of the above
        (every instance of SUID_REQUIRE and SUID_MAYBE
        needs to be justified in comment)
        NB: please update FEATURE_SUID help text whenever you add/remove
        SUID_REQUIRE or SUID_MAYBE applet.
*/

#[derive(Copy, Clone)]
pub enum InstallLoc {
  DIR_USR_SBIN,
  DIR_USR_BIN,
  DIR_SBIN,
  DIR_BIN,
  DIR_ROOT,
}

pub enum Entrypoint {
  CStyle(unsafe fn(_: libc::c_int, _: *mut *mut libc::c_char) -> libc::c_int),
  SafeStyle(fn(&[&str]) -> !),
}

pub struct applet {
  pub name: &'static str,
  pub main: &'static str,
  pub entrypoint: Entrypoint,
  pub install_loc: InstallLoc,
  pub usage: &'static str,
}

lazy_static! {
  pub static ref applets: Vec<applet> = {
    let mut appy_mcappface: Vec<applet> = Vec::new();

    #[cfg(feature = "test-bracket1")]
    appy_mcappface.push(applet {
      name: "[",
      main: "test",
      entrypoint: Entrypoint::CStyle(crate::coreutils::test::test_main),
      install_loc: InstallLoc::DIR_USR_BIN,
      usage: std::include_str!("../usage/["),
    });
    #[cfg(feature = "test-bracket2")]
    appy_mcappface.push(applet {
      name: "[[",
      main: "test",
      entrypoint: Entrypoint::CStyle(crate::coreutils::test::test_main),
      install_loc: InstallLoc::DIR_USR_BIN,
      usage: std::include_str!("../usage/[["),
    });
    #[cfg(feature = "acpid")]
    appy_mcappface.push(applet {
      name: "acpid",
      main: "acpid",
      entrypoint: Entrypoint::CStyle(acpcrate::coreutils::id::id_main),
      install_loc: InstallLoc::DIR_SBIN,
      usage: std::include_str!("../usage/acpid"),
    });
    #[cfg(feature = "add-shell")]
    appy_mcappface.push(applet {
      name: "add-shell",
      main: "add_remove_shell",
      entrypoint: Entrypoint::CStyle(crate::loginutils::add_remove_shell::add_remove_shell_main),
      install_loc: InstallLoc::DIR_USR_SBIN,
      usage: std::include_str!("../usage/add-shell"),
    });
    #[cfg(feature = "addgroup")]
    appy_mcappface.push(applet {
      name: "addgroup",
      main: "addgroup",
      entrypoint: Entrypoint::CStyle(crate::loginutils::addgroup::addgroup_main),
      install_loc: InstallLoc::DIR_USR_SBIN,
      usage: std::include_str!("../usage/addgroup"),
    });
    #[cfg(feature = "adduser")]
    appy_mcappface.push(applet {
      name: "adduser",
      main: "adduser",
      entrypoint: Entrypoint::CStyle(crate::loginutils::adduser::adduser_main),
      install_loc: InstallLoc::DIR_USR_SBIN,
      usage: std::include_str!("../usage/adduser"),
    });
    #[cfg(feature = "adjtimex")]
    appy_mcappface.push(applet {
      name: "adjtimex",
      main: "adjtimex",
      entrypoint: Entrypoint::CStyle(crate::miscutils::adjtimex::adjtimex_main),
      install_loc: InstallLoc::DIR_SBIN,
      usage: std::include_str!("../usage/adjtimex"),
    });
    #[cfg(feature = "arch")]
    appy_mcappface.push(applet {
      name: "arch",
      main: "uname",
      entrypoint: Entrypoint::CStyle(crate::coreutils::uname::uname_main),
      install_loc: InstallLoc::DIR_BIN,
      usage: std::include_str!("../usage/arch"),
    });
    #[cfg(feature = "arp")]
    appy_mcappface.push(applet {
      name: "arp",
      main: "arp",
      entrypoint: Entrypoint::CStyle(crate::networking::arp::arp_main),
      install_loc: InstallLoc::DIR_SBIN,
      usage: std::include_str!("../usage/arp"),
    });
    #[cfg(feature = "arping")]
    appy_mcappface.push(applet {
      name: "arping",
      main: "arping",
      entrypoint: Entrypoint::CStyle(crate::networking::arping::arcrate::networking::ping::ping_main),
      install_loc: InstallLoc::DIR_USR_SBIN,
      usage: std::include_str!("../usage/arping"),
    });
    #[cfg(feature = "ash")]
    appy_mcappface.push(applet {
      name: "ash",
      main: "ash",
      entrypoint: Entrypoint::CStyle(crate::shell::ash::ash_main),
      install_loc: InstallLoc::DIR_BIN,
      usage: std::include_str!("../usage/ash"),
    });
    #[cfg(feature = "awk")]
    appy_mcappface.push(applet {
      name: "awk",
      main: "awk",
      entrypoint: Entrypoint::CStyle(crate::editors::awk::awk_main),
      install_loc: InstallLoc::DIR_USR_BIN,
      usage: std::include_str!("../usage/awk"),
    });
    #[cfg(feature = "base64")]
    appy_mcappface.push(applet {
      name: "base64",
      main: "base64",
      entrypoint: Entrypoint::CStyle(crate::coreutils::uudecode::base64_main),
      install_loc: InstallLoc::DIR_BIN,
      usage: std::include_str!("../usage/base64"),
    });
    #[cfg(feature = "basename")]
    appy_mcappface.push(applet {
      name: "basename",
      main: "basename",
      entrypoint: Entrypoint::CStyle(crate::coreutils::basename::basename_main),
      install_loc: InstallLoc::DIR_USR_BIN,
      usage: std::include_str!("../usage/basename"),
    });
    #[cfg(feature = "bc")]
    appy_mcappface.push(applet {
      name: "bc",
      main: "bc",
      entrypoint: Entrypoint::CStyle(crate::miscutils::bc::bc_main),
      install_loc: InstallLoc::DIR_USR_BIN,
      usage: std::include_str!("../usage/bc"),
    });
    #[cfg(feature = "beep")]
    appy_mcappface.push(applet {
      name: "beep",
      main: "beep",
      entrypoint: Entrypoint::CStyle(crate::miscutils::beep::beep_main),
      install_loc: InstallLoc::DIR_USR_BIN,
      usage: std::include_str!("../usage/beep"),
    });
    #[cfg(feature = "blkdiscard")]
    appy_mcappface.push(applet {
      name: "blkdiscard",
      main: "blkdiscard",
      entrypoint: Entrypoint::CStyle(crate::util_linux::blkdiscard::blkdiscard_main),
      install_loc: InstallLoc::DIR_USR_BIN,
      usage: std::include_str!("../usage/blkdiscard"),
    });
    #[cfg(feature = "blkid")]
    appy_mcappface.push(applet {
      name: "blkid",
      main: "blkid",
      entrypoint: Entrypoint::CStyle(blkcrate::coreutils::id::id_main),
      install_loc: InstallLoc::DIR_SBIN,
      usage: std::include_str!("../usage/blkid"),
    });
    #[cfg(feature = "blockdev")]
    appy_mcappface.push(applet {
      name: "blockdev",
      main: "blockdev",
      entrypoint: Entrypoint::CStyle(crate::util_linux::blockdev::blockdev_main),
      install_loc: InstallLoc::DIR_SBIN,
      usage: std::include_str!("../usage/blockdev"),
    });
    #[cfg(feature = "bootchartd")]
    appy_mcappface.push(applet {
      name: "bootchartd",
      main: "bootchartd",
      entrypoint: Entrypoint::CStyle(crate::init::bootchartd::bootchartd_main),
      install_loc: InstallLoc::DIR_SBIN,
      usage: std::include_str!("../usage/bootchartd"),
    });
    #[cfg(feature = "brctl")]
    appy_mcappface.push(applet {
      name: "brctl",
      main: "brctl",
      entrypoint: Entrypoint::CStyle(crate::networking::brctl::brctl_main),
      install_loc: InstallLoc::DIR_USR_SBIN,
      usage: std::include_str!("../usage/brctl"),
    });
    #[cfg(feature = "bunzip2")]
    appy_mcappface.push(applet {
      name: "bunzip2",
      main: "bunzip2",
      entrypoint: Entrypoint::CStyle(crate::archival::bbunzip::bunzip2_main),
      install_loc: InstallLoc::DIR_USR_BIN,
      usage: std::include_str!("../usage/bunzip2"),
    });
    #[cfg(feature = "bzcat")]
    appy_mcappface.push(applet {
      name: "bzcat",
      main: "bunzip2",
      entrypoint: Entrypoint::CStyle(crate::archival::bbunzip::bunzip2_main),
      install_loc: InstallLoc::DIR_USR_BIN,
      usage: std::include_str!("../usage/bzcat"),
    });
    #[cfg(feature = "bzip2")]
    appy_mcappface.push(applet {
      name: "bzip2",
      main: "bzip2",
      entrypoint: Entrypoint::CStyle(crate::archival::bzip2::bzip2_main),
      install_loc: InstallLoc::DIR_USR_BIN,
      usage: std::include_str!("../usage/bzip2"),
    });
    #[cfg(feature = "cal")]
    appy_mcappface.push(applet {
      name: "cal",
      main: "cal",
      entrypoint: Entrypoint::CStyle(crate::util_linux::cal::cal_main),
      install_loc: InstallLoc::DIR_USR_BIN,
      usage: std::include_str!("../usage/cal"),
    });
    #[cfg(feature = "cat")]
    appy_mcappface.push(applet {
      name: "cat",
      main: "cat",
      entrypoint: Entrypoint::CStyle(crate::coreutils::cat::cat_main),
      install_loc: InstallLoc::DIR_BIN,
      usage: std::include_str!("../usage/cat"),
    });
    #[cfg(feature = "chat")]
    appy_mcappface.push(applet {
      name: "chat",
      main: "chat",
      entrypoint: Entrypoint::CStyle(crate::miscutils::chat::chat_main),
      install_loc: InstallLoc::DIR_USR_SBIN,
      usage: std::include_str!("../usage/chat"),
    });
    #[cfg(feature = "chattr")]
    appy_mcappface.push(applet {
      name: "chattr",
      main: "chattr",
      entrypoint: Entrypoint::CStyle(chatcrate::coreutils::tr::tr_main),
      install_loc: InstallLoc::DIR_BIN,
      usage: std::include_str!("../usage/chattr"),
    });
    #[cfg(feature = "chgrp")]
    appy_mcappface.push(applet {
      name: "chgrp",
      main: "chgrp",
      entrypoint: Entrypoint::CStyle(crate::coreutils::chgrp::chgrp_main),
      install_loc: InstallLoc::DIR_BIN,
      usage: std::include_str!("../usage/chgrp"),
    });
    #[cfg(feature = "chmod")]
    appy_mcappface.push(applet {
      name: "chmod",
      main: "chmod",
      entrypoint: Entrypoint::CStyle(crate::coreutils::chmod::chmcrate::coreutils::od::od_main),
      install_loc: InstallLoc::DIR_BIN,
      usage: std::include_str!("../usage/chmod"),
    });
    #[cfg(feature = "chown")]
    appy_mcappface.push(applet {
      name: "chown",
      main: "chown",
      entrypoint: Entrypoint::CStyle(crate::coreutils::chown::chown_main),
      install_loc: InstallLoc::DIR_BIN,
      usage: std::include_str!("../usage/chown"),
    });
    #[cfg(feature = "chpasswd")]
    appy_mcappface.push(applet {
      name: "chpasswd",
      main: "chpasswd",
      entrypoint: Entrypoint::CStyle(crate::loginutils::chpasswd::chcrate::loginutils::passwd::passwd_main),
      install_loc: InstallLoc::DIR_USR_SBIN,
      usage: std::include_str!("../usage/chpasswd"),
    });
    #[cfg(feature = "chpst")]
    appy_mcappface.push(applet {
      name: "chpst",
      main: "chpst",
      entrypoint: Entrypoint::CStyle(crate::runit::chpst::chpst_main),
      install_loc: InstallLoc::DIR_USR_BIN,
      usage: std::include_str!("../usage/chpst"),
    });
    #[cfg(feature = "chroot")]
    appy_mcappface.push(applet {
      name: "chroot",
      main: "chroot",
      entrypoint: Entrypoint::CStyle(crate::coreutils::chroot::chroot_main),
      install_loc: InstallLoc::DIR_USR_SBIN,
      usage: std::include_str!("../usage/chroot"),
    });
    #[cfg(feature = "chrt")]
    appy_mcappface.push(applet {
      name: "chrt",
      main: "chrt",
      entrypoint: Entrypoint::CStyle(crate::util_linux::chrt::chrt_main),
      install_loc: InstallLoc::DIR_USR_BIN,
      usage: std::include_str!("../usage/chrt"),
    });
    #[cfg(feature = "chvt")]
    appy_mcappface.push(applet {
      name: "chvt",
      main: "chvt",
      entrypoint: Entrypoint::CStyle(crate::console_tools::chvt::chvt_main),
      install_loc: InstallLoc::DIR_USR_BIN,
      usage: std::include_str!("../usage/chvt"),
    });
    #[cfg(feature = "cksum")]
    appy_mcappface.push(applet {
      name: "cksum",
      main: "cksum",
      entrypoint: Entrypoint::CStyle(crate::coreutils::cksum::ckcrate::coreutils::sum::sum_main),
      install_loc: InstallLoc::DIR_USR_BIN,
      usage: std::include_str!("../usage/cksum"),
    });
    #[cfg(feature = "clear")]
    appy_mcappface.push(applet {
      name: "clear",
      main: "clear",
      entrypoint: Entrypoint::CStyle(crate::console_tools::clear::clear_main),
      install_loc: InstallLoc::DIR_USR_BIN,
      usage: std::include_str!("../usage/clear"),
    });
    #[cfg(feature = "cmp")]
    appy_mcappface.push(applet {
      name: "cmp",
      main: "cmp",
      entrypoint: Entrypoint::CStyle(crate::editors::cmp::cmp_main),
      install_loc: InstallLoc::DIR_USR_BIN,
      usage: std::include_str!("../usage/cmp"),
    });
    #[cfg(feature = "comm")]
    appy_mcappface.push(applet {
      name: "comm",
      main: "comm",
      entrypoint: Entrypoint::CStyle(crate::coreutils::comm::comm_main),
      install_loc: InstallLoc::DIR_USR_BIN,
      usage: std::include_str!("../usage/comm"),
    });
    #[cfg(feature = "conspy")]
    appy_mcappface.push(applet {
      name: "conspy",
      main: "conspy",
      entrypoint: Entrypoint::CStyle(crate::miscutils::conspy::conspy_main),
      install_loc: InstallLoc::DIR_BIN,
      usage: std::include_str!("../usage/conspy"),
    });
    #[cfg(feature = "cp")]
    appy_mcappface.push(applet {
      name: "cp",
      main: "cp",
      entrypoint: Entrypoint::CStyle(crate::coreutils::cp::cp_main),
      install_loc: InstallLoc::DIR_BIN,
      usage: std::include_str!("../usage/cp"),
    });
    #[cfg(feature = "cpio")]
    appy_mcappface.push(applet {
      name: "cpio",
      main: "cpio",
      entrypoint: Entrypoint::CStyle(crate::archival::cpio::cpio_main),
      install_loc: InstallLoc::DIR_BIN,
      usage: std::include_str!("../usage/cpio"),
    });
    #[cfg(feature = "crond")]
    appy_mcappface.push(applet {
      name: "crond",
      main: "crond",
      entrypoint: Entrypoint::CStyle(crate::miscutils::crond::crond_main),
      install_loc: InstallLoc::DIR_USR_SBIN,
      usage: std::include_str!("../usage/crond"),
    });
    #[cfg(feature = "crontab")]
    appy_mcappface.push(applet {
      name: "crontab",
      main: "crontab",
      entrypoint: Entrypoint::CStyle(crate::miscutils::crontab::crontab_main),
      install_loc: InstallLoc::DIR_USR_BIN,
      usage: std::include_str!("../usage/crontab"),
    });
    #[cfg(feature = "cryptpw")]
    appy_mcappface.push(applet {
      name: "cryptpw",
      main: "cryptpw",
      entrypoint: Entrypoint::CStyle(crate::loginutils::cryptpw::cryptpw_main),
      install_loc: InstallLoc::DIR_USR_BIN,
      usage: std::include_str!("../usage/cryptpw"),
    });
    #[cfg(feature = "cttyhack")]
    appy_mcappface.push(applet {
      name: "cttyhack",
      main: "cttyhack",
      entrypoint: Entrypoint::CStyle(crate::shell::cttyhack::cttyhack_main),
      install_loc: InstallLoc::DIR_BIN,
      usage: std::include_str!("../usage/cttyhack"),
    });
    #[cfg(feature = "cut")]
    appy_mcappface.push(applet {
      name: "cut",
      main: "cut",
      entrypoint: Entrypoint::CStyle(crate::coreutils::cut::cut_main),
      install_loc: InstallLoc::DIR_USR_BIN,
      usage: std::include_str!("../usage/cut"),
    });
    #[cfg(feature = "date")]
    appy_mcappface.push(applet {
      name: "date",
      main: "date",
      entrypoint: Entrypoint::CStyle(crate::coreutils::date::date_main),
      install_loc: InstallLoc::DIR_BIN,
      usage: std::include_str!("../usage/date"),
    });
    #[cfg(feature = "dc")]
    appy_mcappface.push(applet {
      name: "dc",
      main: "dc",
      entrypoint: Entrypoint::CStyle(crate::miscutils::bc::dc_main),
      install_loc: InstallLoc::DIR_USR_BIN,
      usage: std::include_str!("../usage/dc"),
    });
    #[cfg(feature = "dd")]
    appy_mcappface.push(applet {
      name: "dd",
      main: "dd",
      entrypoint: Entrypoint::CStyle(crate::coreutils::dd::dd_main),
      install_loc: InstallLoc::DIR_BIN,
      usage: std::include_str!("../usage/dd"),
    });
    #[cfg(feature = "deallocvt")]
    appy_mcappface.push(applet {
      name: "deallocvt",
      main: "deallocvt",
      entrypoint: Entrypoint::CStyle(crate::console_tools::deallocvt::deallocvt_main),
      install_loc: InstallLoc::DIR_USR_BIN,
      usage: std::include_str!("../usage/deallocvt"),
    });
    #[cfg(feature = "delgroup")]
    appy_mcappface.push(applet {
      name: "delgroup",
      main: "deluser",
      entrypoint: Entrypoint::CStyle(crate::loginutils::deluser::deluser_main),
      install_loc: InstallLoc::DIR_USR_SBIN,
      usage: std::include_str!("../usage/delgroup"),
    });
    #[cfg(feature = "deluser")]
    appy_mcappface.push(applet {
      name: "deluser",
      main: "deluser",
      entrypoint: Entrypoint::CStyle(crate::loginutils::deluser::deluser_main),
      install_loc: InstallLoc::DIR_USR_SBIN,
      usage: std::include_str!("../usage/deluser"),
    });
    #[cfg(feature = "depmod")]
    appy_mcappface.push(applet {
      name: "depmod",
      main: "modprobe",
      entrypoint: Entrypoint::CStyle(crate::modutils::modprobe_small::modprobe_main),
      install_loc: InstallLoc::DIR_SBIN,
      usage: std::include_str!("../usage/depmod"),
    });
    #[cfg(feature = "devmem")]
    appy_mcappface.push(applet {
      name: "devmem",
      main: "devmem",
      entrypoint: Entrypoint::CStyle(crate::miscutils::devmem::devmem_main),
      install_loc: InstallLoc::DIR_SBIN,
      usage: std::include_str!("../usage/devmem"),
    });
    #[cfg(feature = "df")]
    appy_mcappface.push(applet {
      name: "df",
      main: "df",
      entrypoint: Entrypoint::CStyle(crate::coreutils::df::df_main),
      install_loc: InstallLoc::DIR_BIN,
      usage: std::include_str!("../usage/df"),
    });
    #[cfg(feature = "dhcprelay")]
    appy_mcappface.push(applet {
      name: "dhcprelay",
      main: "dhcprelay",
      entrypoint: Entrypoint::CStyle(crate::networking::udhcp::dhcprelay::dhcprelay_main),
      install_loc: InstallLoc::DIR_USR_SBIN,
      usage: std::include_str!("../usage/dhcprelay"),
    });
    #[cfg(feature = "diff")]
    appy_mcappface.push(applet {
      name: "diff",
      main: "diff",
      entrypoint: Entrypoint::CStyle(crate::editors::diff::diff_main),
      install_loc: InstallLoc::DIR_USR_BIN,
      usage: std::include_str!("../usage/diff"),
    });
    #[cfg(feature = "dirname")]
    appy_mcappface.push(applet {
      name: "dirname",
      main: "dirname",
      entrypoint: Entrypoint::CStyle(crate::coreutils::dirname::dirname_main),
      install_loc: InstallLoc::DIR_USR_BIN,
      usage: std::include_str!("../usage/dirname"),
    });
    #[cfg(feature = "dmesg")]
    appy_mcappface.push(applet {
      name: "dmesg",
      main: "dmesg",
      entrypoint: Entrypoint::CStyle(crate::util_linux::dmesg::dcrate::util_linux::mesg::mesg_main),
      install_loc: InstallLoc::DIR_BIN,
      usage: std::include_str!("../usage/dmesg"),
    });
    #[cfg(feature = "dnsd")]
    appy_mcappface.push(applet {
      name: "dnsd",
      main: "dnsd",
      entrypoint: Entrypoint::CStyle(crate::networking::dnsd::dnsd_main),
      install_loc: InstallLoc::DIR_USR_SBIN,
      usage: std::include_str!("../usage/dnsd"),
    });
    #[cfg(feature = "dnsdomainname")]
    appy_mcappface.push(applet {
      name: "dnsdomainname",
      main: "hostname",
      entrypoint: Entrypoint::CStyle(crate::networking::hostname::hostname_main),
      install_loc: InstallLoc::DIR_BIN,
      usage: std::include_str!("../usage/dnsdomainname"),
    });
    #[cfg(feature = "dos2unix")]
    appy_mcappface.push(applet {
      name: "dos2unix",
      main: "dos2unix",
      entrypoint: Entrypoint::CStyle(crate::coreutils::dos2unix::dos2unix_main),
      install_loc: InstallLoc::DIR_USR_BIN,
      usage: std::include_str!("../usage/dos2unix"),
    });
    #[cfg(feature = "dpkg")]
    appy_mcappface.push(applet {
      name: "dpkg",
      main: "dpkg",
      entrypoint: Entrypoint::CStyle(crate::archival::dpkg_deb::dpkg_deb_main),
      install_loc: InstallLoc::DIR_USR_BIN,
      usage: std::include_str!("../usage/dpkg"),
    });
    #[cfg(feature = "dpkg-deb")]
    appy_mcappface.push(applet {
      name: "dpkg-deb",
      main: "dpkg_deb",
      entrypoint: Entrypoint::CStyle(crate::archival::dpkg::dpkg_main),
      install_loc: InstallLoc::DIR_USR_BIN,
      usage: std::include_str!("../usage/dpkg-deb"),
    });
    #[cfg(feature = "du")]
    appy_mcappface.push(applet {
      name: "du",
      main: "du",
      entrypoint: Entrypoint::CStyle(crate::coreutils::du::du_main),
      install_loc: InstallLoc::DIR_USR_BIN,
      usage: std::include_str!("../usage/du"),
    });
    #[cfg(feature = "dumpkmap")]
    appy_mcappface.push(applet {
      name: "dumpkmap",
      main: "dumpkmap",
      entrypoint: Entrypoint::CStyle(crate::console_tools::dumpkmap::dumpkmap_main),
      install_loc: InstallLoc::DIR_BIN,
      usage: std::include_str!("../usage/dumpkmap"),
    });
    #[cfg(feature = "dumpleases")]
    appy_mcappface.push(applet {
      name: "dumpleases",
      main: "dumpleases",
      entrypoint: Entrypoint::CStyle(crate::networking::udhcp::dumpleases::dumpleases_main),
      install_loc: InstallLoc::DIR_USR_BIN,
      usage: std::include_str!("../usage/dumpleases"),
    });
    #[cfg(feature = "echo")]
    appy_mcappface.push(applet {
      name: "echo",
      main: "echo",
      entrypoint: Entrypoint::CStyle(crate::coreutils::echo::echo_main),
      install_loc: InstallLoc::DIR_BIN,
      usage: std::include_str!("../usage/echo"),
    });
    #[cfg(feature = "ed")]
    appy_mcappface.push(applet {
      name: "ed",
      main: "ed",
      entrypoint: Entrypoint::CStyle(crate::editors::ed::ed_main),
      install_loc: InstallLoc::DIR_BIN,
      usage: std::include_str!("../usage/ed"),
    });
    #[cfg(feature = "egrep")]
    appy_mcappface.push(applet {
      name: "egrep",
      main: "grep",
      entrypoint: Entrypoint::CStyle(crate::findutils::grep::grep_main),
      install_loc: InstallLoc::DIR_BIN,
      usage: std::include_str!("../usage/egrep"),
    });
    #[cfg(feature = "eject")]
    appy_mcappface.push(applet {
      name: "eject",
      main: "eject",
      entrypoint: Entrypoint::CStyle(crate::util_linux::eject::eject_main),
      install_loc: InstallLoc::DIR_USR_BIN,
      usage: std::include_str!("../usage/eject"),
    });
    #[cfg(feature = "env")]
    appy_mcappface.push(applet {
      name: "env",
      main: "env",
      entrypoint: Entrypoint::CStyle(crate::coreutils::env::env_main),
      install_loc: InstallLoc::DIR_USR_BIN,
      usage: std::include_str!("../usage/env"),
    });
    #[cfg(feature = "envdir")]
    appy_mcappface.push(applet {
      name: "envdir",
      main: "chpst",
      entrypoint: Entrypoint::CStyle(crate::runit::chpst::chpst_main),
      install_loc: InstallLoc::DIR_USR_BIN,
      usage: std::include_str!("../usage/envdir"),
    });
    #[cfg(feature = "envuidgid")]
    appy_mcappface.push(applet {
      name: "envuidgid",
      main: "chpst",
      entrypoint: Entrypoint::CStyle(crate::runit::chpst::chpst_main),
      install_loc: InstallLoc::DIR_USR_BIN,
      usage: std::include_str!("../usage/envuidgid"),
    });
    #[cfg(feature = "ether-wake")]
    appy_mcappface.push(applet {
      name: "ether-wake",
      main: "ether_wake",
      entrypoint: Entrypoint::CStyle(crate::networking::ether_wake::ether_wake_main),
      install_loc: InstallLoc::DIR_USR_SBIN,
      usage: std::include_str!("../usage/ether-wake"),
    });
    #[cfg(feature = "expand")]
    appy_mcappface.push(applet {
      name: "expand",
      main: "expand",
      entrypoint: Entrypoint::CStyle(crate::coreutils::expand::expand_main),
      install_loc: InstallLoc::DIR_USR_BIN,
      usage: std::include_str!("../usage/expand"),
    });
    #[cfg(feature = "expr")]
    appy_mcappface.push(applet {
      name: "expr",
      main: "expr",
      entrypoint: Entrypoint::CStyle(crate::coreutils::expr::expr_main),
      install_loc: InstallLoc::DIR_USR_BIN,
      usage: std::include_str!("../usage/expr"),
    });
    #[cfg(feature = "factor")]
    appy_mcappface.push(applet {
      name: "factor",
      main: "factor",
      entrypoint: Entrypoint::CStyle(crate::coreutils::factor::factor_main),
      install_loc: InstallLoc::DIR_USR_BIN,
      usage: std::include_str!("../usage/factor"),
    });
    #[cfg(feature = "fakeidentd")]
    appy_mcappface.push(applet {
      name: "fakeidentd",
      main: "fakeidentd",
      entrypoint: Entrypoint::CStyle(crate::networking::isrv_identd::fakeidentd_main),
      install_loc: InstallLoc::DIR_USR_SBIN,
      usage: std::include_str!("../usage/fakeidentd"),
    });
    #[cfg(feature = "fallocate")]
    appy_mcappface.push(applet {
      name: "fallocate",
      main: "fallocate",
      entrypoint: Entrypoint::CStyle(crate::util_linux::fallocate::fallocate_main),
      install_loc: InstallLoc::DIR_USR_BIN,
      usage: std::include_str!("../usage/fallocate"),
    });
    #[cfg(feature = "false")]
    appy_mcappface.push(applet {
      name: "false",
      main: "false",
      entrypoint: Entrypoint::SafeStyle(crate::coreutils::r#false::false_main),
      install_loc: InstallLoc::DIR_BIN,
      usage: std::include_str!("../usage/false"),
    });
    #[cfg(feature = "fatattr")]
    appy_mcappface.push(applet {
      name: "fatattr",
      main: "fatattr",
      entrypoint: Entrypoint::CStyle(fatatcrate::coreutils::tr::tr_main),
      install_loc: InstallLoc::DIR_BIN,
      usage: std::include_str!("../usage/fatattr"),
    });
    #[cfg(feature = "fbset")]
    appy_mcappface.push(applet {
      name: "fbset",
      main: "fbset",
      entrypoint: Entrypoint::CStyle(crate::util_linux::fbset::fbset_main),
      install_loc: InstallLoc::DIR_USR_SBIN,
      usage: std::include_str!("../usage/fbset"),
    });
    #[cfg(feature = "fbsplash")]
    appy_mcappface.push(applet {
      name: "fbsplash",
      main: "fbsplash",
      entrypoint: Entrypoint::CStyle(crate::miscutils::fbsplash::fbsplcrate::shell::ash::ash_main),
      install_loc: InstallLoc::DIR_SBIN,
      usage: std::include_str!("../usage/fbsplash"),
    });
    #[cfg(feature = "fdflush")]
    appy_mcappface.push(applet {
      name: "fdflush",
      main: "freeramdisk",
      entrypoint: Entrypoint::CStyle(crate::util_linux::freeramdisk::freeramdisk_main),
      install_loc: InstallLoc::DIR_BIN,
      usage: std::include_str!("../usage/fdflush"),
    });
    #[cfg(feature = "fdformat")]
    appy_mcappface.push(applet {
      name: "fdformat",
      main: "fdformat",
      entrypoint: Entrypoint::CStyle(crate::util_linux::fdformat::fdformat_main),
      install_loc: InstallLoc::DIR_USR_SBIN,
      usage: std::include_str!("../usage/fdformat"),
    });
    #[cfg(feature = "fdisk")]
    appy_mcappface.push(applet {
      name: "fdisk",
      main: "fdisk",
      entrypoint: Entrypoint::CStyle(crate::util_linux::fdisk::fdisk_main),
      install_loc: InstallLoc::DIR_SBIN,
      usage: std::include_str!("../usage/fdisk"),
    });
    #[cfg(feature = "fgconsole")]
    appy_mcappface.push(applet {
      name: "fgconsole",
      main: "fgconsole",
      entrypoint: Entrypoint::CStyle(crate::console_tools::fgconsole::fgconsole_main),
      install_loc: InstallLoc::DIR_USR_BIN,
      usage: std::include_str!("../usage/fgconsole"),
    });
    #[cfg(feature = "fgrep")]
    appy_mcappface.push(applet {
      name: "fgrep",
      main: "grep",
      entrypoint: Entrypoint::CStyle(crate::findutils::grep::grep_main),
      install_loc: InstallLoc::DIR_BIN,
      usage: std::include_str!("../usage/fgrep"),
    });
    #[cfg(feature = "find")]
    appy_mcappface.push(applet {
      name: "find",
      main: "find",
      entrypoint: Entrypoint::CStyle(crate::findutils::find::find_main),
      install_loc: InstallLoc::DIR_USR_BIN,
      usage: std::include_str!("../usage/find"),
    });
    #[cfg(feature = "findfs")]
    appy_mcappface.push(applet {
      name: "findfs",
      main: "findfs",
      entrypoint: Entrypoint::CStyle(crate::util_linux::findfs::findfs_main),
      install_loc: InstallLoc::DIR_SBIN,
      usage: std::include_str!("../usage/findfs"),
    });
    #[cfg(feature = "flock")]
    appy_mcappface.push(applet {
      name: "flock",
      main: "flock",
      entrypoint: Entrypoint::CStyle(crate::util_linux::flock::flock_main),
      install_loc: InstallLoc::DIR_USR_BIN,
      usage: std::include_str!("../usage/flock"),
    });
    #[cfg(feature = "fold")]
    appy_mcappface.push(applet {
      name: "fold",
      main: "fold",
      entrypoint: Entrypoint::CStyle(crate::coreutils::fold::fold_main),
      install_loc: InstallLoc::DIR_USR_BIN,
      usage: std::include_str!("../usage/fold"),
    });
    #[cfg(feature = "free")]
    appy_mcappface.push(applet {
      name: "free",
      main: "free",
      entrypoint: Entrypoint::CStyle(crate::procps::free::free_main),
      install_loc: InstallLoc::DIR_USR_BIN,
      usage: std::include_str!("../usage/free"),
    });
    #[cfg(feature = "freeramdisk")]
    appy_mcappface.push(applet {
      name: "freeramdisk",
      main: "freeramdisk",
      entrypoint: Entrypoint::CStyle(crate::util_linux::freeramdisk::freeramdisk_main),
      install_loc: InstallLoc::DIR_SBIN,
      usage: std::include_str!("../usage/freeramdisk"),
    });
    #[cfg(feature = "fsck")]
    appy_mcappface.push(applet {
      name: "fsck",
      main: "fsck",
      entrypoint: Entrypoint::CStyle(crate::util_linux::fsck_minix::fsck_minix_main),
      install_loc: InstallLoc::DIR_SBIN,
      usage: std::include_str!("../usage/fsck"),
    });
    #[cfg(feature = "fsck_minix")]
    appy_mcappface.push(applet {
      name: "fsck.minix",
      main: "fsck_minix",
      entrypoint: Entrypoint::CStyle(crate::e2fsprogs::fsck::fsck_main),
      install_loc: InstallLoc::DIR_SBIN,
      usage: std::include_str!("../usage/fsck.minix"),
    });
    #[cfg(feature = "fsfreeze")]
    appy_mcappface.push(applet {
      name: "fsfreeze",
      main: "fsfreeze",
      entrypoint: Entrypoint::CStyle(crate::util_linux::fsfreeze::fsfreeze_main),
      install_loc: InstallLoc::DIR_USR_SBIN,
      usage: std::include_str!("../usage/fsfreeze"),
    });
    #[cfg(feature = "fstrim")]
    appy_mcappface.push(applet {
      name: "fstrim",
      main: "fstrim",
      entrypoint: Entrypoint::CStyle(crate::util_linux::fstrim::fstrim_main),
      install_loc: InstallLoc::DIR_SBIN,
      usage: std::include_str!("../usage/fstrim"),
    });
    #[cfg(feature = "fsync")]
    appy_mcappface.push(applet {
      name: "fsync",
      main: "fsync",
      entrypoint: Entrypoint::CStyle(crate::coreutils::sync::fcrate::coreutils::sync::sycrate::networking::nc::nc_main),
      install_loc: InstallLoc::DIR_BIN,
      usage: std::include_str!("../usage/fsync"),
    });
    #[cfg(feature = "ftpd")]
    appy_mcappface.push(applet {
      name: "ftpd",
      main: "ftpd",
      entrypoint: Entrypoint::CStyle(crate::networking::ftpd::ftpd_main),
      install_loc: InstallLoc::DIR_USR_SBIN,
      usage: std::include_str!("../usage/ftpd"),
    });
    #[cfg(feature = "ftpget")]
    appy_mcappface.push(applet {
      name: "ftpget",
      main: "ftpgetput",
      entrypoint: Entrypoint::CStyle(crate::networking::ftpgetput::ftpgetput_main),
      install_loc: InstallLoc::DIR_USR_BIN,
      usage: std::include_str!("../usage/ftpget"),
    });
    #[cfg(feature = "ftpput")]
    appy_mcappface.push(applet {
      name: "ftpput",
      main: "ftpgetput",
      entrypoint: Entrypoint::CStyle(crate::networking::ftpgetput::ftpgetput_main),
      install_loc: InstallLoc::DIR_USR_BIN,
      usage: std::include_str!("../usage/ftpput"),
    });
    #[cfg(feature = "fuser")]
    appy_mcappface.push(applet {
      name: "fuser",
      main: "fuser",
      entrypoint: Entrypoint::CStyle(crate::procps::fuser::fuser_main),
      install_loc: InstallLoc::DIR_USR_BIN,
      usage: std::include_str!("../usage/fuser"),
    });
    #[cfg(feature = "getopt")]
    appy_mcappface.push(applet {
      name: "getopt",
      main: "getopt",
      entrypoint: Entrypoint::CStyle(crate::util_linux::getopt::getopt_main),
      install_loc: InstallLoc::DIR_BIN,
      usage: std::include_str!("../usage/getopt"),
    });
    #[cfg(feature = "getty")]
    appy_mcappface.push(applet {
      name: "getty",
      main: "getty",
      entrypoint: Entrypoint::CStyle(gecrate::coreutils::tty::tty_main),
      install_loc: InstallLoc::DIR_SBIN,
      usage: std::include_str!("../usage/getty"),
    });
    #[cfg(feature = "grep")]
    appy_mcappface.push(applet {
      name: "grep",
      main: "grep",
      entrypoint: Entrypoint::CStyle(crate::findutils::grep::grep_main),
      install_loc: InstallLoc::DIR_BIN,
      usage: std::include_str!("../usage/grep"),
    });
    #[cfg(feature = "groups")]
    appy_mcappface.push(applet {
      name: "groups",
      main: "id",
      entrypoint: Entrypoint::CStyle(crate::coreutils::id::id_main),
      install_loc: InstallLoc::DIR_USR_BIN,
      usage: std::include_str!("../usage/groups"),
    });
    #[cfg(feature = "gunzip")]
    appy_mcappface.push(applet {
      name: "gunzip",
      main: "gunzip",
      entrypoint: Entrypoint::CStyle(crate::archival::bbunzip::gcrate::archival::unzip::unzcrate::networking::ip::ip_main),
      install_loc: InstallLoc::DIR_BIN,
      usage: std::include_str!("../usage/gunzip"),
    });
    #[cfg(feature = "gzip")]
    appy_mcappface.push(applet {
      name: "gzip",
      main: "gzip",
      entrypoint: Entrypoint::CStyle(crate::archival::gzip::gzcrate::networking::ip::ip_main),
      install_loc: InstallLoc::DIR_BIN,
      usage: std::include_str!("../usage/gzip"),
    });
    #[cfg(feature = "halt")]
    appy_mcappface.push(applet {
      name: "halt",
      main: "halt",
      entrypoint: Entrypoint::CStyle(crate::init::halt::halt_main),
      install_loc: InstallLoc::DIR_SBIN,
      usage: std::include_str!("../usage/halt"),
    });
    #[cfg(feature = "hd")]
    appy_mcappface.push(applet {
      name: "hd",
      main: "hexdump",
      entrypoint: Entrypoint::CStyle(crate::util_linux::hexdump::hexdump_main),
      install_loc: InstallLoc::DIR_USR_BIN,
      usage: std::include_str!("../usage/hd"),
    });
    #[cfg(feature = "hdparm")]
    appy_mcappface.push(applet {
      name: "hdparm",
      main: "hdparm",
      entrypoint: Entrypoint::CStyle(hdpacrate::coreutils::rm::rm_main),
      install_loc: InstallLoc::DIR_SBIN,
      usage: std::include_str!("../usage/hdparm"),
    });
    #[cfg(feature = "head")]
    appy_mcappface.push(applet {
      name: "head",
      main: "head",
      entrypoint: Entrypoint::CStyle(crate::coreutils::head::head_main),
      install_loc: InstallLoc::DIR_USR_BIN,
      usage: std::include_str!("../usage/head"),
    });
    #[cfg(feature = "hexdump")]
    appy_mcappface.push(applet {
      name: "hexdump",
      main: "hexdump",
      entrypoint: Entrypoint::CStyle(crate::util_linux::hexdump::hexdump_main),
      install_loc: InstallLoc::DIR_USR_BIN,
      usage: std::include_str!("../usage/hexdump"),
    });
    #[cfg(feature = "hexedit")]
    appy_mcappface.push(applet {
      name: "hexedit",
      main: "hexedit",
      entrypoint: Entrypoint::CStyle(crate::miscutils::hexedit::hexedit_main),
      install_loc: InstallLoc::DIR_USR_BIN,
      usage: std::include_str!("../usage/hexedit"),
    });
    #[cfg(feature = "hostid")]
    appy_mcappface.push(applet {
      name: "hostid",
      main: "hostid",
      entrypoint: Entrypoint::CStyle(crate::coreutils::hostid::hostcrate::coreutils::id::id_main),
      install_loc: InstallLoc::DIR_USR_BIN,
      usage: std::include_str!("../usage/hostid"),
    });
    #[cfg(feature = "hostname")]
    appy_mcappface.push(applet {
      name: "hostname",
      main: "hostname",
      entrypoint: Entrypoint::CStyle(crate::networking::hostname::hostname_main),
      install_loc: InstallLoc::DIR_BIN,
      usage: std::include_str!("../usage/hostname"),
    });
    #[cfg(feature = "httpd")]
    appy_mcappface.push(applet {
      name: "httpd",
      main: "httpd",
      entrypoint: Entrypoint::CStyle(crate::networking::httpd::httpd_main),
      install_loc: InstallLoc::DIR_USR_SBIN,
      usage: std::include_str!("../usage/httpd"),
    });
    #[cfg(feature = "hush")]
    appy_mcappface.push(applet {
      name: "hush",
      main: "hush",
      entrypoint: Entrypoint::CStyle(crate::shell::hush::hush_main),
      install_loc: InstallLoc::DIR_BIN,
      usage: std::include_str!("../usage/hush"),
    });
    #[cfg(feature = "hwclock")]
    appy_mcappface.push(applet {
      name: "hwclock",
      main: "hwclock",
      entrypoint: Entrypoint::CStyle(crate::util_linux::hwclock::hwclock_main),
      install_loc: InstallLoc::DIR_SBIN,
      usage: std::include_str!("../usage/hwclock"),
    });
    #[cfg(feature = "i2cdetect")]
    appy_mcappface.push(applet {
      name: "i2cdetect",
      main: "i2cdetect",
      entrypoint: Entrypoint::CStyle(crate::miscutils::i2c_tools::i2cdetect_main),
      install_loc: InstallLoc::DIR_USR_SBIN,
      usage: std::include_str!("../usage/i2cdetect"),
    });
    #[cfg(feature = "i2cdump")]
    appy_mcappface.push(applet {
      name: "i2cdump",
      main: "i2cdump",
      entrypoint: Entrypoint::CStyle(crate::miscutils::i2c_tools::i2cdump_main),
      install_loc: InstallLoc::DIR_USR_SBIN,
      usage: std::include_str!("../usage/i2cdump"),
    });
    #[cfg(feature = "i2cget")]
    appy_mcappface.push(applet {
      name: "i2cget",
      main: "i2cget",
      entrypoint: Entrypoint::CStyle(crate::miscutils::i2c_tools::i2cget_main),
      install_loc: InstallLoc::DIR_USR_SBIN,
      usage: std::include_str!("../usage/i2cget"),
    });
    #[cfg(feature = "i2cset")]
    appy_mcappface.push(applet {
      name: "i2cset",
      main: "i2cset",
      entrypoint: Entrypoint::CStyle(crate::miscutils::i2c_tools::i2cset_main),
      install_loc: InstallLoc::DIR_USR_SBIN,
      usage: std::include_str!("../usage/i2cset"),
    });
    #[cfg(feature = "i2ctransfer")]
    appy_mcappface.push(applet {
      name: "i2ctransfer",
      main: "i2ctransfer",
      entrypoint: Entrypoint::CStyle(crate::miscutils::i2c_tools::i2ctransfer_main),
      install_loc: InstallLoc::DIR_USR_SBIN,
      usage: std::include_str!("../usage/i2ctransfer"),
    });
    #[cfg(feature = "id")]
    appy_mcappface.push(applet {
      name: "id",
      main: "id",
      entrypoint: Entrypoint::CStyle(crate::coreutils::id::id_main),
      install_loc: InstallLoc::DIR_USR_BIN,
      usage: std::include_str!("../usage/id"),
    });
    #[cfg(feature = "ifconfig")]
    appy_mcappface.push(applet {
      name: "ifconfig",
      main: "ifconfig",
      entrypoint: Entrypoint::CStyle(crate::networking::ifconfig::ifconfig_main),
      install_loc: InstallLoc::DIR_SBIN,
      usage: std::include_str!("../usage/ifconfig"),
    });
    #[cfg(feature = "ifdown")]
    appy_mcappface.push(applet {
      name: "ifdown",
      main: "ifupdown",
      entrypoint: Entrypoint::CStyle(crate::networking::ifupdown::ifupdown_main),
      install_loc: InstallLoc::DIR_SBIN,
      usage: std::include_str!("../usage/ifdown"),
    });
    #[cfg(feature = "ifenslave")]
    appy_mcappface.push(applet {
      name: "ifenslave",
      main: "ifenslave",
      entrypoint: Entrypoint::CStyle(crate::networking::ifenslave::ifenslave_main),
      install_loc: InstallLoc::DIR_SBIN,
      usage: std::include_str!("../usage/ifenslave"),
    });
    #[cfg(feature = "ifplugd")]
    appy_mcappface.push(applet {
      name: "ifplugd",
      main: "ifplugd",
      entrypoint: Entrypoint::CStyle(crate::networking::ifplugd::ifplugd_main),
      install_loc: InstallLoc::DIR_USR_SBIN,
      usage: std::include_str!("../usage/ifplugd"),
    });
    #[cfg(feature = "ifup")]
    appy_mcappface.push(applet {
      name: "ifup",
      main: "ifupdown",
      entrypoint: Entrypoint::CStyle(crate::networking::ifupdown::ifupdown_main),
      install_loc: InstallLoc::DIR_SBIN,
      usage: std::include_str!("../usage/ifup"),
    });
    #[cfg(feature = "inetd")]
    appy_mcappface.push(applet {
      name: "inetd",
      main: "inetd",
      entrypoint: Entrypoint::CStyle(crate::networking::inetd::inetd_main),
      install_loc: InstallLoc::DIR_USR_SBIN,
      usage: std::include_str!("../usage/inetd"),
    });
    #[cfg(feature = "init")]
    appy_mcappface.push(applet {
      name: "init",
      main: "init",
      entrypoint: Entrypoint::CStyle(crate::init::init::init_main),
      install_loc: InstallLoc::DIR_SBIN,
      usage: std::include_str!("../usage/init"),
    });
    #[cfg(feature = "insmod")]
    appy_mcappface.push(applet {
      name: "insmod",
      main: "modprobe",
      entrypoint: Entrypoint::CStyle(crate::modutils::modprobe_small::modprobe_main),
      install_loc: InstallLoc::DIR_SBIN,
      usage: std::include_str!("../usage/insmod"),
    });
    #[cfg(feature = "install")]
    appy_mcappface.push(applet {
      name: "install",
      main: "install",
      entrypoint: Entrypoint::CStyle(crate::coreutils::install::install_main),
      install_loc: InstallLoc::DIR_USR_BIN,
      usage: std::include_str!("../usage/install"),
    });
    #[cfg(feature = "ionice")]
    appy_mcappface.push(applet {
      name: "ionice",
      main: "ionice",
      entrypoint: Entrypoint::CStyle(iocrate::coreutils::nice::nice_main),
      install_loc: InstallLoc::DIR_BIN,
      usage: std::include_str!("../usage/ionice"),
    });
    #[cfg(feature = "iostat")]
    appy_mcappface.push(applet {
      name: "iostat",
      main: "iostat",
      entrypoint: Entrypoint::CStyle(iocrate::coreutils::stat::stat_main),
      install_loc: InstallLoc::DIR_BIN,
      usage: std::include_str!("../usage/iostat"),
    });
    #[cfg(feature = "ip")]
    appy_mcappface.push(applet {
      name: "ip",
      main: "ip",
      entrypoint: Entrypoint::CStyle(crate::networking::ip::ip_main),
      install_loc: InstallLoc::DIR_SBIN,
      usage: std::include_str!("../usage/ip"),
    });
    #[cfg(feature = "ipaddr")]
    appy_mcappface.push(applet {
      name: "ipaddr",
      main: "ipaddr",
      entrypoint: Entrypoint::CStyle(crate::networking::ip::ipaddr_main),
      install_loc: InstallLoc::DIR_SBIN,
      usage: std::include_str!("../usage/ipaddr"),
    });
    #[cfg(feature = "ipcalc")]
    appy_mcappface.push(applet {
      name: "ipcalc",
      main: "ipcalc",
      entrypoint: Entrypoint::CStyle(crate::networking::ipcalc::ipcalc_main),
      install_loc: InstallLoc::DIR_BIN,
      usage: std::include_str!("../usage/ipcalc"),
    });
    #[cfg(feature = "ipcrm")]
    appy_mcappface.push(applet {
      name: "ipcrm",
      main: "ipcrm",
      entrypoint: Entrypoint::CStyle(ipccrate::coreutils::rm::rm_main),
      install_loc: InstallLoc::DIR_USR_BIN,
      usage: std::include_str!("../usage/ipcrm"),
    });
    #[cfg(feature = "ipcs")]
    appy_mcappface.push(applet {
      name: "ipcs",
      main: "ipcs",
      entrypoint: Entrypoint::CStyle(crate::util_linux::ipcs::ipcs_main),
      install_loc: InstallLoc::DIR_USR_BIN,
      usage: std::include_str!("../usage/ipcs"),
    });
    #[cfg(feature = "iplink")]
    appy_mcappface.push(applet {
      name: "iplink",
      main: "iplink",
      entrypoint: Entrypoint::CStyle(ipcrate::coreutils::link::link_main),
      install_loc: InstallLoc::DIR_SBIN,
      usage: std::include_str!("../usage/iplink"),
    });
    #[cfg(feature = "ipneigh")]
    appy_mcappface.push(applet {
      name: "ipneigh",
      main: "ipneigh",
      entrypoint: Entrypoint::CStyle(crate::networking::ip::ipneigh_main),
      install_loc: InstallLoc::DIR_SBIN,
      usage: std::include_str!("../usage/ipneigh"),
    });
    #[cfg(feature = "iproute")]
    appy_mcappface.push(applet {
      name: "iproute",
      main: "iproute",
      entrypoint: Entrypoint::CStyle(crate::networking::ip::ipcrate::networking::route::route_main),
      install_loc: InstallLoc::DIR_SBIN,
      usage: std::include_str!("../usage/iproute"),
    });
    #[cfg(feature = "iprule")]
    appy_mcappface.push(applet {
      name: "iprule",
      main: "iprule",
      entrypoint: Entrypoint::CStyle(crate::networking::ip::iprule_main),
      install_loc: InstallLoc::DIR_SBIN,
      usage: std::include_str!("../usage/iprule"),
    });
    #[cfg(feature = "iptunnel")]
    appy_mcappface.push(applet {
      name: "iptunnel",
      main: "iptunnel",
      entrypoint: Entrypoint::CStyle(crate::networking::ip::iptunnel_main),
      install_loc: InstallLoc::DIR_SBIN,
      usage: std::include_str!("../usage/iptunnel"),
    });
    #[cfg(feature = "kbd_mode")]
    appy_mcappface.push(applet {
      name: "kbd_mode",
      main: "kbd_mode",
      entrypoint: Entrypoint::CStyle(crate::console_tools::kbd_mode::kbd_mode_main),
      install_loc: InstallLoc::DIR_BIN,
      usage: std::include_str!("../usage/kbd_mode"),
    });
    #[cfg(feature = "kill")]
    appy_mcappface.push(applet {
      name: "kill",
      main: "kill",
      entrypoint: Entrypoint::CStyle(crate::procps::kill::kill_main),
      install_loc: InstallLoc::DIR_BIN,
      usage: std::include_str!("../usage/kill"),
    });
    #[cfg(feature = "killall")]
    appy_mcappface.push(applet {
      name: "killall",
      main: "kill",
      entrypoint: Entrypoint::CStyle(crate::procps::kill::kill_main),
      install_loc: InstallLoc::DIR_USR_BIN,
      usage: std::include_str!("../usage/killall"),
    });
    #[cfg(feature = "killall5")]
    appy_mcappface.push(applet {
      name: "killall5",
      main: "kill",
      entrypoint: Entrypoint::CStyle(crate::procps::kill::kill_main),
      install_loc: InstallLoc::DIR_USR_SBIN,
      usage: std::include_str!("../usage/killall5"),
    });
    #[cfg(feature = "klogd")]
    appy_mcappface.push(applet {
      name: "klogd",
      main: "klogd",
      entrypoint: Entrypoint::CStyle(crate::sysklogd::klogd::klogd_main),
      install_loc: InstallLoc::DIR_SBIN,
      usage: std::include_str!("../usage/klogd"),
    });
    #[cfg(feature = "last")]
    appy_mcappface.push(applet {
      name: "last",
      main: "last",
      entrypoint: Entrypoint::CStyle(crate::util_linux::last_fancy::last_main),
      install_loc: InstallLoc::DIR_USR_BIN,
      usage: std::include_str!("../usage/last"),
    });
    #[cfg(feature = "less")]
    appy_mcappface.push(applet {
      name: "less",
      main: "less",
      entrypoint: Entrypoint::CStyle(crate::miscutils::less::less_main),
      install_loc: InstallLoc::DIR_USR_BIN,
      usage: std::include_str!("../usage/less"),
    });
    #[cfg(feature = "link")]
    appy_mcappface.push(applet {
      name: "link",
      main: "link",
      entrypoint: Entrypoint::CStyle(crate::coreutils::link::link_main),
      install_loc: InstallLoc::DIR_BIN,
      usage: std::include_str!("../usage/link"),
    });
    #[cfg(feature = "linux32")]
    appy_mcappface.push(applet {
      name: "linux32",
      main: "setarch",
      entrypoint: Entrypoint::CStyle(crate::util_linux::setarch::setarch_main),
      install_loc: InstallLoc::DIR_BIN,
      usage: std::include_str!("../usage/linux32"),
    });
    #[cfg(feature = "linux64")]
    appy_mcappface.push(applet {
      name: "linux64",
      main: "setarch",
      entrypoint: Entrypoint::CStyle(crate::util_linux::setarch::setarch_main),
      install_loc: InstallLoc::DIR_BIN,
      usage: std::include_str!("../usage/linux64"),
    });
    #[cfg(feature = "linuxrc")]
    appy_mcappface.push(applet {
      name: "linuxrc",
      main: "init",
      entrypoint: Entrypoint::CStyle(crate::init::init::init_main),
      install_loc: InstallLoc::DIR_ROOT,
      usage: std::include_str!("../usage/linuxrc"),
    });
    #[cfg(feature = "ln")]
    appy_mcappface.push(applet {
      name: "ln",
      main: "ln",
      entrypoint: Entrypoint::CStyle(crate::coreutils::ln::ln_main),
      install_loc: InstallLoc::DIR_BIN,
      usage: std::include_str!("../usage/ln"),
    });
    #[cfg(feature = "loadfont")]
    appy_mcappface.push(applet {
      name: "loadfont",
      main: "loadfont",
      entrypoint: Entrypoint::CStyle(crate::console_tools::loadfont::loadfont_main),
      install_loc: InstallLoc::DIR_USR_SBIN,
      usage: std::include_str!("../usage/loadfont"),
    });
    #[cfg(feature = "loadkmap")]
    appy_mcappface.push(applet {
      name: "loadkmap",
      main: "loadkmap",
      entrypoint: Entrypoint::CStyle(crate::console_tools::loadkmap::loadkmap_main),
      install_loc: InstallLoc::DIR_SBIN,
      usage: std::include_str!("../usage/loadkmap"),
    });
    #[cfg(feature = "logger")]
    appy_mcappface.push(applet {
      name: "logger",
      main: "logger",
      entrypoint: Entrypoint::CStyle(crate::sysklogd::syslogd_and_logger::logger_main),
      install_loc: InstallLoc::DIR_USR_BIN,
      usage: std::include_str!("../usage/logger"),
    });
    #[cfg(feature = "login")]
    appy_mcappface.push(applet {
      name: "login",
      main: "login",
      entrypoint: Entrypoint::CStyle(crate::loginutils::login::login_main),
      install_loc: InstallLoc::DIR_BIN,
      usage: std::include_str!("../usage/login"),
    });
    #[cfg(feature = "logname")]
    appy_mcappface.push(applet {
      name: "logname",
      main: "logname",
      entrypoint: Entrypoint::CStyle(crate::coreutils::logname::logname_main),
      install_loc: InstallLoc::DIR_USR_BIN,
      usage: std::include_str!("../usage/logname"),
    });
    #[cfg(feature = "logread")]
    appy_mcappface.push(applet {
      name: "logread",
      main: "logread",
      entrypoint: Entrypoint::CStyle(crate::sysklogd::logread::logread_main),
      install_loc: InstallLoc::DIR_SBIN,
      usage: std::include_str!("../usage/logread"),
    });
    #[cfg(feature = "losetup")]
    appy_mcappface.push(applet {
      name: "losetup",
      main: "losetup",
      entrypoint: Entrypoint::CStyle(crate::util_linux::losetup::losetup_main),
      install_loc: InstallLoc::DIR_SBIN,
      usage: std::include_str!("../usage/losetup"),
    });
    #[cfg(feature = "lpd")]
    appy_mcappface.push(applet {
      name: "lpd",
      main: "lpd",
      entrypoint: Entrypoint::CStyle(crate::printutils::lpd::lpd_main),
      install_loc: InstallLoc::DIR_USR_SBIN,
      usage: std::include_str!("../usage/lpd"),
    });
    #[cfg(feature = "lpq")]
    appy_mcappface.push(applet {
      name: "lpq",
      main: "lpqr",
      entrypoint: Entrypoint::CStyle(crate::printutils::lpr::lpqr_main),
      install_loc: InstallLoc::DIR_USR_BIN,
      usage: std::include_str!("../usage/lpq"),
    });
    #[cfg(feature = "lpr")]
    appy_mcappface.push(applet {
      name: "lpr",
      main: "lpqr",
      entrypoint: Entrypoint::CStyle(crate::printutils::lpr::lpqr_main),
      install_loc: InstallLoc::DIR_USR_BIN,
      usage: std::include_str!("../usage/lpr"),
    });
    #[cfg(feature = "ls")]
    appy_mcappface.push(applet {
      name: "ls",
      main: "ls",
      entrypoint: Entrypoint::CStyle(crate::coreutils::ls::ls_main),
      install_loc: InstallLoc::DIR_BIN,
      usage: std::include_str!("../usage/ls"),
    });
    #[cfg(feature = "lsattr")]
    appy_mcappface.push(applet {
      name: "lsattr",
      main: "lsattr",
      entrypoint: Entrypoint::CStyle(lsatcrate::coreutils::tr::tr_main),
      install_loc: InstallLoc::DIR_BIN,
      usage: std::include_str!("../usage/lsattr"),
    });
    #[cfg(feature = "lsmod")]
    appy_mcappface.push(applet {
      name: "lsmod",
      main: "lsmod",
      entrypoint: Entrypoint::CStyle(lsmcrate::coreutils::od::od_main),
      install_loc: InstallLoc::DIR_SBIN,
      usage: std::include_str!("../usage/lsmod"),
    });
    #[cfg(feature = "lsof")]
    appy_mcappface.push(applet {
      name: "lsof",
      main: "lsof",
      entrypoint: Entrypoint::CStyle(crate::procps::lsof::lsof_main),
      install_loc: InstallLoc::DIR_USR_BIN,
      usage: std::include_str!("../usage/lsof"),
    });
    #[cfg(feature = "lspci")]
    appy_mcappface.push(applet {
      name: "lspci",
      main: "lspci",
      entrypoint: Entrypoint::CStyle(crate::util_linux::lspci::lspci_main),
      install_loc: InstallLoc::DIR_USR_BIN,
      usage: std::include_str!("../usage/lspci"),
    });
    #[cfg(feature = "lsscsi")]
    appy_mcappface.push(applet {
      name: "lsscsi",
      main: "lsscsi",
      entrypoint: Entrypoint::CStyle(crate::miscutils::lsscsi::lsscsi_main),
      install_loc: InstallLoc::DIR_USR_BIN,
      usage: std::include_str!("../usage/lsscsi"),
    });
    #[cfg(feature = "lsusb")]
    appy_mcappface.push(applet {
      name: "lsusb",
      main: "lsusb",
      entrypoint: Entrypoint::CStyle(crate::util_linux::lsusb::lsusb_main),
      install_loc: InstallLoc::DIR_USR_BIN,
      usage: std::include_str!("../usage/lsusb"),
    });
    #[cfg(feature = "lzcat")]
    appy_mcappface.push(applet {
      name: "lzcat",
      main: "unlzma",
      entrypoint: Entrypoint::CStyle(crate::archival::bbunzip::unlzma_main),
      install_loc: InstallLoc::DIR_USR_BIN,
      usage: std::include_str!("../usage/lzcat"),
    });
    #[cfg(feature = "lzma")]
    appy_mcappface.push(applet {
      name: "lzma",
      main: "unlzma",
      entrypoint: Entrypoint::CStyle(crate::archival::bbunzip::unlzma_main),
      install_loc: InstallLoc::DIR_USR_BIN,
      usage: std::include_str!("../usage/lzma"),
    });
    #[cfg(feature = "lzop")]
    appy_mcappface.push(applet {
      name: "lzop",
      main: "lzop",
      entrypoint: Entrypoint::CStyle(crate::archival::lzop::lzop_main),
      install_loc: InstallLoc::DIR_BIN,
      usage: std::include_str!("../usage/lzop"),
    });
    #[cfg(feature = "makedevs")]
    appy_mcappface.push(applet {
      name: "makedevs",
      main: "makedevs",
      entrypoint: Entrypoint::CStyle(crate::miscutils::makedevs::makedevs_main),
      install_loc: InstallLoc::DIR_SBIN,
      usage: std::include_str!("../usage/makedevs"),
    });
    #[cfg(feature = "makemime")]
    appy_mcappface.push(applet {
      name: "makemime",
      main: "makemime",
      entrypoint: Entrypoint::CStyle(crate::mailutils::makemime::makemime_main),
      install_loc: InstallLoc::DIR_BIN,
      usage: std::include_str!("../usage/makemime"),
    });
    #[cfg(feature = "man")]
    appy_mcappface.push(applet {
      name: "man",
      main: "man",
      entrypoint: Entrypoint::CStyle(crate::miscutils::man::man_main),
      install_loc: InstallLoc::DIR_USR_BIN,
      usage: std::include_str!("../usage/man"),
    });
    #[cfg(feature = "md5sum")]
    appy_mcappface.push(applet {
      name: "md5sum",
      main: "md5_sha1_sum",
      entrypoint: Entrypoint::CStyle(crate::coreutils::md5_sha1_sum::md5_sha1_crate::coreutils::sum::sum_main),
      install_loc: InstallLoc::DIR_USR_BIN,
      usage: std::include_str!("../usage/md5sum"),
    });
    #[cfg(feature = "mdev")]
    appy_mcappface.push(applet {
      name: "mdev",
      main: "mdev",
      entrypoint: Entrypoint::CStyle(crate::util_linux::mdev::mdev_main),
      install_loc: InstallLoc::DIR_SBIN,
      usage: std::include_str!("../usage/mdev"),
    });
    #[cfg(feature = "mesg")]
    appy_mcappface.push(applet {
      name: "mesg",
      main: "mesg",
      entrypoint: Entrypoint::CStyle(crate::util_linux::mesg::mesg_main),
      install_loc: InstallLoc::DIR_USR_BIN,
      usage: std::include_str!("../usage/mesg"),
    });
    #[cfg(feature = "microcom")]
    appy_mcappface.push(applet {
      name: "microcom",
      main: "microcom",
      entrypoint: Entrypoint::CStyle(crate::miscutils::microcom::microcom_main),
      install_loc: InstallLoc::DIR_USR_BIN,
      usage: std::include_str!("../usage/microcom"),
    });
    #[cfg(feature = "mkdir")]
    appy_mcappface.push(applet {
      name: "mkdir",
      main: "mkdir",
      entrypoint: Entrypoint::CStyle(crate::coreutils::mkdir::mkdir_main),
      install_loc: InstallLoc::DIR_BIN,
      usage: std::include_str!("../usage/mkdir"),
    });
    #[cfg(feature = "mkdosfs")]
    appy_mcappface.push(applet {
      name: "mkdosfs",
      main: "mkfs_vfat",
      entrypoint: Entrypoint::CStyle(crate::util_linux::mkfs_vfat::mkfs_vfat_main),
      install_loc: InstallLoc::DIR_SBIN,
      usage: std::include_str!("../usage/mkdosfs"),
    });
    #[cfg(feature = "mke2fs")]
    appy_mcappface.push(applet {
      name: "mke2fs",
      main: "mkfs_ext2",
      entrypoint: Entrypoint::CStyle(crate::util_linux::mkfs_ext2::mkfs_ext2_main),
      install_loc: InstallLoc::DIR_SBIN,
      usage: std::include_str!("../usage/mke2fs"),
    });
    #[cfg(feature = "mkfifo")]
    appy_mcappface.push(applet {
      name: "mkfifo",
      main: "mkfifo",
      entrypoint: Entrypoint::CStyle(crate::coreutils::mkfifo::mkfifo_main),
      install_loc: InstallLoc::DIR_USR_BIN,
      usage: std::include_str!("../usage/mkfifo"),
    });
    #[cfg(feature = "mkfs_ext2")]
    appy_mcappface.push(applet {
      name: "mkfs.ext2",
      main: "mkfs_ext2",
      entrypoint: Entrypoint::CStyle(crate::util_linux::mkfs_ext2::mkfs_ext2_main),
      install_loc: InstallLoc::DIR_SBIN,
      usage: std::include_str!("../usage/mkfs.ext2"),
    });
    #[cfg(feature = "mkfs_minix")]
    appy_mcappface.push(applet {
      name: "mkfs.minix",
      main: "mkfs_minix",
      entrypoint: Entrypoint::CStyle(crate::util_linux::mkfs_minix::mkfs_minix_main),
      install_loc: InstallLoc::DIR_SBIN,
      usage: std::include_str!("../usage/mkfs.minix"),
    });
    #[cfg(feature = "mkfs_vfat")]
    appy_mcappface.push(applet {
      name: "mkfs.vfat",
      main: "mkfs_vfat",
      entrypoint: Entrypoint::CStyle(crate::util_linux::mkfs_vfat::mkfs_vfat_main),
      install_loc: InstallLoc::DIR_SBIN,
      usage: std::include_str!("../usage/mkfs.vfat"),
    });
    #[cfg(feature = "mknod")]
    appy_mcappface.push(applet {
      name: "mknod",
      main: "mknod",
      entrypoint: Entrypoint::CStyle(crate::coreutils::mknod::mkncrate::coreutils::od::od_main),
      install_loc: InstallLoc::DIR_BIN,
      usage: std::include_str!("../usage/mknod"),
    });
    #[cfg(feature = "mkpasswd")]
    appy_mcappface.push(applet {
      name: "mkpasswd",
      main: "cryptpw",
      entrypoint: Entrypoint::CStyle(crate::loginutils::cryptpw::cryptpw_main),
      install_loc: InstallLoc::DIR_USR_BIN,
      usage: std::include_str!("../usage/mkpasswd"),
    });
    #[cfg(feature = "mkswap")]
    appy_mcappface.push(applet {
      name: "mkswap",
      main: "mkswap",
      entrypoint: Entrypoint::CStyle(crate::util_linux::mkswap::mkswap_main),
      install_loc: InstallLoc::DIR_SBIN,
      usage: std::include_str!("../usage/mkswap"),
    });
    #[cfg(feature = "mktemp")]
    appy_mcappface.push(applet {
      name: "mktemp",
      main: "mktemp",
      entrypoint: Entrypoint::CStyle(crate::coreutils::mktemp::mktemp_main),
      install_loc: InstallLoc::DIR_BIN,
      usage: std::include_str!("../usage/mktemp"),
    });
    #[cfg(feature = "modinfo")]
    appy_mcappface.push(applet {
      name: "modinfo",
      main: "modinfo",
      entrypoint: Entrypoint::CStyle(crate::modutils::modinfo::modinfo_main),
      install_loc: InstallLoc::DIR_SBIN,
      usage: std::include_str!("../usage/modinfo"),
    });
    #[cfg(feature = "modprobe")]
    appy_mcappface.push(applet {
      name: "modprobe",
      main: "modprobe",
      entrypoint: Entrypoint::CStyle(crate::modutils::modprobe_small::modprobe_main),
      install_loc: InstallLoc::DIR_SBIN,
      usage: std::include_str!("../usage/modprobe"),
    });
    #[cfg(feature = "more")]
    appy_mcappface.push(applet {
      name: "more",
      main: "more",
      entrypoint: Entrypoint::CStyle(crate::util_linux::more::more_main),
      install_loc: InstallLoc::DIR_BIN,
      usage: std::include_str!("../usage/more"),
    });
    #[cfg(feature = "mount")]
    appy_mcappface.push(applet {
      name: "mount",
      main: "mount",
      entrypoint: Entrypoint::CStyle(crate::util_linux::mount::mount_main),
      install_loc: InstallLoc::DIR_BIN,
      usage: std::include_str!("../usage/mount"),
    });
    #[cfg(feature = "mountpoint")]
    appy_mcappface.push(applet {
      name: "mountpoint",
      main: "mountpoint",
      entrypoint: Entrypoint::CStyle(crate::util_linux::mountpoint::mountpoint_main),
      install_loc: InstallLoc::DIR_BIN,
      usage: std::include_str!("../usage/mountpoint"),
    });
    #[cfg(feature = "mpstat")]
    appy_mcappface.push(applet {
      name: "mpstat",
      main: "mpstat",
      entrypoint: Entrypoint::CStyle(mpcrate::coreutils::stat::stat_main),
      install_loc: InstallLoc::DIR_BIN,
      usage: std::include_str!("../usage/mpstat"),
    });
    #[cfg(feature = "mt")]
    appy_mcappface.push(applet {
      name: "mt",
      main: "mt",
      entrypoint: Entrypoint::CStyle(crate::miscutils::mt::mt_main),
      install_loc: InstallLoc::DIR_BIN,
      usage: std::include_str!("../usage/mt"),
    });
    #[cfg(feature = "mv")]
    appy_mcappface.push(applet {
      name: "mv",
      main: "mv",
      entrypoint: Entrypoint::CStyle(crate::coreutils::mv::mv_main),
      install_loc: InstallLoc::DIR_BIN,
      usage: std::include_str!("../usage/mv"),
    });
    #[cfg(feature = "nameif")]
    appy_mcappface.push(applet {
      name: "nameif",
      main: "nameif",
      entrypoint: Entrypoint::CStyle(crate::networking::nameif::nameif_main),
      install_loc: InstallLoc::DIR_SBIN,
      usage: std::include_str!("../usage/nameif"),
    });
    #[cfg(feature = "nanddump")]
    appy_mcappface.push(applet {
      name: "nanddump",
      main: "nandwrite",
      entrypoint: Entrypoint::CStyle(crate::miscutils::nandwrite::nandwrite_main),
      install_loc: InstallLoc::DIR_USR_SBIN,
      usage: std::include_str!("../usage/nanddump"),
    });
    #[cfg(feature = "nandwrite")]
    appy_mcappface.push(applet {
      name: "nandwrite",
      main: "nandwrite",
      entrypoint: Entrypoint::CStyle(crate::miscutils::nandwrite::nandwrite_main),
      install_loc: InstallLoc::DIR_USR_SBIN,
      usage: std::include_str!("../usage/nandwrite"),
    });
    #[cfg(feature = "nbd-client")]
    appy_mcappface.push(applet {
      name: "nbd-client",
      main: "nbdclient",
      entrypoint: Entrypoint::CStyle(crate::networking::nbd_client::nbdclient_main),
      install_loc: InstallLoc::DIR_USR_SBIN,
      usage: std::include_str!("../usage/nbd-client"),
    });
    #[cfg(feature = "nc")]
    appy_mcappface.push(applet {
      name: "nc",
      main: "nc",
      entrypoint: Entrypoint::CStyle(crate::networking::nc::nc_main),
      install_loc: InstallLoc::DIR_USR_BIN,
      usage: std::include_str!("../usage/nc"),
    });
    #[cfg(feature = "netstat")]
    appy_mcappface.push(applet {
      name: "netstat",
      main: "netstat",
      entrypoint: Entrypoint::CStyle(netcrate::coreutils::stat::stat_main),
      install_loc: InstallLoc::DIR_BIN,
      usage: std::include_str!("../usage/netstat"),
    });
    #[cfg(feature = "nice")]
    appy_mcappface.push(applet {
      name: "nice",
      main: "nice",
      entrypoint: Entrypoint::CStyle(crate::coreutils::nice::nice_main),
      install_loc: InstallLoc::DIR_BIN,
      usage: std::include_str!("../usage/nice"),
    });
    #[cfg(feature = "nl")]
    appy_mcappface.push(applet {
      name: "nl",
      main: "nl",
      entrypoint: Entrypoint::CStyle(crate::coreutils::nl::nl_main),
      install_loc: InstallLoc::DIR_USR_BIN,
      usage: std::include_str!("../usage/nl"),
    });
    #[cfg(feature = "nmeter")]
    appy_mcappface.push(applet {
      name: "nmeter",
      main: "nmeter",
      entrypoint: Entrypoint::CStyle(crate::procps::nmeter::nmeter_main),
      install_loc: InstallLoc::DIR_USR_BIN,
      usage: std::include_str!("../usage/nmeter"),
    });
    #[cfg(feature = "nohup")]
    appy_mcappface.push(applet {
      name: "nohup",
      main: "nohup",
      entrypoint: Entrypoint::CStyle(crate::coreutils::nohup::nohup_main),
      install_loc: InstallLoc::DIR_USR_BIN,
      usage: std::include_str!("../usage/nohup"),
    });
    #[cfg(feature = "nproc")]
    appy_mcappface.push(applet {
      name: "nproc",
      main: "nproc",
      entrypoint: Entrypoint::CStyle(crate::coreutils::nproc::nproc_main),
      install_loc: InstallLoc::DIR_USR_BIN,
      usage: std::include_str!("../usage/nproc"),
    });
    #[cfg(feature = "nsenter")]
    appy_mcappface.push(applet {
      name: "nsenter",
      main: "nsenter",
      entrypoint: Entrypoint::CStyle(crate::util_linux::nsenter::nsenter_main),
      install_loc: InstallLoc::DIR_USR_BIN,
      usage: std::include_str!("../usage/nsenter"),
    });
    #[cfg(feature = "nslookup")]
    appy_mcappface.push(applet {
      name: "nslookup",
      main: "nslookup",
      entrypoint: Entrypoint::CStyle(crate::networking::nslookup::nslookup_main),
      install_loc: InstallLoc::DIR_USR_BIN,
      usage: std::include_str!("../usage/nslookup"),
    });
    #[cfg(feature = "ntpd")]
    appy_mcappface.push(applet {
      name: "ntpd",
      main: "ntpd",
      entrypoint: Entrypoint::CStyle(crate::networking::ntpd::ntpd_main),
      install_loc: InstallLoc::DIR_USR_SBIN,
      usage: std::include_str!("../usage/ntpd"),
    });
    #[cfg(feature = "nuke")]
    appy_mcappface.push(applet {
      name: "nuke",
      main: "nuke",
      entrypoint: Entrypoint::CStyle(crate::klibc_utils::nuke::nuke_main),
      install_loc: InstallLoc::DIR_BIN,
      usage: std::include_str!("../usage/nuke"),
    });
    #[cfg(feature = "od")]
    appy_mcappface.push(applet {
      name: "od",
      main: "od",
      entrypoint: Entrypoint::CStyle(crate::coreutils::od::od_main),
      install_loc: InstallLoc::DIR_USR_BIN,
      usage: std::include_str!("../usage/od"),
    });
    #[cfg(feature = "openvt")]
    appy_mcappface.push(applet {
      name: "openvt",
      main: "openvt",
      entrypoint: Entrypoint::CStyle(crate::console_tools::openvt::openvt_main),
      install_loc: InstallLoc::DIR_USR_BIN,
      usage: std::include_str!("../usage/openvt"),
    });
    #[cfg(feature = "partprobe")]
    appy_mcappface.push(applet {
      name: "partprobe",
      main: "partprobe",
      entrypoint: Entrypoint::CStyle(crate::miscutils::partprobe::partprobe_main),
      install_loc: InstallLoc::DIR_USR_SBIN,
      usage: std::include_str!("../usage/partprobe"),
    });
    #[cfg(feature = "passwd")]
    appy_mcappface.push(applet {
      name: "passwd",
      main: "passwd",
      entrypoint: Entrypoint::CStyle(crate::loginutils::passwd::passwd_main),
      install_loc: InstallLoc::DIR_USR_BIN,
      usage: std::include_str!("../usage/passwd"),
    });
    #[cfg(feature = "paste")]
    appy_mcappface.push(applet {
      name: "paste",
      main: "paste",
      entrypoint: Entrypoint::CStyle(crate::coreutils::paste::paste_main),
      install_loc: InstallLoc::DIR_USR_BIN,
      usage: std::include_str!("../usage/paste"),
    });
    #[cfg(feature = "patch")]
    appy_mcappface.push(applet {
      name: "patch",
      main: "patch",
      entrypoint: Entrypoint::CStyle(crate::editors::patch::patch_main),
      install_loc: InstallLoc::DIR_USR_BIN,
      usage: std::include_str!("../usage/patch"),
    });
    #[cfg(feature = "pgrep")]
    appy_mcappface.push(applet {
      name: "pgrep",
      main: "pgrep",
      entrypoint: Entrypoint::CStyle(pcrate::findutils::grep::grep_main),
      install_loc: InstallLoc::DIR_USR_BIN,
      usage: std::include_str!("../usage/pgrep"),
    });
    #[cfg(feature = "pidof")]
    appy_mcappface.push(applet {
      name: "pidof",
      main: "pidof",
      entrypoint: Entrypoint::CStyle(crate::procps::pidof::pidof_main),
      install_loc: InstallLoc::DIR_BIN,
      usage: std::include_str!("../usage/pidof"),
    });
    #[cfg(feature = "ping")]
    appy_mcappface.push(applet {
      name: "ping",
      main: "ping",
      entrypoint: Entrypoint::CStyle(crate::networking::ping::ping_main),
      install_loc: InstallLoc::DIR_BIN,
      usage: std::include_str!("../usage/ping"),
    });
    #[cfg(feature = "ping6")]
    appy_mcappface.push(applet {
      name: "ping6",
      main: "ping6",
      entrypoint: Entrypoint::CStyle(crate::networking::ping::ping6_main),
      install_loc: InstallLoc::DIR_BIN,
      usage: std::include_str!("../usage/ping6"),
    });
    #[cfg(feature = "pipe_progress")]
    appy_mcappface.push(applet {
      name: "pipe_progress",
      main: "pipe_progress",
      entrypoint: Entrypoint::CStyle(crate::debianutils::pipe_progress::pipe_progress_main),
      install_loc: InstallLoc::DIR_BIN,
      usage: std::include_str!("../usage/pipe_progress"),
    });
    #[cfg(feature = "pivot_root")]
    appy_mcappface.push(applet {
      name: "pivot_root",
      main: "pivot_root",
      entrypoint: Entrypoint::CStyle(crate::util_linux::pivot_root::pivot_root_main),
      install_loc: InstallLoc::DIR_SBIN,
      usage: std::include_str!("../usage/pivot_root"),
    });
    #[cfg(feature = "pkill")]
    appy_mcappface.push(applet {
      name: "pkill",
      main: "pgrep",
      entrypoint: Entrypoint::CStyle(pcrate::findutils::grep::grep_main),
      install_loc: InstallLoc::DIR_USR_BIN,
      usage: std::include_str!("../usage/pkill"),
    });
    #[cfg(feature = "pmap")]
    appy_mcappface.push(applet {
      name: "pmap",
      main: "pmap",
      entrypoint: Entrypoint::CStyle(crate::procps::pmap::pmap_main),
      install_loc: InstallLoc::DIR_USR_BIN,
      usage: std::include_str!("../usage/pmap"),
    });
    #[cfg(feature = "popmaildir")]
    appy_mcappface.push(applet {
      name: "popmaildir",
      main: "popmaildir",
      entrypoint: Entrypoint::CStyle(crate::mailutils::popmaildir::popmaildir_main),
      install_loc: InstallLoc::DIR_USR_SBIN,
      usage: std::include_str!("../usage/popmaildir"),
    });
    #[cfg(feature = "poweroff")]
    appy_mcappface.push(applet {
      name: "poweroff",
      main: "halt",
      entrypoint: Entrypoint::CStyle(crate::init::halt::halt_main),
      install_loc: InstallLoc::DIR_SBIN,
      usage: std::include_str!("../usage/poweroff"),
    });
    #[cfg(feature = "powertop")]
    appy_mcappface.push(applet {
      name: "powertop",
      main: "powertop",
      entrypoint: Entrypoint::CStyle(crate::procps::powertop::powercrate::procps::top::top_main),
      install_loc: InstallLoc::DIR_USR_SBIN,
      usage: std::include_str!("../usage/powertop"),
    });
    #[cfg(feature = "printenv")]
    appy_mcappface.push(applet {
      name: "printenv",
      main: "printenv",
      entrypoint: Entrypoint::CStyle(printcrate::coreutils::env::env_main),
      install_loc: InstallLoc::DIR_BIN,
      usage: std::include_str!("../usage/printenv"),
    });
    #[cfg(feature = "printf")]
    appy_mcappface.push(applet {
      name: "printf",
      main: "printf",
      entrypoint: Entrypoint::CStyle(crate::coreutils::printf::printf_main),
      install_loc: InstallLoc::DIR_USR_BIN,
      usage: std::include_str!("../usage/printf"),
    });
    #[cfg(feature = "ps")]
    appy_mcappface.push(applet {
      name: "ps",
      main: "ps",
      entrypoint: Entrypoint::CStyle(crate::procps::ps::ps_main),
      install_loc: InstallLoc::DIR_BIN,
      usage: std::include_str!("../usage/ps"),
    });
    #[cfg(feature = "pscan")]
    appy_mcappface.push(applet {
      name: "pscan",
      main: "pscan",
      entrypoint: Entrypoint::CStyle(crate::networking::pscan::pscan_main),
      install_loc: InstallLoc::DIR_USR_BIN,
      usage: std::include_str!("../usage/pscan"),
    });
    #[cfg(feature = "pstree")]
    appy_mcappface.push(applet {
      name: "pstree",
      main: "pstree",
      entrypoint: Entrypoint::CStyle(crate::procps::pstree::pstree_main),
      install_loc: InstallLoc::DIR_USR_BIN,
      usage: std::include_str!("../usage/pstree"),
    });
    #[cfg(feature = "pwd")]
    appy_mcappface.push(applet {
      name: "pwd",
      main: "pwd",
      entrypoint: Entrypoint::CStyle(crate::coreutils::pwd::pwd_main),
      install_loc: InstallLoc::DIR_BIN,
      usage: std::include_str!("../usage/pwd"),
    });
    #[cfg(feature = "pwdx")]
    appy_mcappface.push(applet {
      name: "pwdx",
      main: "pwdx",
      entrypoint: Entrypoint::CStyle(crate::procps::pwdx::pwdx_main),
      install_loc: InstallLoc::DIR_USR_BIN,
      usage: std::include_str!("../usage/pwdx"),
    });
    #[cfg(feature = "raidautorun")]
    appy_mcappface.push(applet {
      name: "raidautorun",
      main: "raidautorun",
      entrypoint: Entrypoint::CStyle(crate::miscutils::raidautorun::raidautorun_main),
      install_loc: InstallLoc::DIR_SBIN,
      usage: std::include_str!("../usage/raidautorun"),
    });
    #[cfg(feature = "rdate")]
    appy_mcappface.push(applet {
      name: "rdate",
      main: "rdate",
      entrypoint: Entrypoint::CStyle(rcrate::coreutils::date::date_main),
      install_loc: InstallLoc::DIR_USR_SBIN,
      usage: std::include_str!("../usage/rdate"),
    });
    #[cfg(feature = "rdev")]
    appy_mcappface.push(applet {
      name: "rdev",
      main: "rdev",
      entrypoint: Entrypoint::CStyle(crate::util_linux::rdev::rdev_main),
      install_loc: InstallLoc::DIR_USR_SBIN,
      usage: std::include_str!("../usage/rdev"),
    });
    #[cfg(feature = "readahead")]
    appy_mcappface.push(applet {
      name: "readahead",
      main: "readahead",
      entrypoint: Entrypoint::CStyle(readacrate::coreutils::head::head_main),
      install_loc: InstallLoc::DIR_USR_SBIN,
      usage: std::include_str!("../usage/readahead"),
    });
    #[cfg(feature = "readlink")]
    appy_mcappface.push(applet {
      name: "readlink",
      main: "readlink",
      entrypoint: Entrypoint::CStyle(readcrate::coreutils::link::link_main),
      install_loc: InstallLoc::DIR_USR_BIN,
      usage: std::include_str!("../usage/readlink"),
    });
    #[cfg(feature = "readprofile")]
    appy_mcappface.push(applet {
      name: "readprofile",
      main: "readprofile",
      entrypoint: Entrypoint::CStyle(crate::util_linux::readprofile::readprofile_main),
      install_loc: InstallLoc::DIR_USR_SBIN,
      usage: std::include_str!("../usage/readprofile"),
    });
    #[cfg(feature = "realpath")]
    appy_mcappface.push(applet {
      name: "realpath",
      main: "realpath",
      entrypoint: Entrypoint::CStyle(crate::coreutils::realpath::realpath_main),
      install_loc: InstallLoc::DIR_USR_BIN,
      usage: std::include_str!("../usage/realpath"),
    });
    #[cfg(feature = "reboot")]
    appy_mcappface.push(applet {
      name: "reboot",
      main: "halt",
      entrypoint: Entrypoint::CStyle(crate::init::halt::halt_main),
      install_loc: InstallLoc::DIR_SBIN,
      usage: std::include_str!("../usage/reboot"),
    });
    #[cfg(feature = "reformime")]
    appy_mcappface.push(applet {
      name: "reformime",
      main: "reformime",
      entrypoint: Entrypoint::CStyle(crate::mailutils::reformime::reformime_main),
      install_loc: InstallLoc::DIR_BIN,
      usage: std::include_str!("../usage/reformime"),
    });
    #[cfg(feature = "remove-shell")]
    appy_mcappface.push(applet {
      name: "remove-shell",
      main: "add_remove_shell",
      entrypoint: Entrypoint::CStyle(crate::loginutils::add_remove_shell::add_remove_shell_main),
      install_loc: InstallLoc::DIR_USR_SBIN,
      usage: std::include_str!("../usage/remove-shell"),
    });
    #[cfg(feature = "renice")]
    appy_mcappface.push(applet {
      name: "renice",
      main: "renice",
      entrypoint: Entrypoint::CStyle(recrate::coreutils::nice::nice_main),
      install_loc: InstallLoc::DIR_USR_BIN,
      usage: std::include_str!("../usage/renice"),
    });
    #[cfg(feature = "reset")]
    appy_mcappface.push(applet {
      name: "reset",
      main: "reset",
      entrypoint: Entrypoint::CStyle(crate::console_tools::reset::reset_main),
      install_loc: InstallLoc::DIR_USR_BIN,
      usage: std::include_str!("../usage/reset"),
    });
    #[cfg(feature = "resize")]
    appy_mcappface.push(applet {
      name: "resize",
      main: "resize",
      entrypoint: Entrypoint::CStyle(crate::console_tools::resize::resize_main),
      install_loc: InstallLoc::DIR_USR_BIN,
      usage: std::include_str!("../usage/resize"),
    });
    #[cfg(feature = "resume")]
    appy_mcappface.push(applet {
      name: "resume",
      main: "resume",
      entrypoint: Entrypoint::CStyle(crate::klibc_utils::resume::resume_main),
      install_loc: InstallLoc::DIR_BIN,
      usage: std::include_str!("../usage/resume"),
    });
    #[cfg(feature = "rev")]
    appy_mcappface.push(applet {
      name: "rev",
      main: "rev",
      entrypoint: Entrypoint::CStyle(crate::util_linux::rev::rev_main),
      install_loc: InstallLoc::DIR_BIN,
      usage: std::include_str!("../usage/rev"),
    });
    #[cfg(feature = "rm")]
    appy_mcappface.push(applet {
      name: "rm",
      main: "rm",
      entrypoint: Entrypoint::CStyle(crate::coreutils::rm::rm_main),
      install_loc: InstallLoc::DIR_BIN,
      usage: std::include_str!("../usage/rm"),
    });
    #[cfg(feature = "rmdir")]
    appy_mcappface.push(applet {
      name: "rmdir",
      main: "rmdir",
      entrypoint: Entrypoint::CStyle(crate::coreutils::rmdir::rmdir_main),
      install_loc: InstallLoc::DIR_BIN,
      usage: std::include_str!("../usage/rmdir"),
    });
    #[cfg(feature = "rmmod")]
    appy_mcappface.push(applet {
      name: "rmmod",
      main: "modprobe",
      entrypoint: Entrypoint::CStyle(crate::modutils::modprobe_small::modprobe_main),
      install_loc: InstallLoc::DIR_SBIN,
      usage: std::include_str!("../usage/rmmod"),
    });
    #[cfg(feature = "route")]
    appy_mcappface.push(applet {
      name: "route",
      main: "route",
      entrypoint: Entrypoint::CStyle(crate::networking::route::route_main),
      install_loc: InstallLoc::DIR_SBIN,
      usage: std::include_str!("../usage/route"),
    });
    #[cfg(feature = "rpm")]
    appy_mcappface.push(applet {
      name: "rpm",
      main: "rpm",
      entrypoint: Entrypoint::CStyle(crate::archival::rpm::rpm_main),
      install_loc: InstallLoc::DIR_BIN,
      usage: std::include_str!("../usage/rpm"),
    });
    #[cfg(feature = "rpm2cpio")]
    appy_mcappface.push(applet {
      name: "rpm2cpio",
      main: "rpm2cpio",
      entrypoint: Entrypoint::CStyle(rpm2crate::archival::cpio::cpio_main),
      install_loc: InstallLoc::DIR_USR_BIN,
      usage: std::include_str!("../usage/rpm2cpio"),
    });
    #[cfg(feature = "rtcwake")]
    appy_mcappface.push(applet {
      name: "rtcwake",
      main: "rtcwake",
      entrypoint: Entrypoint::CStyle(crate::util_linux::rtcwake::rtcwake_main),
      install_loc: InstallLoc::DIR_USR_SBIN,
      usage: std::include_str!("../usage/rtcwake"),
    });
    #[cfg(feature = "run-init")]
    appy_mcappface.push(applet {
      name: "run-init",
      main: "switch_root",
      entrypoint: Entrypoint::CStyle(crate::util_linux::switch_root::switch_root_main),
      install_loc: InstallLoc::DIR_SBIN,
      usage: std::include_str!("../usage/run-init"),
    });
    #[cfg(feature = "run-parts")]
    appy_mcappface.push(applet {
      name: "run-parts",
      main: "run_parts",
      entrypoint: Entrypoint::CStyle(crate::debianutils::run_parts::run_parcrate::miscutils::ts::ts_main),
      install_loc: InstallLoc::DIR_BIN,
      usage: std::include_str!("../usage/run-parts"),
    });
    #[cfg(feature = "runlevel")]
    appy_mcappface.push(applet {
      name: "runlevel",
      main: "runlevel",
      entrypoint: Entrypoint::CStyle(crate::miscutils::runlevel::runlevel_main),
      install_loc: InstallLoc::DIR_SBIN,
      usage: std::include_str!("../usage/runlevel"),
    });
    #[cfg(feature = "runsv")]
    appy_mcappface.push(applet {
      name: "runsv",
      main: "runsv",
      entrypoint: Entrypoint::CStyle(crate::runit::runsv::runcrate::runit::sv::sv_main),
      install_loc: InstallLoc::DIR_USR_BIN,
      usage: std::include_str!("../usage/runsv"),
    });
    #[cfg(feature = "runsvdir")]
    appy_mcappface.push(applet {
      name: "runsvdir",
      main: "runsvdir",
      entrypoint: Entrypoint::CStyle(crate::runit::runsvdir::runsvdir_main),
      install_loc: InstallLoc::DIR_USR_BIN,
      usage: std::include_str!("../usage/runsvdir"),
    });
    #[cfg(feature = "rx")]
    appy_mcappface.push(applet {
      name: "rx",
      main: "rx",
      entrypoint: Entrypoint::CStyle(crate::miscutils::rx::rx_main),
      install_loc: InstallLoc::DIR_USR_BIN,
      usage: std::include_str!("../usage/rx"),
    });
    #[cfg(feature = "script")]
    appy_mcappface.push(applet {
      name: "script",
      main: "script",
      entrypoint: Entrypoint::CStyle(crate::util_linux::script::script_main),
      install_loc: InstallLoc::DIR_USR_BIN,
      usage: std::include_str!("../usage/script"),
    });
    #[cfg(feature = "scriptreplay")]
    appy_mcappface.push(applet {
      name: "scriptreplay",
      main: "scriptreplay",
      entrypoint: Entrypoint::CStyle(crate::util_linux::scriptreplay::scriptreplay_main),
      install_loc: InstallLoc::DIR_BIN,
      usage: std::include_str!("../usage/scriptreplay"),
    });
    #[cfg(feature = "sed")]
    appy_mcappface.push(applet {
      name: "sed",
      main: "sed",
      entrypoint: Entrypoint::CStyle(scrate::editors::ed::ed_main),
      install_loc: InstallLoc::DIR_BIN,
      usage: std::include_str!("../usage/sed"),
    });
    #[cfg(feature = "sendmail")]
    appy_mcappface.push(applet {
      name: "sendmail",
      main: "sendmail",
      entrypoint: Entrypoint::CStyle(crate::mailutils::sendmail::sendmail_main),
      install_loc: InstallLoc::DIR_USR_SBIN,
      usage: std::include_str!("../usage/sendmail"),
    });
    #[cfg(feature = "seq")]
    appy_mcappface.push(applet {
      name: "seq",
      main: "seq",
      entrypoint: Entrypoint::CStyle(crate::coreutils::seq::seq_main),
      install_loc: InstallLoc::DIR_USR_BIN,
      usage: std::include_str!("../usage/seq"),
    });
    #[cfg(feature = "setarch")]
    appy_mcappface.push(applet {
      name: "setarch",
      main: "setarch",
      entrypoint: Entrypoint::CStyle(crate::util_linux::setarch::setarch_main),
      install_loc: InstallLoc::DIR_BIN,
      usage: std::include_str!("../usage/setarch"),
    });
    #[cfg(feature = "setconsole")]
    appy_mcappface.push(applet {
      name: "setconsole",
      main: "setconsole",
      entrypoint: Entrypoint::CStyle(crate::console_tools::setconsole::setconsole_main),
      install_loc: InstallLoc::DIR_SBIN,
      usage: std::include_str!("../usage/setconsole"),
    });
    #[cfg(feature = "setfattr")]
    appy_mcappface.push(applet {
      name: "setfattr",
      main: "setfattr",
      entrypoint: Entrypoint::CStyle(setfatcrate::coreutils::tr::tr_main),
      install_loc: InstallLoc::DIR_USR_BIN,
      usage: std::include_str!("../usage/setfattr"),
    });
    #[cfg(feature = "setfont")]
    appy_mcappface.push(applet {
      name: "setfont",
      main: "setfont",
      entrypoint: Entrypoint::CStyle(crate::console_tools::loadfont::setfont_main),
      install_loc: InstallLoc::DIR_USR_SBIN,
      usage: std::include_str!("../usage/setfont"),
    });
    #[cfg(feature = "setkeycodes")]
    appy_mcappface.push(applet {
      name: "setkeycodes",
      main: "setkeycodes",
      entrypoint: Entrypoint::CStyle(crate::console_tools::setkeycodes::setkeycodes_main),
      install_loc: InstallLoc::DIR_USR_BIN,
      usage: std::include_str!("../usage/setkeycodes"),
    });
    #[cfg(feature = "setlogcons")]
    appy_mcappface.push(applet {
      name: "setlogcons",
      main: "setlogcons",
      entrypoint: Entrypoint::CStyle(crate::console_tools::setlogcons::setlogcons_main),
      install_loc: InstallLoc::DIR_USR_SBIN,
      usage: std::include_str!("../usage/setlogcons"),
    });
    #[cfg(feature = "setpriv")]
    appy_mcappface.push(applet {
      name: "setpriv",
      main: "setpriv",
      entrypoint: Entrypoint::CStyle(crate::util_linux::setpriv::setpriv_main),
      install_loc: InstallLoc::DIR_BIN,
      usage: std::include_str!("../usage/setpriv"),
    });
    #[cfg(feature = "setserial")]
    appy_mcappface.push(applet {
      name: "setserial",
      main: "setserial",
      entrypoint: Entrypoint::CStyle(crate::miscutils::setserial::setserial_main),
      install_loc: InstallLoc::DIR_BIN,
      usage: std::include_str!("../usage/setserial"),
    });
    #[cfg(feature = "setsid")]
    appy_mcappface.push(applet {
      name: "setsid",
      main: "setsid",
      entrypoint: Entrypoint::CStyle(setscrate::coreutils::id::id_main),
      install_loc: InstallLoc::DIR_USR_BIN,
      usage: std::include_str!("../usage/setsid"),
    });
    #[cfg(feature = "setuidgid")]
    appy_mcappface.push(applet {
      name: "setuidgid",
      main: "chpst",
      entrypoint: Entrypoint::CStyle(crate::runit::chpst::chpst_main),
      install_loc: InstallLoc::DIR_USR_BIN,
      usage: std::include_str!("../usage/setuidgid"),
    });
    #[cfg(feature = "sh")]
    appy_mcappface.push(applet {
      name: "sh",
      main: "ash",
      entrypoint: Entrypoint::CStyle(crate::shell::ash::ash_main),
      install_loc: InstallLoc::DIR_BIN,
      usage: std::include_str!("../usage/sh"),
    });
    #[cfg(feature = "sha1sum")]
    appy_mcappface.push(applet {
      name: "sha1sum",
      main: "md5_sha1_sum",
      entrypoint: Entrypoint::CStyle(crate::coreutils::md5_sha1_sum::md5_sha1_crate::coreutils::sum::sum_main),
      install_loc: InstallLoc::DIR_USR_BIN,
      usage: std::include_str!("../usage/sha1sum"),
    });
    #[cfg(feature = "sha256sum")]
    appy_mcappface.push(applet {
      name: "sha256sum",
      main: "md5_sha1_sum",
      entrypoint: Entrypoint::CStyle(crate::coreutils::md5_sha1_sum::md5_sha1_crate::coreutils::sum::sum_main),
      install_loc: InstallLoc::DIR_USR_BIN,
      usage: std::include_str!("../usage/sha256sum"),
    });
    #[cfg(feature = "sha3sum")]
    appy_mcappface.push(applet {
      name: "sha3sum",
      main: "md5_sha1_sum",
      entrypoint: Entrypoint::CStyle(crate::coreutils::md5_sha1_sum::md5_sha1_crate::coreutils::sum::sum_main),
      install_loc: InstallLoc::DIR_USR_BIN,
      usage: std::include_str!("../usage/sha3sum"),
    });
    #[cfg(feature = "sha512sum")]
    appy_mcappface.push(applet {
      name: "sha512sum",
      main: "md5_sha1_sum",
      entrypoint: Entrypoint::CStyle(crate::coreutils::md5_sha1_sum::md5_sha1_crate::coreutils::sum::sum_main),
      install_loc: InstallLoc::DIR_USR_BIN,
      usage: std::include_str!("../usage/sha512sum"),
    });
    #[cfg(feature = "showkey")]
    appy_mcappface.push(applet {
      name: "showkey",
      main: "showkey",
      entrypoint: Entrypoint::CStyle(crate::console_tools::showkey::showkey_main),
      install_loc: InstallLoc::DIR_USR_BIN,
      usage: std::include_str!("../usage/showkey"),
    });
    #[cfg(feature = "shred")]
    appy_mcappface.push(applet {
      name: "shred",
      main: "shred",
      entrypoint: Entrypoint::CStyle(crate::coreutils::shred::shrcrate::editors::ed::ed_main),
      install_loc: InstallLoc::DIR_USR_BIN,
      usage: std::include_str!("../usage/shred"),
    });
    #[cfg(feature = "shuf")]
    appy_mcappface.push(applet {
      name: "shuf",
      main: "shuf",
      entrypoint: Entrypoint::CStyle(crate::coreutils::shuf::shuf_main),
      install_loc: InstallLoc::DIR_USR_BIN,
      usage: std::include_str!("../usage/shuf"),
    });
    #[cfg(feature = "slattach")]
    appy_mcappface.push(applet {
      name: "slattach",
      main: "slattach",
      entrypoint: Entrypoint::CStyle(crate::networking::slattach::slattach_main),
      install_loc: InstallLoc::DIR_SBIN,
      usage: std::include_str!("../usage/slattach"),
    });
    #[cfg(feature = "sleep")]
    appy_mcappface.push(applet {
      name: "sleep",
      main: "sleep",
      entrypoint: Entrypoint::CStyle(crate::coreutils::sleep::sleep_main),
      install_loc: InstallLoc::DIR_BIN,
      usage: std::include_str!("../usage/sleep"),
    });
    #[cfg(feature = "smemcap")]
    appy_mcappface.push(applet {
      name: "smemcap",
      main: "smemcap",
      entrypoint: Entrypoint::CStyle(crate::procps::smemcap::smemcap_main),
      install_loc: InstallLoc::DIR_USR_BIN,
      usage: std::include_str!("../usage/smemcap"),
    });
    #[cfg(feature = "softlimit")]
    appy_mcappface.push(applet {
      name: "softlimit",
      main: "chpst",
      entrypoint: Entrypoint::CStyle(crate::runit::chpst::chpst_main),
      install_loc: InstallLoc::DIR_USR_BIN,
      usage: std::include_str!("../usage/softlimit"),
    });
    #[cfg(feature = "sort")]
    appy_mcappface.push(applet {
      name: "sort",
      main: "sort",
      entrypoint: Entrypoint::CStyle(crate::coreutils::sort::sort_main),
      install_loc: InstallLoc::DIR_USR_BIN,
      usage: std::include_str!("../usage/sort"),
    });
    #[cfg(feature = "split")]
    appy_mcappface.push(applet {
      name: "split",
      main: "split",
      entrypoint: Entrypoint::CStyle(crate::coreutils::split::split_main),
      install_loc: InstallLoc::DIR_USR_BIN,
      usage: std::include_str!("../usage/split"),
    });
    #[cfg(feature = "ssl_client")]
    appy_mcappface.push(applet {
      name: "ssl_client",
      main: "ssl_client",
      entrypoint: Entrypoint::CStyle(crate::networking::ssl_client::ssl_client_main),
      install_loc: InstallLoc::DIR_USR_BIN,
      usage: std::include_str!("../usage/ssl_client"),
    });
    #[cfg(feature = "start-stop-daemon")]
    appy_mcappface.push(applet {
      name: "start-stop-daemon",
      main: "start_stop_daemon",
      entrypoint: Entrypoint::CStyle(crate::debianutils::start_stop_daemon::start_stop_daemon_main),
      install_loc: InstallLoc::DIR_SBIN,
      usage: std::include_str!("../usage/start-stop-daemon"),
    });
    #[cfg(feature = "stat")]
    appy_mcappface.push(applet {
      name: "stat",
      main: "stat",
      entrypoint: Entrypoint::CStyle(crate::coreutils::stat::stat_main),
      install_loc: InstallLoc::DIR_BIN,
      usage: std::include_str!("../usage/stat"),
    });
    #[cfg(feature = "strings")]
    appy_mcappface.push(applet {
      name: "strings",
      main: "strings",
      entrypoint: Entrypoint::CStyle(crate::miscutils::strings::strings_main),
      install_loc: InstallLoc::DIR_USR_BIN,
      usage: std::include_str!("../usage/strings"),
    });
    #[cfg(feature = "stty")]
    appy_mcappface.push(applet {
      name: "stty",
      main: "stty",
      entrypoint: Entrypoint::CStyle(crate::coreutils::stty::scrate::coreutils::tty::tty_main),
      install_loc: InstallLoc::DIR_BIN,
      usage: std::include_str!("../usage/stty"),
    });
    #[cfg(feature = "su")]
    appy_mcappface.push(applet {
      name: "su",
      main: "su",
      entrypoint: Entrypoint::CStyle(crate::loginutils::su::su_main),
      install_loc: InstallLoc::DIR_BIN,
      usage: std::include_str!("../usage/su"),
    });
    #[cfg(feature = "sulogin")]
    appy_mcappface.push(applet {
      name: "sulogin",
      main: "sulogin",
      entrypoint: Entrypoint::CStyle(sucrate::loginutils::login::login_main),
      install_loc: InstallLoc::DIR_SBIN,
      usage: std::include_str!("../usage/sulogin"),
    });
    #[cfg(feature = "sum")]
    appy_mcappface.push(applet {
      name: "sum",
      main: "sum",
      entrypoint: Entrypoint::CStyle(crate::coreutils::sum::sum_main),
      install_loc: InstallLoc::DIR_USR_BIN,
      usage: std::include_str!("../usage/sum"),
    });
    #[cfg(feature = "sv")]
    appy_mcappface.push(applet {
      name: "sv",
      main: "sv",
      entrypoint: Entrypoint::CStyle(crate::runit::sv::sv_main),
      install_loc: InstallLoc::DIR_USR_BIN,
      usage: std::include_str!("../usage/sv"),
    });
    #[cfg(feature = "svc")]
    appy_mcappface.push(applet {
      name: "svc",
      main: "svc",
      entrypoint: Entrypoint::CStyle(crate::runit::sv::svc_main),
      install_loc: InstallLoc::DIR_USR_BIN,
      usage: std::include_str!("../usage/svc"),
    });
    #[cfg(feature = "svlogd")]
    appy_mcappface.push(applet {
      name: "svlogd",
      main: "svlogd",
      entrypoint: Entrypoint::CStyle(crate::runit::svlogd::svlogd_main),
      install_loc: InstallLoc::DIR_USR_SBIN,
      usage: std::include_str!("../usage/svlogd"),
    });
    #[cfg(feature = "svok")]
    appy_mcappface.push(applet {
      name: "svok",
      main: "svok",
      entrypoint: Entrypoint::CStyle(crate::runit::sv::svok_main),
      install_loc: InstallLoc::DIR_USR_BIN,
      usage: std::include_str!("../usage/svok"),
    });
    #[cfg(feature = "swapoff")]
    appy_mcappface.push(applet {
      name: "swapoff",
      main: "swap_on_off",
      entrypoint: Entrypoint::CStyle(crate::util_linux::swaponoff::swap_on_off_main),
      install_loc: InstallLoc::DIR_SBIN,
      usage: std::include_str!("../usage/swapoff"),
    });
    #[cfg(feature = "swapon")]
    appy_mcappface.push(applet {
      name: "swapon",
      main: "swap_on_off",
      entrypoint: Entrypoint::CStyle(crate::util_linux::swaponoff::swap_on_off_main),
      install_loc: InstallLoc::DIR_SBIN,
      usage: std::include_str!("../usage/swapon"),
    });
    #[cfg(feature = "switch_root")]
    appy_mcappface.push(applet {
      name: "switch_root",
      main: "switch_root",
      entrypoint: Entrypoint::CStyle(crate::util_linux::switch_root::switch_root_main),
      install_loc: InstallLoc::DIR_SBIN,
      usage: std::include_str!("../usage/switch_root"),
    });
    #[cfg(feature = "sync")]
    appy_mcappface.push(applet {
      name: "sync",
      main: "sync",
      entrypoint: Entrypoint::CStyle(crate::coreutils::sync::sycrate::networking::nc::nc_main),
      install_loc: InstallLoc::DIR_BIN,
      usage: std::include_str!("../usage/sync"),
    });
    #[cfg(feature = "sysctl")]
    appy_mcappface.push(applet {
      name: "sysctl",
      main: "sysctl",
      entrypoint: Entrypoint::CStyle(crate::procps::sysctl::sysctl_main),
      install_loc: InstallLoc::DIR_SBIN,
      usage: std::include_str!("../usage/sysctl"),
    });
    #[cfg(feature = "syslogd")]
    appy_mcappface.push(applet {
      name: "syslogd",
      main: "syslogd",
      entrypoint: Entrypoint::CStyle(crate::sysklogd::syslogd_and_logger::syslogd_main),
      install_loc: InstallLoc::DIR_SBIN,
      usage: std::include_str!("../usage/syslogd"),
    });
    #[cfg(feature = "tac")]
    appy_mcappface.push(applet {
      name: "tac",
      main: "tac",
      entrypoint: Entrypoint::CStyle(crate::coreutils::tac::tac_main),
      install_loc: InstallLoc::DIR_USR_BIN,
      usage: std::include_str!("../usage/tac"),
    });
    #[cfg(feature = "tail")]
    appy_mcappface.push(applet {
      name: "tail",
      main: "tail",
      entrypoint: Entrypoint::CStyle(crate::coreutils::tail::tail_main),
      install_loc: InstallLoc::DIR_USR_BIN,
      usage: std::include_str!("../usage/tail"),
    });
    #[cfg(feature = "tar")]
    appy_mcappface.push(applet {
      name: "tar",
      main: "tar",
      entrypoint: Entrypoint::CStyle(crate::archival::tar::tar_main),
      install_loc: InstallLoc::DIR_BIN,
      usage: std::include_str!("../usage/tar"),
    });
    #[cfg(feature = "taskset")]
    appy_mcappface.push(applet {
      name: "taskset",
      main: "taskset",
      entrypoint: Entrypoint::CStyle(crate::util_linux::taskset::taskset_main),
      install_loc: InstallLoc::DIR_USR_BIN,
      usage: std::include_str!("../usage/taskset"),
    });
    #[cfg(feature = "tc")]
    appy_mcappface.push(applet {
      name: "tc",
      main: "tc",
      entrypoint: Entrypoint::CStyle(crate::networking::tc::tc_main),
      install_loc: InstallLoc::DIR_SBIN,
      usage: std::include_str!("../usage/tc"),
    });
    #[cfg(feature = "tcpsvd")]
    appy_mcappface.push(applet {
      name: "tcpsvd",
      main: "tcpudpsvd",
      entrypoint: Entrypoint::CStyle(crate::networking::tcpudp::tcpudpsvd_main),
      install_loc: InstallLoc::DIR_USR_BIN,
      usage: std::include_str!("../usage/tcpsvd"),
    });
    #[cfg(feature = "tee")]
    appy_mcappface.push(applet {
      name: "tee",
      main: "tee",
      entrypoint: Entrypoint::CStyle(crate::coreutils::tee::tee_main),
      install_loc: InstallLoc::DIR_USR_BIN,
      usage: std::include_str!("../usage/tee"),
    });
    #[cfg(feature = "telnet")]
    appy_mcappface.push(applet {
      name: "telnet",
      main: "telnet",
      entrypoint: Entrypoint::CStyle(crate::networking::telnet::telnet_main),
      install_loc: InstallLoc::DIR_USR_BIN,
      usage: std::include_str!("../usage/telnet"),
    });
    #[cfg(feature = "telnetd")]
    appy_mcappface.push(applet {
      name: "telnetd",
      main: "telnetd",
      entrypoint: Entrypoint::CStyle(crate::networking::telnetd::telnetd_main),
      install_loc: InstallLoc::DIR_USR_SBIN,
      usage: std::include_str!("../usage/telnetd"),
    });
    #[cfg(feature = "test")]
    appy_mcappface.push(applet {
      name: "test",
      main: "test",
      entrypoint: Entrypoint::CStyle(crate::coreutils::test::test_main),
      install_loc: InstallLoc::DIR_USR_BIN,
      usage: std::include_str!("../usage/test"),
    });
    #[cfg(feature = "tftp")]
    appy_mcappface.push(applet {
      name: "tftp",
      main: "tftp",
      entrypoint: Entrypoint::CStyle(crate::networking::tftp::tftp_main),
      install_loc: InstallLoc::DIR_USR_BIN,
      usage: std::include_str!("../usage/tftp"),
    });
    #[cfg(feature = "tftpd")]
    appy_mcappface.push(applet {
      name: "tftpd",
      main: "tftpd",
      entrypoint: Entrypoint::CStyle(tcrate::networking::ftpd::ftpd_main),
      install_loc: InstallLoc::DIR_USR_SBIN,
      usage: std::include_str!("../usage/tftpd"),
    });
    #[cfg(feature = "time")]
    appy_mcappface.push(applet {
      name: "time",
      main: "time",
      entrypoint: Entrypoint::CStyle(crate::miscutils::time::time_main),
      install_loc: InstallLoc::DIR_USR_BIN,
      usage: std::include_str!("../usage/time"),
    });
    #[cfg(feature = "timeout")]
    appy_mcappface.push(applet {
      name: "timeout",
      main: "timeout",
      entrypoint: Entrypoint::CStyle(crate::coreutils::timeout::timeout_main),
      install_loc: InstallLoc::DIR_USR_BIN,
      usage: std::include_str!("../usage/timeout"),
    });
    #[cfg(feature = "top")]
    appy_mcappface.push(applet {
      name: "top",
      main: "top",
      entrypoint: Entrypoint::CStyle(crate::procps::top::top_main),
      install_loc: InstallLoc::DIR_USR_BIN,
      usage: std::include_str!("../usage/top"),
    });
    #[cfg(feature = "touch")]
    appy_mcappface.push(applet {
      name: "touch",
      main: "touch",
      entrypoint: Entrypoint::CStyle(crate::coreutils::touch::touch_main),
      install_loc: InstallLoc::DIR_BIN,
      usage: std::include_str!("../usage/touch"),
    });
    #[cfg(feature = "tr")]
    appy_mcappface.push(applet {
      name: "tr",
      main: "tr",
      entrypoint: Entrypoint::CStyle(crate::coreutils::tr::tr_main),
      install_loc: InstallLoc::DIR_USR_BIN,
      usage: std::include_str!("../usage/tr"),
    });
    #[cfg(feature = "traceroute")]
    appy_mcappface.push(applet {
      name: "traceroute",
      main: "traceroute",
      entrypoint: Entrypoint::CStyle(tracecrate::networking::route::route_main),
      install_loc: InstallLoc::DIR_USR_BIN,
      usage: std::include_str!("../usage/traceroute"),
    });
    #[cfg(feature = "traceroute6")]
    appy_mcappface.push(applet {
      name: "traceroute6",
      main: "traceroute6",
      entrypoint: Entrypoint::CStyle(crate::networking::traceroute::traceroute6_main),
      install_loc: InstallLoc::DIR_USR_BIN,
      usage: std::include_str!("../usage/traceroute6"),
    });
    #[cfg(feature = "true")]
    appy_mcappface.push(applet {
      name: "true",
      main: "true",
      entrypoint: Entrypoint::SafeStyle(crate::coreutils::r#true::true_main),
      install_loc: InstallLoc::DIR_BIN,
      usage: std::include_str!("../usage/true"),
    });
    #[cfg(feature = "truncate")]
    appy_mcappface.push(applet {
      name: "truncate",
      main: "truncate",
      entrypoint: Entrypoint::CStyle(crate::coreutils::truncate::truncate_main),
      install_loc: InstallLoc::DIR_USR_BIN,
      usage: std::include_str!("../usage/truncate"),
    });
    #[cfg(feature = "ts")]
    appy_mcappface.push(applet {
      name: "ts",
      main: "ts",
      entrypoint: Entrypoint::CStyle(crate::miscutils::ts::ts_main),
      install_loc: InstallLoc::DIR_USR_BIN,
      usage: std::include_str!("../usage/ts"),
    });
    #[cfg(feature = "tty")]
    appy_mcappface.push(applet {
      name: "tty",
      main: "tty",
      entrypoint: Entrypoint::CStyle(crate::coreutils::tty::tty_main),
      install_loc: InstallLoc::DIR_USR_BIN,
      usage: std::include_str!("../usage/tty"),
    });
    #[cfg(feature = "ttysize")]
    appy_mcappface.push(applet {
      name: "ttysize",
      main: "ttysize",
      entrypoint: Entrypoint::CStyle(crate::miscutils::ttysize::ttysize_main),
      install_loc: InstallLoc::DIR_USR_BIN,
      usage: std::include_str!("../usage/ttysize"),
    });
    #[cfg(feature = "tunctl")]
    appy_mcappface.push(applet {
      name: "tunctl",
      main: "tunctl",
      entrypoint: Entrypoint::CStyle(crate::networking::tunctl::tunctl_main),
      install_loc: InstallLoc::DIR_SBIN,
      usage: std::include_str!("../usage/tunctl"),
    });
    #[cfg(feature = "ubiattach")]
    appy_mcappface.push(applet {
      name: "ubiattach",
      main: "ubi_tools",
      entrypoint: Entrypoint::CStyle(ubi_toocrate::coreutils::ls::ls_main),
      install_loc: InstallLoc::DIR_USR_SBIN,
      usage: std::include_str!("../usage/ubiattach"),
    });
    #[cfg(feature = "ubidetach")]
    appy_mcappface.push(applet {
      name: "ubidetach",
      main: "ubi_tools",
      entrypoint: Entrypoint::CStyle(ubi_toocrate::coreutils::ls::ls_main),
      install_loc: InstallLoc::DIR_USR_SBIN,
      usage: std::include_str!("../usage/ubidetach"),
    });
    #[cfg(feature = "ubimkvol")]
    appy_mcappface.push(applet {
      name: "ubimkvol",
      main: "ubi_tools",
      entrypoint: Entrypoint::CStyle(ubi_toocrate::coreutils::ls::ls_main),
      install_loc: InstallLoc::DIR_USR_SBIN,
      usage: std::include_str!("../usage/ubimkvol"),
    });
    #[cfg(feature = "ubirename")]
    appy_mcappface.push(applet {
      name: "ubirename",
      main: "ubirename",
      entrypoint: Entrypoint::CStyle(crate::miscutils::ubirename::ubirename_main),
      install_loc: InstallLoc::DIR_USR_SBIN,
      usage: std::include_str!("../usage/ubirename"),
    });
    #[cfg(feature = "ubirmvol")]
    appy_mcappface.push(applet {
      name: "ubirmvol",
      main: "ubi_tools",
      entrypoint: Entrypoint::CStyle(ubi_toocrate::coreutils::ls::ls_main),
      install_loc: InstallLoc::DIR_USR_SBIN,
      usage: std::include_str!("../usage/ubirmvol"),
    });
    #[cfg(feature = "ubirsvol")]
    appy_mcappface.push(applet {
      name: "ubirsvol",
      main: "ubi_tools",
      entrypoint: Entrypoint::CStyle(ubi_toocrate::coreutils::ls::ls_main),
      install_loc: InstallLoc::DIR_USR_SBIN,
      usage: std::include_str!("../usage/ubirsvol"),
    });
    #[cfg(feature = "ubiupdatevol")]
    appy_mcappface.push(applet {
      name: "ubiupdatevol",
      main: "ubi_tools",
      entrypoint: Entrypoint::CStyle(ubi_toocrate::coreutils::ls::ls_main),
      install_loc: InstallLoc::DIR_USR_SBIN,
      usage: std::include_str!("../usage/ubiupdatevol"),
    });
    #[cfg(feature = "udhcpc")]
    appy_mcappface.push(applet {
      name: "udhcpc",
      main: "udhcpc",
      entrypoint: Entrypoint::CStyle(crate::networking::udhcp::dhcpc::udhcpc_main),
      install_loc: InstallLoc::DIR_SBIN,
      usage: std::include_str!("../usage/udhcpc"),
    });
    #[cfg(feature = "udhcpc6")]
    appy_mcappface.push(applet {
      name: "udhcpc6",
      main: "udhcpc6",
      entrypoint: Entrypoint::CStyle(crate::networking::udhcp::d6_dhcpc::udhcpc6_main),
      install_loc: InstallLoc::DIR_USR_BIN,
      usage: std::include_str!("../usage/udhcpc6"),
    });
    #[cfg(feature = "udhcpd")]
    appy_mcappface.push(applet {
      name: "udhcpd",
      main: "udhcpd",
      entrypoint: Entrypoint::CStyle(crate::networking::udhcp::dhcpd::udhcpd_main),
      install_loc: InstallLoc::DIR_USR_SBIN,
      usage: std::include_str!("../usage/udhcpd"),
    });
    #[cfg(feature = "udpsvd")]
    appy_mcappface.push(applet {
      name: "udpsvd",
      main: "tcpudpsvd",
      entrypoint: Entrypoint::CStyle(crate::networking::tcpudp::tcpudpsvd_main),
      install_loc: InstallLoc::DIR_USR_BIN,
      usage: std::include_str!("../usage/udpsvd"),
    });
    #[cfg(feature = "uevent")]
    appy_mcappface.push(applet {
      name: "uevent",
      main: "uevent",
      entrypoint: Entrypoint::CStyle(crate::util_linux::uevent::uevent_main),
      install_loc: InstallLoc::DIR_SBIN,
      usage: std::include_str!("../usage/uevent"),
    });
    #[cfg(feature = "umount")]
    appy_mcappface.push(applet {
      name: "umount",
      main: "umount",
      entrypoint: Entrypoint::CStyle(ucrate::util_linux::mount::mount_main),
      install_loc: InstallLoc::DIR_BIN,
      usage: std::include_str!("../usage/umount"),
    });
    #[cfg(feature = "uname")]
    appy_mcappface.push(applet {
      name: "uname",
      main: "uname",
      entrypoint: Entrypoint::CStyle(crate::coreutils::uname::uname_main),
      install_loc: InstallLoc::DIR_BIN,
      usage: std::include_str!("../usage/uname"),
    });
    #[cfg(feature = "unexpand")]
    appy_mcappface.push(applet {
      name: "unexpand",
      main: "expand",
      entrypoint: Entrypoint::CStyle(crate::coreutils::expand::expand_main),
      install_loc: InstallLoc::DIR_USR_BIN,
      usage: std::include_str!("../usage/unexpand"),
    });
    #[cfg(feature = "uniq")]
    appy_mcappface.push(applet {
      name: "uniq",
      main: "uniq",
      entrypoint: Entrypoint::CStyle(crate::coreutils::uniq::uniq_main),
      install_loc: InstallLoc::DIR_USR_BIN,
      usage: std::include_str!("../usage/uniq"),
    });
    #[cfg(feature = "unix2dos")]
    appy_mcappface.push(applet {
      name: "unix2dos",
      main: "dos2unix",
      entrypoint: Entrypoint::CStyle(crate::coreutils::dos2unix::dos2unix_main),
      install_loc: InstallLoc::DIR_USR_BIN,
      usage: std::include_str!("../usage/unix2dos"),
    });
    #[cfg(feature = "unlink")]
    appy_mcappface.push(applet {
      name: "unlink",
      main: "unlink",
      entrypoint: Entrypoint::CStyle(uncrate::coreutils::link::link_main),
      install_loc: InstallLoc::DIR_USR_BIN,
      usage: std::include_str!("../usage/unlink"),
    });
    #[cfg(feature = "unlzma")]
    appy_mcappface.push(applet {
      name: "unlzma",
      main: "unlzma",
      entrypoint: Entrypoint::CStyle(crate::archival::bbunzip::unlzma_main),
      install_loc: InstallLoc::DIR_USR_BIN,
      usage: std::include_str!("../usage/unlzma"),
    });
    #[cfg(feature = "unshare")]
    appy_mcappface.push(applet {
      name: "unshare",
      main: "unshare",
      entrypoint: Entrypoint::CStyle(crate::util_linux::unshare::unshare_main),
      install_loc: InstallLoc::DIR_USR_BIN,
      usage: std::include_str!("../usage/unshare"),
    });
    #[cfg(feature = "unxz")]
    appy_mcappface.push(applet {
      name: "unxz",
      main: "unxz",
      entrypoint: Entrypoint::CStyle(crate::archival::bbunzip::unxz_main),
      install_loc: InstallLoc::DIR_USR_BIN,
      usage: std::include_str!("../usage/unxz"),
    });
    #[cfg(feature = "unzip")]
    appy_mcappface.push(applet {
      name: "unzip",
      main: "unzip",
      entrypoint: Entrypoint::CStyle(crate::archival::unzip::unzcrate::networking::ip::ip_main),
      install_loc: InstallLoc::DIR_USR_BIN,
      usage: std::include_str!("../usage/unzip"),
    });
    #[cfg(feature = "uptime")]
    appy_mcappface.push(applet {
      name: "uptime",
      main: "uptime",
      entrypoint: Entrypoint::CStyle(upcrate::miscutils::time::time_main),
      install_loc: InstallLoc::DIR_USR_BIN,
      usage: std::include_str!("../usage/uptime"),
    });
    #[cfg(feature = "users")]
    appy_mcappface.push(applet {
      name: "users",
      main: "who",
      entrypoint: Entrypoint::CStyle(crate::coreutils::who::who_main),
      install_loc: InstallLoc::DIR_USR_BIN,
      usage: std::include_str!("../usage/users"),
    });
    #[cfg(feature = "usleep")]
    appy_mcappface.push(applet {
      name: "usleep",
      main: "usleep",
      entrypoint: Entrypoint::CStyle(ucrate::coreutils::sleep::sleep_main),
      install_loc: InstallLoc::DIR_BIN,
      usage: std::include_str!("../usage/usleep"),
    });
    #[cfg(feature = "uudecode")]
    appy_mcappface.push(applet {
      name: "uudecode",
      main: "uudecode",
      entrypoint: Entrypoint::CStyle(crate::coreutils::uudecode::uudecode_main),
      install_loc: InstallLoc::DIR_USR_BIN,
      usage: std::include_str!("../usage/uudecode"),
    });
    #[cfg(feature = "uuencode")]
    appy_mcappface.push(applet {
      name: "uuencode",
      main: "uuencode",
      entrypoint: Entrypoint::CStyle(crate::coreutils::uuencode::uuencode_main),
      install_loc: InstallLoc::DIR_USR_BIN,
      usage: std::include_str!("../usage/uuencode"),
    });
    #[cfg(feature = "vconfig")]
    appy_mcappface.push(applet {
      name: "vconfig",
      main: "vconfig",
      entrypoint: Entrypoint::CStyle(crate::networking::vconfig::vconfig_main),
      install_loc: InstallLoc::DIR_SBIN,
      usage: std::include_str!("../usage/vconfig"),
    });
    #[cfg(feature = "vi")]
    appy_mcappface.push(applet {
      name: "vi",
      main: "vi",
      entrypoint: Entrypoint::CStyle(crate::editors::vi::vi_main),
      install_loc: InstallLoc::DIR_BIN,
      usage: std::include_str!("../usage/vi"),
    });
    #[cfg(feature = "vlock")]
    appy_mcappface.push(applet {
      name: "vlock",
      main: "vlock",
      entrypoint: Entrypoint::CStyle(crate::loginutils::vlock::vlock_main),
      install_loc: InstallLoc::DIR_USR_BIN,
      usage: std::include_str!("../usage/vlock"),
    });
    #[cfg(feature = "volname")]
    appy_mcappface.push(applet {
      name: "volname",
      main: "volname",
      entrypoint: Entrypoint::CStyle(crate::miscutils::volname::volname_main),
      install_loc: InstallLoc::DIR_USR_BIN,
      usage: std::include_str!("../usage/volname"),
    });
    #[cfg(feature = "w")]
    appy_mcappface.push(applet {
      name: "w",
      main: "who",
      entrypoint: Entrypoint::CStyle(crate::coreutils::who::who_main),
      install_loc: InstallLoc::DIR_USR_BIN,
      usage: std::include_str!("../usage/w"),
    });
    #[cfg(feature = "wall")]
    appy_mcappface.push(applet {
      name: "wall",
      main: "wall",
      entrypoint: Entrypoint::CStyle(crate::util_linux::wall::wall_main),
      install_loc: InstallLoc::DIR_USR_BIN,
      usage: std::include_str!("../usage/wall"),
    });
    #[cfg(feature = "watch")]
    appy_mcappface.push(applet {
      name: "watch",
      main: "watch",
      entrypoint: Entrypoint::CStyle(crate::procps::watch::watch_main),
      install_loc: InstallLoc::DIR_BIN,
      usage: std::include_str!("../usage/watch"),
    });
    #[cfg(feature = "watchdog")]
    appy_mcappface.push(applet {
      name: "watchdog",
      main: "watchdog",
      entrypoint: Entrypoint::CStyle(crate::miscutils::watchdog::watchdog_main),
      install_loc: InstallLoc::DIR_SBIN,
      usage: std::include_str!("../usage/watchdog"),
    });
    #[cfg(feature = "wc")]
    appy_mcappface.push(applet {
      name: "wc",
      main: "wc",
      entrypoint: Entrypoint::CStyle(crate::coreutils::wc::wc_main),
      install_loc: InstallLoc::DIR_USR_BIN,
      usage: std::include_str!("../usage/wc"),
    });
    #[cfg(feature = "wget")]
    appy_mcappface.push(applet {
      name: "wget",
      main: "wget",
      entrypoint: Entrypoint::CStyle(crate::networking::wget::wget_main),
      install_loc: InstallLoc::DIR_USR_BIN,
      usage: std::include_str!("../usage/wget"),
    });
    #[cfg(feature = "which")]
    appy_mcappface.push(applet {
      name: "which",
      main: "which",
      entrypoint: Entrypoint::CStyle(crate::debianutils::which::which_main),
      install_loc: InstallLoc::DIR_USR_BIN,
      usage: std::include_str!("../usage/which"),
    });
    #[cfg(feature = "who")]
    appy_mcappface.push(applet {
      name: "who",
      main: "who",
      entrypoint: Entrypoint::CStyle(crate::coreutils::who::who_main),
      install_loc: InstallLoc::DIR_USR_BIN,
      usage: std::include_str!("../usage/who"),
    });
    #[cfg(feature = "whoami")]
    appy_mcappface.push(applet {
      name: "whoami",
      main: "whoami",
      entrypoint: Entrypoint::CStyle(crate::coreutils::whoami::whoami_main),
      install_loc: InstallLoc::DIR_USR_BIN,
      usage: std::include_str!("../usage/whoami"),
    });
    #[cfg(feature = "whois")]
    appy_mcappface.push(applet {
      name: "whois",
      main: "whois",
      entrypoint: Entrypoint::CStyle(crate::networking::whois::whois_main),
      install_loc: InstallLoc::DIR_USR_BIN,
      usage: std::include_str!("../usage/whois"),
    });
    #[cfg(feature = "xargs")]
    appy_mcappface.push(applet {
      name: "xargs",
      main: "xargs",
      entrypoint: Entrypoint::CStyle(crate::findutils::xargs::xargs_main),
      install_loc: InstallLoc::DIR_USR_BIN,
      usage: std::include_str!("../usage/xargs"),
    });
    #[cfg(feature = "xxd")]
    appy_mcappface.push(applet {
      name: "xxd",
      main: "xxd",
      entrypoint: Entrypoint::CStyle(crate::util_linux::hexdump_xxd::xxd_main),
      install_loc: InstallLoc::DIR_USR_BIN,
      usage: std::include_str!("../usage/xxd"),
    });
    #[cfg(feature = "xz")]
    appy_mcappface.push(applet {
      name: "xz",
      main: "unxz",
      entrypoint: Entrypoint::CStyle(crate::archival::bbunzip::unxz_main),
      install_loc: InstallLoc::DIR_USR_BIN,
      usage: std::include_str!("../usage/xz"),
    });
    #[cfg(feature = "xzcat")]
    appy_mcappface.push(applet {
      name: "xzcat",
      main: "unxz",
      entrypoint: Entrypoint::CStyle(crate::archival::bbunzip::unxz_main),
      install_loc: InstallLoc::DIR_USR_BIN,
      usage: std::include_str!("../usage/xzcat"),
    });
    #[cfg(feature = "yes")]
    appy_mcappface.push(applet {
      name: "yes",
      main: "yes",
      entrypoint: Entrypoint::SafeStyle(crate::coreutils::yes::yes_main),
      install_loc: InstallLoc::DIR_USR_BIN,
      usage: std::include_str!("../usage/yes"),
    });
    #[cfg(feature = "zcat")]
    appy_mcappface.push(applet {
      name: "zcat",
      main: "gunzip",
      entrypoint: Entrypoint::CStyle(crate::archival::bbunzip::gcrate::archival::unzip::unzcrate::networking::ip::ip_main),
      install_loc: InstallLoc::DIR_BIN,
      usage: std::include_str!("../usage/zcat"),
    });
    #[cfg(feature = "zcip")]
    appy_mcappface.push(applet {
      name: "zcip",
      main: "zcip",
      entrypoint: Entrypoint::CStyle(zccrate::networking::ip::ip_main),
      install_loc: InstallLoc::DIR_SBIN,
      usage: std::include_str!("../usage/zcip"),
    });

    // See https://stackoverflow.com/questions/51272571/how-do-i-check-if-a-slice-is-sorted.
    // We use < and not <= since names should be unique.
    assert!(appy_mcappface.windows(2).all(|w| w[0].name < w[1].name));

    appy_mcappface
  };
}
