// This file is part of syscall-alt. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/syscall-alt/master/COPYRIGHT. No part of syscall-alt, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2016 The developers of syscall-alt. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/syscall-alt/master/COPYRIGHT.


use ::SyscallNumber;


// Originates from https://github.com/DragonFlyBSD/DragonFlyBSD/blob/master/sys/sys/syscall.h (as of commit 6d2444c on 19th June 2016)
pub const SYS_accept4: SyscallNumber = 541;
pub const SYS_accept: SyscallNumber = 30;
pub const SYS_access: SyscallNumber = 33;
pub const SYS_acct: SyscallNumber = 51;
pub const SYS_adjtime: SyscallNumber = 140;
pub const SYS_aio_cancel: SyscallNumber = 316;
pub const SYS_aio_error: SyscallNumber = 317;
pub const SYS_aio_read: SyscallNumber = 318;
pub const SYS_aio_return: SyscallNumber = 314;
pub const SYS_aio_suspend: SyscallNumber = 315;
pub const SYS_aio_waitcomplete: SyscallNumber = 359;
pub const SYS_aio_write: SyscallNumber = 319;
pub const SYS_bind: SyscallNumber = 104;
pub const SYS_break: SyscallNumber = 17;
pub const SYS_chdir: SyscallNumber = 12;
pub const SYS_chflags: SyscallNumber = 34;
pub const SYS_chflagsat: SyscallNumber = 537;
pub const SYS_chmod: SyscallNumber = 15;
pub const SYS_chown: SyscallNumber = 16;
pub const SYS_chroot: SyscallNumber = 61;
pub const SYS_chroot_kernel: SyscallNumber = 522;
pub const SYS_clock_getres: SyscallNumber = 234;
pub const SYS_clock_gettime: SyscallNumber = 232;
pub const SYS_clock_settime: SyscallNumber = 233;
pub const SYS_close: SyscallNumber = 6;
pub const SYS_closefrom: SyscallNumber = 474;
pub const SYS_connect: SyscallNumber = 98;
pub const SYS_dup2: SyscallNumber = 90;
pub const SYS_dup: SyscallNumber = 41;
pub const SYS_eaccess: SyscallNumber = 532;
pub const SYS_execve: SyscallNumber = 59;
pub const SYS_exec_sys_register: SyscallNumber = 465;
pub const SYS_exec_sys_unregister: SyscallNumber = 466;
pub const SYS_exit: SyscallNumber = 1;
pub const SYS_extaccept: SyscallNumber = 482;
pub const SYS_extattrctl: SyscallNumber = 355;
pub const SYS_extattr_delete_file: SyscallNumber = 358;
pub const SYS_extattr_get_file: SyscallNumber = 357;
pub const SYS_extattr_set_file: SyscallNumber = 356;
pub const SYS_extconnect: SyscallNumber = 483;
pub const SYS_extexit: SyscallNumber = 494;
pub const SYS_extpread: SyscallNumber = 173;
pub const SYS_extpreadv: SyscallNumber = 289;
pub const SYS_extpwrite: SyscallNumber = 174;
pub const SYS_extpwritev: SyscallNumber = 290;
pub const SYS_faccessat: SyscallNumber = 509;
pub const SYS_fchdir: SyscallNumber = 13;
pub const SYS_fchflags: SyscallNumber = 35;
pub const SYS_fchmod: SyscallNumber = 124;
pub const SYS_fchmodat: SyscallNumber = 506;
pub const SYS_fchown: SyscallNumber = 123;
pub const SYS_fchownat: SyscallNumber = 507;
pub const SYS_fcntl: SyscallNumber = 92;
pub const SYS_fhopen: SyscallNumber = 298;
pub const SYS_fhstat: SyscallNumber = 478;
pub const SYS_fhstatfs: SyscallNumber = 297;
pub const SYS_fhstatvfs: SyscallNumber = 502;
pub const SYS_flock: SyscallNumber = 131;
pub const SYS_fork: SyscallNumber = 2;
pub const SYS_fpathconf: SyscallNumber = 192;
pub const SYS_fstat: SyscallNumber = 476;
pub const SYS_fstatat: SyscallNumber = 505;
pub const SYS_fstatfs: SyscallNumber = 158;
pub const SYS_fstatvfs: SyscallNumber = 501;
pub const SYS_fsync: SyscallNumber = 95;
pub const SYS_ftruncate: SyscallNumber = 201;
pub const SYS_futimens: SyscallNumber = 540;
pub const SYS_futimes: SyscallNumber = 206;
pub const SYS_getdents: SyscallNumber = 480;
pub const SYS_getdirentries: SyscallNumber = 479;
pub const SYS_getdomainname: SyscallNumber = 162;
pub const SYS_getdtablesize: SyscallNumber = 89;
pub const SYS_getegid: SyscallNumber = 43;
pub const SYS_geteuid: SyscallNumber = 25;
pub const SYS_getfh: SyscallNumber = 161;
pub const SYS_getfsstat: SyscallNumber = 18;
pub const SYS_getgid: SyscallNumber = 47;
pub const SYS_getgroups: SyscallNumber = 79;
pub const SYS_getitimer: SyscallNumber = 86;
pub const SYS_getlogin: SyscallNumber = 49;
pub const SYS_getpeername: SyscallNumber = 31;
pub const SYS_getpgid: SyscallNumber = 207;
pub const SYS_getpgrp: SyscallNumber = 81;
pub const SYS_getpid: SyscallNumber = 20;
pub const SYS_getppid: SyscallNumber = 39;
pub const SYS_getpriority: SyscallNumber = 100;
pub const SYS_getresgid: SyscallNumber = 361;
pub const SYS_getresuid: SyscallNumber = 360;
pub const SYS_getrlimit: SyscallNumber = 194;
pub const SYS_getrusage: SyscallNumber = 117;
pub const SYS_getsid: SyscallNumber = 310;
pub const SYS_getsockname: SyscallNumber = 32;
pub const SYS_getsockopt: SyscallNumber = 118;
pub const SYS_gettimeofday: SyscallNumber = 116;
pub const SYS_getuid: SyscallNumber = 24;
pub const SYS_getvfsstat: SyscallNumber = 503;
pub const SYS_get_tls_area: SyscallNumber = 473;
pub const SYS_ioctl: SyscallNumber = 54;
pub const SYS_ioprio_get: SyscallNumber = 521;
pub const SYS_ioprio_set: SyscallNumber = 520;
pub const SYS_issetugid: SyscallNumber = 253;
pub const SYS_jail: SyscallNumber = 338;
pub const SYS_jail_attach: SyscallNumber = 471;
pub const SYS_kenv: SyscallNumber = 390;
pub const SYS_kevent: SyscallNumber = 363;
pub const SYS_kill: SyscallNumber = 37;
pub const SYS_kldfind: SyscallNumber = 306;
pub const SYS_kldfirstmod: SyscallNumber = 309;
pub const SYS_kldload: SyscallNumber = 304;
pub const SYS_kldnext: SyscallNumber = 307;
pub const SYS_kldstat: SyscallNumber = 308;
pub const SYS_kldsym: SyscallNumber = 337;
pub const SYS_kldunload: SyscallNumber = 305;
pub const SYS_kqueue: SyscallNumber = 362;
pub const SYS_ktrace: SyscallNumber = 45;
pub const SYS_lchflags: SyscallNumber = 391;
pub const SYS_lchmod: SyscallNumber = 274;
pub const SYS_lchown: SyscallNumber = 254;
pub const SYS_link: SyscallNumber = 9;
pub const SYS_linkat: SyscallNumber = 531;
pub const SYS_lio_listio: SyscallNumber = 320;
pub const SYS_listen: SyscallNumber = 106;
pub const SYS_lpathconf: SyscallNumber = 533;
pub const SYS_lseek: SyscallNumber = 199;
pub const SYS_lstat: SyscallNumber = 477;
pub const SYS_lutimes: SyscallNumber = 276;
pub const SYS_lwp_create: SyscallNumber = 495;
pub const SYS_lwp_gettid: SyscallNumber = 496;
pub const SYS_lwp_kill: SyscallNumber = 497;
pub const SYS_lwp_rtprio: SyscallNumber = 498;
pub const SYS_lwp_setname: SyscallNumber = 542;
pub const SYS_madvise: SyscallNumber = 75;
pub const SYS_mcontrol: SyscallNumber = 485;
pub const SYS_mincore: SyscallNumber = 78;
pub const SYS_minherit: SyscallNumber = 250;
pub const SYS_mkdir: SyscallNumber = 136;
pub const SYS_mkdirat: SyscallNumber = 524;
pub const SYS_mkfifo: SyscallNumber = 132;
pub const SYS_mkfifoat: SyscallNumber = 525;
pub const SYS_mknod: SyscallNumber = 14;
pub const SYS_mknodat: SyscallNumber = 526;
pub const SYS_mlock: SyscallNumber = 203;
pub const SYS_mlockall: SyscallNumber = 324;
pub const SYS_mmap: SyscallNumber = 197;
pub const SYS_modfind: SyscallNumber = 303;
pub const SYS_modfnext: SyscallNumber = 302;
pub const SYS_modnext: SyscallNumber = 300;
pub const SYS_modstat: SyscallNumber = 301;
pub const SYS_mount: SyscallNumber = 21;
pub const SYS_mountctl: SyscallNumber = 468;
pub const SYS_mprotect: SyscallNumber = 74;
pub const SYS_mq_close: SyscallNumber = 511;
pub const SYS_mq_getattr: SyscallNumber = 513;
pub const SYS_mq_notify: SyscallNumber = 515;
pub const SYS_mq_open: SyscallNumber = 510;
pub const SYS_mq_receive: SyscallNumber = 517;
pub const SYS_mq_send: SyscallNumber = 516;
pub const SYS_mq_setattr: SyscallNumber = 514;
pub const SYS_mq_timedreceive: SyscallNumber = 519;
pub const SYS_mq_timedsend: SyscallNumber = 518;
pub const SYS_mq_unlink: SyscallNumber = 512;
pub const SYS_msgctl: SyscallNumber = 224;
pub const SYS_msgget: SyscallNumber = 225;
pub const SYS_msgrcv: SyscallNumber = 227;
pub const SYS_msgsnd: SyscallNumber = 226;
pub const SYS_msync: SyscallNumber = 65;
pub const SYS_munlock: SyscallNumber = 204;
pub const SYS_munlockall: SyscallNumber = 325;
pub const SYS_munmap: SyscallNumber = 73;
pub const SYS_nanosleep: SyscallNumber = 240;
pub const SYS_netbsd_lchown: SyscallNumber = 275;
pub const SYS_netbsd_msync: SyscallNumber = 277;
pub const SYS_nfssvc: SyscallNumber = 155;
pub const SYS_ntp_adjtime: SyscallNumber = 176;
pub const SYS_open: SyscallNumber = 5;
pub const SYS_openat: SyscallNumber = 504;
pub const SYS_openbsd_poll: SyscallNumber = 252;
pub const SYS_pathconf: SyscallNumber = 191;
pub const SYS_pipe2: SyscallNumber = 538;
pub const SYS_pipe: SyscallNumber = 42;
pub const SYS_poll: SyscallNumber = 209;
pub const SYS_ppoll: SyscallNumber = 543;
pub const SYS_procctl: SyscallNumber = 536;
pub const SYS_profil: SyscallNumber = 44;
pub const SYS_pselect: SyscallNumber = 499;
pub const SYS_ptrace: SyscallNumber = 26;
pub const SYS_quotactl: SyscallNumber = 148;
pub const SYS_read: SyscallNumber = 3;
pub const SYS_readlink: SyscallNumber = 58;
pub const SYS_readlinkat: SyscallNumber = 527;
pub const SYS_readv: SyscallNumber = 120;
pub const SYS_reboot: SyscallNumber = 55;
pub const SYS_recvfrom: SyscallNumber = 29;
pub const SYS_recvmsg: SyscallNumber = 27;
pub const SYS_rename: SyscallNumber = 128;
pub const SYS_renameat: SyscallNumber = 523;
pub const SYS_revoke: SyscallNumber = 56;
pub const SYS_rfork: SyscallNumber = 251;
pub const SYS_rmdir: SyscallNumber = 137;
pub const SYS_rtprio: SyscallNumber = 166;
pub const SYS_sbrk: SyscallNumber = 69;
pub const SYS_sched_getparam: SyscallNumber = 328;
pub const SYS_sched_getscheduler: SyscallNumber = 330;
pub const SYS_sched_get_priority_max: SyscallNumber = 332;
pub const SYS_sched_get_priority_min: SyscallNumber = 333;
pub const SYS_sched_rr_get_interval: SyscallNumber = 334;
pub const SYS_sched_setparam: SyscallNumber = 327;
pub const SYS_sched_setscheduler: SyscallNumber = 329;
pub const SYS_sched_yield: SyscallNumber = 331;
pub const SYS_select: SyscallNumber = 93;
pub const SYS_semget: SyscallNumber = 221;
pub const SYS_semop: SyscallNumber = 222;
pub const SYS_sendfile: SyscallNumber = 393;
pub const SYS_sendmsg: SyscallNumber = 28;
pub const SYS_sendto: SyscallNumber = 133;
pub const SYS_setdomainname: SyscallNumber = 163;
pub const SYS_setegid: SyscallNumber = 182;
pub const SYS_seteuid: SyscallNumber = 183;
pub const SYS_setgid: SyscallNumber = 181;
pub const SYS_setgroups: SyscallNumber = 80;
pub const SYS_setitimer: SyscallNumber = 83;
pub const SYS_setlogin: SyscallNumber = 50;
pub const SYS_setpgid: SyscallNumber = 82;
pub const SYS_setpriority: SyscallNumber = 96;
pub const SYS_setregid: SyscallNumber = 127;
pub const SYS_setresgid: SyscallNumber = 312;
pub const SYS_setresuid: SyscallNumber = 311;
pub const SYS_setreuid: SyscallNumber = 126;
pub const SYS_setrlimit: SyscallNumber = 195;
pub const SYS_setsid: SyscallNumber = 147;
pub const SYS_setsockopt: SyscallNumber = 105;
pub const SYS_settimeofday: SyscallNumber = 122;
pub const SYS_setuid: SyscallNumber = 23;
pub const SYS_set_tls_area: SyscallNumber = 472;
pub const SYS_shmat: SyscallNumber = 228;
pub const SYS_shmctl: SyscallNumber = 229;
pub const SYS_shmdt: SyscallNumber = 230;
pub const SYS_shmget: SyscallNumber = 231;
pub const SYS_shutdown: SyscallNumber = 134;
pub const SYS_sigaction: SyscallNumber = 342;
pub const SYS_sigaltstack: SyscallNumber = 53;
pub const SYS_sigpending: SyscallNumber = 343;
pub const SYS_sigprocmask: SyscallNumber = 340;
pub const SYS_sigreturn: SyscallNumber = 344;
pub const SYS_sigsuspend: SyscallNumber = 341;
pub const SYS_sigtimedwait: SyscallNumber = 345;
pub const SYS_sigwaitinfo: SyscallNumber = 346;
pub const SYS_socket: SyscallNumber = 97;
pub const SYS_socketpair: SyscallNumber = 135;
pub const SYS_sstk: SyscallNumber = 70;
pub const SYS_stat: SyscallNumber = 475;
pub const SYS_statfs: SyscallNumber = 157;
pub const SYS_statvfs: SyscallNumber = 500;
pub const SYS_swapoff: SyscallNumber = 529;
pub const SYS_swapon: SyscallNumber = 85;
pub const SYS_symlink: SyscallNumber = 57;
pub const SYS_symlinkat: SyscallNumber = 528;
pub const SYS_sync: SyscallNumber = 36;
pub const SYS_sysarch: SyscallNumber = 165;
pub const SYS_syscall: SyscallNumber = 0;
pub const SYS_sys_checkpoint: SyscallNumber = 467;
pub const SYS_truncate: SyscallNumber = 200;
pub const SYS_umask: SyscallNumber = 60;
pub const SYS_umtx_sleep: SyscallNumber = 469;
pub const SYS_umtx_wakeup: SyscallNumber = 470;
pub const SYS_uname: SyscallNumber = 164;
pub const SYS_undelete: SyscallNumber = 205;
pub const SYS_unlink: SyscallNumber = 10;
pub const SYS_unlinkat: SyscallNumber = 508;
pub const SYS_unmount: SyscallNumber = 22;
pub const SYS_usched_set: SyscallNumber = 481;
pub const SYS_utimensat: SyscallNumber = 539;
pub const SYS_utimes: SyscallNumber = 138;
pub const SYS_utrace: SyscallNumber = 335;
pub const SYS_uuidgen: SyscallNumber = 392;
pub const SYS_varsym_get: SyscallNumber = 451;
pub const SYS_varsym_list: SyscallNumber = 452;
pub const SYS_varsym_set: SyscallNumber = 450;
pub const SYS_vfork: SyscallNumber = 66;
pub const SYS_vmm_guest_ctl: SyscallNumber = 534;
pub const SYS_vmm_guest_sync_addr: SyscallNumber = 535;
pub const SYS_vmspace_create: SyscallNumber = 486;
pub const SYS_vmspace_ctl: SyscallNumber = 488;
pub const SYS_vmspace_destroy: SyscallNumber = 487;
pub const SYS_vmspace_mcontrol: SyscallNumber = 491;
pub const SYS_vmspace_mmap: SyscallNumber = 489;
pub const SYS_vmspace_munmap: SyscallNumber = 490;
pub const SYS_vmspace_pread: SyscallNumber = 492;
pub const SYS_vmspace_pwrite: SyscallNumber = 493;
pub const SYS_vquotactl: SyscallNumber = 530;
pub const SYS_wait4: SyscallNumber = 7;
pub const SYS_write: SyscallNumber = 4;
pub const SYS_writev: SyscallNumber = 121;
pub const SYS_yield: SyscallNumber = 321;
pub const SYS___acl_aclcheck_fd: SyscallNumber = 354;
pub const SYS___acl_aclcheck_file: SyscallNumber = 353;
pub const SYS___acl_delete_fd: SyscallNumber = 352;
pub const SYS___acl_delete_file: SyscallNumber = 351;
pub const SYS___acl_get_fd: SyscallNumber = 349;
pub const SYS___acl_get_file: SyscallNumber = 347;
pub const SYS___acl_set_fd: SyscallNumber = 350;
pub const SYS___acl_set_file: SyscallNumber = 348;
pub const SYS___getcwd: SyscallNumber = 326;
pub const SYS___semctl: SyscallNumber = 220;
pub const SYS___syscall: SyscallNumber = 198;
pub const SYS___sysctl: SyscallNumber = 202;
