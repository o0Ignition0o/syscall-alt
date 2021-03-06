// This file is part of syscall-alt. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/syscall-alt/master/COPYRIGHT. No part of syscall-alt, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2016 The developers of syscall-alt. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/syscall-alt/master/COPYRIGHT.


#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![feature(asm)]
#![feature(associated_consts)]
#![feature(const_fn)]
#![no_std]


include!("PosixErrorNumber.rs");
include!("SyscallArgument.rs");
include!("SyscallNumber.rs");
include!("SyscallResult.rs");
include!("syscallResultFromPosixErrorNumber.rs");
include!("negativePosixErrorNumber.rs");


pub mod constants;
pub mod syscalls;
