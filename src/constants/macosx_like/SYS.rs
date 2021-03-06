// This file is part of syscall-alt. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/syscall-alt/master/COPYRIGHT. No part of syscall-alt, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2016 The developers of syscall-alt. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/syscall-alt/master/COPYRIGHT.


use ::SyscallNumber;


// Generated from Mac OS X El Capitan 10.11.6 (15th September 2016), file /usr/include/sys/syscall.h
pub const SYS_syscall: SyscallNumber = 0;
pub const SYS_exit: SyscallNumber = 1;
pub const SYS_fork: SyscallNumber = 2;
pub const SYS_read: SyscallNumber = 3;
pub const SYS_write: SyscallNumber = 4;
pub const SYS_open: SyscallNumber = 5;
pub const SYS_close: SyscallNumber = 6;
pub const SYS_wait4: SyscallNumber = 7;
pub const SYS_link: SyscallNumber = 9;
pub const SYS_unlink: SyscallNumber = 10;
pub const SYS_chdir: SyscallNumber = 12;
pub const SYS_fchdir: SyscallNumber = 13;
pub const SYS_mknod: SyscallNumber = 14;
pub const SYS_chmod: SyscallNumber = 15;
pub const SYS_chown: SyscallNumber = 16;
pub const SYS_getfsstat: SyscallNumber = 18;
pub const SYS_getpid: SyscallNumber = 20;
pub const SYS_setuid: SyscallNumber = 23;
pub const SYS_getuid: SyscallNumber = 24;
pub const SYS_geteuid: SyscallNumber = 25;
pub const SYS_ptrace: SyscallNumber = 26;
pub const SYS_recvmsg: SyscallNumber = 27;
pub const SYS_sendmsg: SyscallNumber = 28;
pub const SYS_recvfrom: SyscallNumber = 29;
pub const SYS_accept: SyscallNumber = 30;
pub const SYS_getpeername: SyscallNumber = 31;
pub const SYS_getsockname: SyscallNumber = 32;
pub const SYS_access: SyscallNumber = 33;
pub const SYS_chflags: SyscallNumber = 34;
pub const SYS_fchflags: SyscallNumber = 35;
pub const SYS_sync: SyscallNumber = 36;
pub const SYS_kill: SyscallNumber = 37;
pub const SYS_getppid: SyscallNumber = 39;
pub const SYS_dup: SyscallNumber = 41;
pub const SYS_pipe: SyscallNumber = 42;
pub const SYS_getegid: SyscallNumber = 43;
pub const SYS_sigaction: SyscallNumber = 46;
pub const SYS_getgid: SyscallNumber = 47;
pub const SYS_sigprocmask: SyscallNumber = 48;
pub const SYS_getlogin: SyscallNumber = 49;
pub const SYS_setlogin: SyscallNumber = 50;
pub const SYS_acct: SyscallNumber = 51;
pub const SYS_sigpending: SyscallNumber = 52;
pub const SYS_sigaltstack: SyscallNumber = 53;
pub const SYS_ioctl: SyscallNumber = 54;
pub const SYS_reboot: SyscallNumber = 55;
pub const SYS_revoke: SyscallNumber = 56;
pub const SYS_symlink: SyscallNumber = 57;
pub const SYS_readlink: SyscallNumber = 58;
pub const SYS_execve: SyscallNumber = 59;
pub const SYS_umask: SyscallNumber = 60;
pub const SYS_chroot: SyscallNumber = 61;
pub const SYS_msync: SyscallNumber = 65;
pub const SYS_vfork: SyscallNumber = 66;
pub const SYS_munmap: SyscallNumber = 73;
pub const SYS_mprotect: SyscallNumber = 74;
pub const SYS_madvise: SyscallNumber = 75;
pub const SYS_mincore: SyscallNumber = 78;
pub const SYS_getgroups: SyscallNumber = 79;
pub const SYS_setgroups: SyscallNumber = 80;
pub const SYS_getpgrp: SyscallNumber = 81;
pub const SYS_setpgid: SyscallNumber = 82;
pub const SYS_setitimer: SyscallNumber = 83;
pub const SYS_swapon: SyscallNumber = 85;
pub const SYS_getitimer: SyscallNumber = 86;
pub const SYS_getdtablesize: SyscallNumber = 89;
pub const SYS_dup2: SyscallNumber = 90;
pub const SYS_fcntl: SyscallNumber = 92;
pub const SYS_select: SyscallNumber = 93;
pub const SYS_fsync: SyscallNumber = 95;
pub const SYS_setpriority: SyscallNumber = 96;
pub const SYS_socket: SyscallNumber = 97;
pub const SYS_connect: SyscallNumber = 98;
pub const SYS_getpriority: SyscallNumber = 100;
pub const SYS_bind: SyscallNumber = 104;
pub const SYS_setsockopt: SyscallNumber = 105;
pub const SYS_listen: SyscallNumber = 106;
pub const SYS_sigsuspend: SyscallNumber = 111;
pub const SYS_gettimeofday: SyscallNumber = 116;
pub const SYS_getrusage: SyscallNumber = 117;
pub const SYS_getsockopt: SyscallNumber = 118;
pub const SYS_readv: SyscallNumber = 120;
pub const SYS_writev: SyscallNumber = 121;
pub const SYS_settimeofday: SyscallNumber = 122;
pub const SYS_fchown: SyscallNumber = 123;
pub const SYS_fchmod: SyscallNumber = 124;
pub const SYS_setreuid: SyscallNumber = 126;
pub const SYS_setregid: SyscallNumber = 127;
pub const SYS_rename: SyscallNumber = 128;
pub const SYS_flock: SyscallNumber = 131;
pub const SYS_mkfifo: SyscallNumber = 132;
pub const SYS_sendto: SyscallNumber = 133;
pub const SYS_shutdown: SyscallNumber = 134;
pub const SYS_socketpair: SyscallNumber = 135;
pub const SYS_mkdir: SyscallNumber = 136;
pub const SYS_rmdir: SyscallNumber = 137;
pub const SYS_utimes: SyscallNumber = 138;
pub const SYS_futimes: SyscallNumber = 139;
pub const SYS_adjtime: SyscallNumber = 140;
pub const SYS_gethostuuid: SyscallNumber = 142;
pub const SYS_setsid: SyscallNumber = 147;
pub const SYS_getpgid: SyscallNumber = 151;
pub const SYS_setprivexec: SyscallNumber = 152;
pub const SYS_pread: SyscallNumber = 153;
pub const SYS_pwrite: SyscallNumber = 154;
pub const SYS_nfssvc: SyscallNumber = 155;
pub const SYS_statfs: SyscallNumber = 157;
pub const SYS_fstatfs: SyscallNumber = 158;
pub const SYS_unmount: SyscallNumber = 159;
pub const SYS_getfh: SyscallNumber = 161;
pub const SYS_quotactl: SyscallNumber = 165;
pub const SYS_mount: SyscallNumber = 167;
pub const SYS_csops: SyscallNumber = 169;
pub const SYS_csops_audittoken: SyscallNumber = 170;
pub const SYS_waitid: SyscallNumber = 173;
pub const SYS_kdebug_trace_string: SyscallNumber = 178;
pub const SYS_kdebug_trace64: SyscallNumber = 179;
pub const SYS_kdebug_trace: SyscallNumber = 180;
pub const SYS_setgid: SyscallNumber = 181;
pub const SYS_setegid: SyscallNumber = 182;
pub const SYS_seteuid: SyscallNumber = 183;
pub const SYS_sigreturn: SyscallNumber = 184;
pub const SYS_chud: SyscallNumber = 185;
pub const SYS_fdatasync: SyscallNumber = 187;
pub const SYS_stat: SyscallNumber = 188;
pub const SYS_fstat: SyscallNumber = 189;
pub const SYS_lstat: SyscallNumber = 190;
pub const SYS_pathconf: SyscallNumber = 191;
pub const SYS_fpathconf: SyscallNumber = 192;
pub const SYS_getrlimit: SyscallNumber = 194;
pub const SYS_setrlimit: SyscallNumber = 195;
pub const SYS_getdirentries: SyscallNumber = 196;
pub const SYS_mmap: SyscallNumber = 197;
pub const SYS_lseek: SyscallNumber = 199;
pub const SYS_truncate: SyscallNumber = 200;
pub const SYS_ftruncate: SyscallNumber = 201;
pub const SYS_sysctl: SyscallNumber = 202;
pub const SYS_mlock: SyscallNumber = 203;
pub const SYS_munlock: SyscallNumber = 204;
pub const SYS_undelete: SyscallNumber = 205;
pub const SYS_open_dprotected_np: SyscallNumber = 216;
pub const SYS_getattrlist: SyscallNumber = 220;
pub const SYS_setattrlist: SyscallNumber = 221;
pub const SYS_getdirentriesattr: SyscallNumber = 222;
pub const SYS_exchangedata: SyscallNumber = 223;
pub const SYS_searchfs: SyscallNumber = 225;
pub const SYS_delete: SyscallNumber = 226;
pub const SYS_copyfile: SyscallNumber = 227;
pub const SYS_fgetattrlist: SyscallNumber = 228;
pub const SYS_fsetattrlist: SyscallNumber = 229;
pub const SYS_poll: SyscallNumber = 230;
pub const SYS_watchevent: SyscallNumber = 231;
pub const SYS_waitevent: SyscallNumber = 232;
pub const SYS_modwatch: SyscallNumber = 233;
pub const SYS_getxattr: SyscallNumber = 234;
pub const SYS_fgetxattr: SyscallNumber = 235;
pub const SYS_setxattr: SyscallNumber = 236;
pub const SYS_fsetxattr: SyscallNumber = 237;
pub const SYS_removexattr: SyscallNumber = 238;
pub const SYS_fremovexattr: SyscallNumber = 239;
pub const SYS_listxattr: SyscallNumber = 240;
pub const SYS_flistxattr: SyscallNumber = 241;
pub const SYS_fsctl: SyscallNumber = 242;
pub const SYS_initgroups: SyscallNumber = 243;
pub const SYS_posix_spawn: SyscallNumber = 244;
pub const SYS_ffsctl: SyscallNumber = 245;
pub const SYS_nfsclnt: SyscallNumber = 247;
pub const SYS_fhopen: SyscallNumber = 248;
pub const SYS_minherit: SyscallNumber = 250;
pub const SYS_semsys: SyscallNumber = 251;
pub const SYS_msgsys: SyscallNumber = 252;
pub const SYS_shmsys: SyscallNumber = 253;
pub const SYS_semctl: SyscallNumber = 254;
pub const SYS_semget: SyscallNumber = 255;
pub const SYS_semop: SyscallNumber = 256;
pub const SYS_msgctl: SyscallNumber = 258;
pub const SYS_msgget: SyscallNumber = 259;
pub const SYS_msgsnd: SyscallNumber = 260;
pub const SYS_msgrcv: SyscallNumber = 261;
pub const SYS_shmat: SyscallNumber = 262;
pub const SYS_shmctl: SyscallNumber = 263;
pub const SYS_shmdt: SyscallNumber = 264;
pub const SYS_shmget: SyscallNumber = 265;
pub const SYS_shm_open: SyscallNumber = 266;
pub const SYS_shm_unlink: SyscallNumber = 267;
pub const SYS_sem_open: SyscallNumber = 268;
pub const SYS_sem_close: SyscallNumber = 269;
pub const SYS_sem_unlink: SyscallNumber = 270;
pub const SYS_sem_wait: SyscallNumber = 271;
pub const SYS_sem_trywait: SyscallNumber = 272;
pub const SYS_sem_post: SyscallNumber = 273;
pub const SYS_sysctlbyname: SyscallNumber = 274;
pub const SYS_open_extended: SyscallNumber = 277;
pub const SYS_umask_extended: SyscallNumber = 278;
pub const SYS_stat_extended: SyscallNumber = 279;
pub const SYS_lstat_extended: SyscallNumber = 280;
pub const SYS_fstat_extended: SyscallNumber = 281;
pub const SYS_chmod_extended: SyscallNumber = 282;
pub const SYS_fchmod_extended: SyscallNumber = 283;
pub const SYS_access_extended: SyscallNumber = 284;
pub const SYS_settid: SyscallNumber = 285;
pub const SYS_gettid: SyscallNumber = 286;
pub const SYS_setsgroups: SyscallNumber = 287;
pub const SYS_getsgroups: SyscallNumber = 288;
pub const SYS_setwgroups: SyscallNumber = 289;
pub const SYS_getwgroups: SyscallNumber = 290;
pub const SYS_mkfifo_extended: SyscallNumber = 291;
pub const SYS_mkdir_extended: SyscallNumber = 292;
pub const SYS_identitysvc: SyscallNumber = 293;
pub const SYS_shared_region_check_np: SyscallNumber = 294;
pub const SYS_vm_pressure_monitor: SyscallNumber = 296;
pub const SYS_psynch_rw_longrdlock: SyscallNumber = 297;
pub const SYS_psynch_rw_yieldwrlock: SyscallNumber = 298;
pub const SYS_psynch_rw_downgrade: SyscallNumber = 299;
pub const SYS_psynch_rw_upgrade: SyscallNumber = 300;
pub const SYS_psynch_mutexwait: SyscallNumber = 301;
pub const SYS_psynch_mutexdrop: SyscallNumber = 302;
pub const SYS_psynch_cvbroad: SyscallNumber = 303;
pub const SYS_psynch_cvsignal: SyscallNumber = 304;
pub const SYS_psynch_cvwait: SyscallNumber = 305;
pub const SYS_psynch_rw_rdlock: SyscallNumber = 306;
pub const SYS_psynch_rw_wrlock: SyscallNumber = 307;
pub const SYS_psynch_rw_unlock: SyscallNumber = 308;
pub const SYS_psynch_rw_unlock2: SyscallNumber = 309;
pub const SYS_getsid: SyscallNumber = 310;
pub const SYS_settid_with_pid: SyscallNumber = 311;
pub const SYS_psynch_cvclrprepost: SyscallNumber = 312;
pub const SYS_aio_fsync: SyscallNumber = 313;
pub const SYS_aio_return: SyscallNumber = 314;
pub const SYS_aio_suspend: SyscallNumber = 315;
pub const SYS_aio_cancel: SyscallNumber = 316;
pub const SYS_aio_error: SyscallNumber = 317;
pub const SYS_aio_read: SyscallNumber = 318;
pub const SYS_aio_write: SyscallNumber = 319;
pub const SYS_lio_listio: SyscallNumber = 320;
pub const SYS_iopolicysys: SyscallNumber = 322;
pub const SYS_process_policy: SyscallNumber = 323;
pub const SYS_mlockall: SyscallNumber = 324;
pub const SYS_munlockall: SyscallNumber = 325;
pub const SYS_issetugid: SyscallNumber = 327;
pub const SYS___pthread_kill: SyscallNumber = 328;
pub const SYS___pthread_sigmask: SyscallNumber = 329;
pub const SYS___sigwait: SyscallNumber = 330;
pub const SYS___disable_threadsignal: SyscallNumber = 331;
pub const SYS___pthread_markcancel: SyscallNumber = 332;
pub const SYS___pthread_canceled: SyscallNumber = 333;
pub const SYS___semwait_signal: SyscallNumber = 334;
pub const SYS_proc_info: SyscallNumber = 336;
pub const SYS_sendfile: SyscallNumber = 337;
pub const SYS_stat64: SyscallNumber = 338;
pub const SYS_fstat64: SyscallNumber = 339;
pub const SYS_lstat64: SyscallNumber = 340;
pub const SYS_stat64_extended: SyscallNumber = 341;
pub const SYS_lstat64_extended: SyscallNumber = 342;
pub const SYS_fstat64_extended: SyscallNumber = 343;
pub const SYS_getdirentries64: SyscallNumber = 344;
pub const SYS_statfs64: SyscallNumber = 345;
pub const SYS_fstatfs64: SyscallNumber = 346;
pub const SYS_getfsstat64: SyscallNumber = 347;
pub const SYS___pthread_chdir: SyscallNumber = 348;
pub const SYS___pthread_fchdir: SyscallNumber = 349;
pub const SYS_audit: SyscallNumber = 350;
pub const SYS_auditon: SyscallNumber = 351;
pub const SYS_getauid: SyscallNumber = 353;
pub const SYS_setauid: SyscallNumber = 354;
pub const SYS_getaudit_addr: SyscallNumber = 357;
pub const SYS_setaudit_addr: SyscallNumber = 358;
pub const SYS_auditctl: SyscallNumber = 359;
pub const SYS_bsdthread_create: SyscallNumber = 360;
pub const SYS_bsdthread_terminate: SyscallNumber = 361;
pub const SYS_kqueue: SyscallNumber = 362;
pub const SYS_kevent: SyscallNumber = 363;
pub const SYS_lchown: SyscallNumber = 364;
pub const SYS_stack_snapshot: SyscallNumber = 365;
pub const SYS_bsdthread_register: SyscallNumber = 366;
pub const SYS_workq_open: SyscallNumber = 367;
pub const SYS_workq_kernreturn: SyscallNumber = 368;
pub const SYS_kevent64: SyscallNumber = 369;
pub const SYS___old_semwait_signal: SyscallNumber = 370;
pub const SYS___old_semwait_signal_nocancel: SyscallNumber = 371;
pub const SYS_thread_selfid: SyscallNumber = 372;
pub const SYS_ledger: SyscallNumber = 373;
pub const SYS_kevent_qos: SyscallNumber = 374;
pub const SYS___mac_execve: SyscallNumber = 380;
pub const SYS___mac_syscall: SyscallNumber = 381;
pub const SYS___mac_get_file: SyscallNumber = 382;
pub const SYS___mac_set_file: SyscallNumber = 383;
pub const SYS___mac_get_link: SyscallNumber = 384;
pub const SYS___mac_set_link: SyscallNumber = 385;
pub const SYS___mac_get_proc: SyscallNumber = 386;
pub const SYS___mac_set_proc: SyscallNumber = 387;
pub const SYS___mac_get_fd: SyscallNumber = 388;
pub const SYS___mac_set_fd: SyscallNumber = 389;
pub const SYS___mac_get_pid: SyscallNumber = 390;
pub const SYS_pselect: SyscallNumber = 394;
pub const SYS_pselect_nocancel: SyscallNumber = 395;
pub const SYS_read_nocancel: SyscallNumber = 396;
pub const SYS_write_nocancel: SyscallNumber = 397;
pub const SYS_open_nocancel: SyscallNumber = 398;
pub const SYS_close_nocancel: SyscallNumber = 399;
pub const SYS_wait4_nocancel: SyscallNumber = 400;
pub const SYS_recvmsg_nocancel: SyscallNumber = 401;
pub const SYS_sendmsg_nocancel: SyscallNumber = 402;
pub const SYS_recvfrom_nocancel: SyscallNumber = 403;
pub const SYS_accept_nocancel: SyscallNumber = 404;
pub const SYS_msync_nocancel: SyscallNumber = 405;
pub const SYS_fcntl_nocancel: SyscallNumber = 406;
pub const SYS_select_nocancel: SyscallNumber = 407;
pub const SYS_fsync_nocancel: SyscallNumber = 408;
pub const SYS_connect_nocancel: SyscallNumber = 409;
pub const SYS_sigsuspend_nocancel: SyscallNumber = 410;
pub const SYS_readv_nocancel: SyscallNumber = 411;
pub const SYS_writev_nocancel: SyscallNumber = 412;
pub const SYS_sendto_nocancel: SyscallNumber = 413;
pub const SYS_pread_nocancel: SyscallNumber = 414;
pub const SYS_pwrite_nocancel: SyscallNumber = 415;
pub const SYS_waitid_nocancel: SyscallNumber = 416;
pub const SYS_poll_nocancel: SyscallNumber = 417;
pub const SYS_msgsnd_nocancel: SyscallNumber = 418;
pub const SYS_msgrcv_nocancel: SyscallNumber = 419;
pub const SYS_sem_wait_nocancel: SyscallNumber = 420;
pub const SYS_aio_suspend_nocancel: SyscallNumber = 421;
pub const SYS___sigwait_nocancel: SyscallNumber = 422;
pub const SYS___semwait_signal_nocancel: SyscallNumber = 423;
pub const SYS___mac_mount: SyscallNumber = 424;
pub const SYS___mac_get_mount: SyscallNumber = 425;
pub const SYS___mac_getfsstat: SyscallNumber = 426;
pub const SYS_fsgetpath: SyscallNumber = 427;
pub const SYS_audit_session_self: SyscallNumber = 428;
pub const SYS_audit_session_join: SyscallNumber = 429;
pub const SYS_fileport_makeport: SyscallNumber = 430;
pub const SYS_fileport_makefd: SyscallNumber = 431;
pub const SYS_audit_session_port: SyscallNumber = 432;
pub const SYS_pid_suspend: SyscallNumber = 433;
pub const SYS_pid_resume: SyscallNumber = 434;
pub const SYS_pid_hibernate: SyscallNumber = 435;
pub const SYS_pid_shutdown_sockets: SyscallNumber = 436;
pub const SYS_shared_region_map_and_slide_np: SyscallNumber = 438;
pub const SYS_kas_info: SyscallNumber = 439;
pub const SYS_memorystatus_control: SyscallNumber = 440;
pub const SYS_guarded_open_np: SyscallNumber = 441;
pub const SYS_guarded_close_np: SyscallNumber = 442;
pub const SYS_guarded_kqueue_np: SyscallNumber = 443;
pub const SYS_change_fdguard_np: SyscallNumber = 444;
pub const SYS_usrctl: SyscallNumber = 445;
pub const SYS_proc_rlimit_control: SyscallNumber = 446;
pub const SYS_connectx: SyscallNumber = 447;
pub const SYS_disconnectx: SyscallNumber = 448;
pub const SYS_peeloff: SyscallNumber = 449;
pub const SYS_socket_delegate: SyscallNumber = 450;
pub const SYS_telemetry: SyscallNumber = 451;
pub const SYS_proc_uuid_policy: SyscallNumber = 452;
pub const SYS_memorystatus_get_level: SyscallNumber = 453;
pub const SYS_system_override: SyscallNumber = 454;
pub const SYS_vfs_purge: SyscallNumber = 455;
pub const SYS_sfi_ctl: SyscallNumber = 456;
pub const SYS_sfi_pidctl: SyscallNumber = 457;
pub const SYS_coalition: SyscallNumber = 458;
pub const SYS_coalition_info: SyscallNumber = 459;
pub const SYS_necp_match_policy: SyscallNumber = 460;
pub const SYS_getattrlistbulk: SyscallNumber = 461;
pub const SYS_openat: SyscallNumber = 463;
pub const SYS_openat_nocancel: SyscallNumber = 464;
pub const SYS_renameat: SyscallNumber = 465;
pub const SYS_faccessat: SyscallNumber = 466;
pub const SYS_fchmodat: SyscallNumber = 467;
pub const SYS_fchownat: SyscallNumber = 468;
pub const SYS_fstatat: SyscallNumber = 469;
pub const SYS_fstatat64: SyscallNumber = 470;
pub const SYS_linkat: SyscallNumber = 471;
pub const SYS_unlinkat: SyscallNumber = 472;
pub const SYS_readlinkat: SyscallNumber = 473;
pub const SYS_symlinkat: SyscallNumber = 474;
pub const SYS_mkdirat: SyscallNumber = 475;
pub const SYS_getattrlistat: SyscallNumber = 476;
pub const SYS_proc_trace_log: SyscallNumber = 477;
pub const SYS_bsdthread_ctl: SyscallNumber = 478;
pub const SYS_openbyid_np: SyscallNumber = 479;
pub const SYS_recvmsg_x: SyscallNumber = 480;
pub const SYS_sendmsg_x: SyscallNumber = 481;
pub const SYS_thread_selfusage: SyscallNumber = 482;
pub const SYS_csrctl: SyscallNumber = 483;
pub const SYS_guarded_open_dprotected_np: SyscallNumber = 484;
pub const SYS_guarded_write_np: SyscallNumber = 485;
pub const SYS_guarded_pwrite_np: SyscallNumber = 486;
pub const SYS_guarded_writev_np: SyscallNumber = 487;
pub const SYS_rename_ext: SyscallNumber = 488;
pub const SYS_mremap_encrypted: SyscallNumber = 489;
pub const SYS_netagent_trigger: SyscallNumber = 490;
pub const SYS_stack_snapshot_with_config: SyscallNumber = 491;
pub const SYS_microstackshot: SyscallNumber = 492;
pub const SYS_grab_pgo_data: SyscallNumber = 493;
pub const SYS_persona: SyscallNumber = 494;
pub const SYS_work_interval_ctl: SyscallNumber = 499;
