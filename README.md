[](This file is part of syscall-alt. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/syscall-alt/master/COPYRIGHT. No part of syscall-alt, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.)
[](Copyright © 2016 The developers of syscall-alt. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/syscall-alt/master/COPYRIGHT.)

# syscall-alt

[syscall-alt] is a rust crate that provides inline assembler (`asm!` macro) definitions of syscalls for some Linux platforms. It also includes:-

* An enum defining common Linux syscalls;
* Some minor wrapper logic to make straightforward to work with syscalls.
* Correct Syscall / Posix error codes ('E'), eg `EINVAL`
	* These differ slightly on MIPS and PowerPC
	* MIPS64 is the same as MIPS
	* PowerPC64 is the same as PowerPC
	* PowerPC is ***nearly*** identical to x86_64 bar `EDEADLOCK`:-
		* PowerPC is the only platform on which `EDEADLOCK != EDEADLK` (so watch out when using `match`)
	* MIPS is ***very*** different to x86_64
		* But it has ***exactly*** the same set of E numbers

The code is very much 'early-days', so expect breaking changes.


## TODO

* syscalls are not actually implemented for mips, mips64, powerpc, powerpc64, or s390x but could be by tring to use musl's `syscall_arch.h` headers (bar s390x)


## Licensing

The license for this project is AGPL.

[syscall-alt]: https://github.com/lemonrock/syscall-alt "syscall-alt GitHub page"
