// This file is part of syscall-alt. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/syscall-alt/master/COPYRIGHT. No part of syscall-alt, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2016 The developers of syscall-alt. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/syscall-alt/master/COPYRIGHT.


use ::PosixErrorNumber;
use ::negativePosixErrorNumber;
use super::E;


pub const E2BIG: PosixErrorNumber = negativePosixErrorNumber(E::E2BIG);
pub const EACCES: PosixErrorNumber = negativePosixErrorNumber(E::EACCES);
pub const EADDRINUSE: PosixErrorNumber = negativePosixErrorNumber(E::EADDRINUSE);
pub const EADDRNOTAVAIL: PosixErrorNumber = negativePosixErrorNumber(E::EADDRNOTAVAIL);
pub const EAFNOSUPPORT: PosixErrorNumber = negativePosixErrorNumber(E::EAFNOSUPPORT);
pub const EAGAIN: PosixErrorNumber = negativePosixErrorNumber(E::EAGAIN);
pub const EALREADY: PosixErrorNumber = negativePosixErrorNumber(E::EALREADY);
pub const EAUTH: PosixErrorNumber = negativePosixErrorNumber(E::EAUTH);
pub const EBADF: PosixErrorNumber = negativePosixErrorNumber(E::EBADF);
pub const EBADRPC: PosixErrorNumber = negativePosixErrorNumber(E::EBADRPC);
pub const EBUSY: PosixErrorNumber = negativePosixErrorNumber(E::EBUSY);
pub const ECANCELED: PosixErrorNumber = negativePosixErrorNumber(E::ECANCELED);
pub const ECHILD: PosixErrorNumber = negativePosixErrorNumber(E::ECHILD);
pub const ECONNABORTED: PosixErrorNumber = negativePosixErrorNumber(E::ECONNABORTED);
pub const ECONNREFUSED: PosixErrorNumber = negativePosixErrorNumber(E::ECONNREFUSED);
pub const ECONNRESET: PosixErrorNumber = negativePosixErrorNumber(E::ECONNRESET);
pub const EDEADLK: PosixErrorNumber = negativePosixErrorNumber(E::EDEADLK);
pub const EDESTADDRREQ: PosixErrorNumber = negativePosixErrorNumber(E::EDESTADDRREQ);
pub const EDOM: PosixErrorNumber = negativePosixErrorNumber(E::EDOM);
pub const EDQUOT: PosixErrorNumber = negativePosixErrorNumber(E::EDQUOT);
pub const EEXIST: PosixErrorNumber = negativePosixErrorNumber(E::EEXIST);
pub const EFAULT: PosixErrorNumber = negativePosixErrorNumber(E::EFAULT);
pub const EFBIG: PosixErrorNumber = negativePosixErrorNumber(E::EFBIG);
pub const EFTYPE: PosixErrorNumber = negativePosixErrorNumber(E::EFTYPE);
pub const EHOSTDOWN: PosixErrorNumber = negativePosixErrorNumber(E::EHOSTDOWN);
pub const EHOSTUNREACH: PosixErrorNumber = negativePosixErrorNumber(E::EHOSTUNREACH);
pub const EIDRM: PosixErrorNumber = negativePosixErrorNumber(E::EIDRM);
pub const EILSEQ: PosixErrorNumber = negativePosixErrorNumber(E::EILSEQ);
pub const EINPROGRESS: PosixErrorNumber = negativePosixErrorNumber(E::EINPROGRESS);
pub const EINTR: PosixErrorNumber = negativePosixErrorNumber(E::EINTR);
pub const EINVAL: PosixErrorNumber = negativePosixErrorNumber(E::EINVAL);
pub const EIO: PosixErrorNumber = negativePosixErrorNumber(E::EIO);
pub const EIPSEC: PosixErrorNumber = negativePosixErrorNumber(E::EIPSEC);
pub const EISCONN: PosixErrorNumber = negativePosixErrorNumber(E::EISCONN);
pub const EISDIR: PosixErrorNumber = negativePosixErrorNumber(E::EISDIR);
pub const ELOOP: PosixErrorNumber = negativePosixErrorNumber(E::ELOOP);
pub const EMEDIUMTYPE: PosixErrorNumber = negativePosixErrorNumber(E::EMEDIUMTYPE);
pub const EMFILE: PosixErrorNumber = negativePosixErrorNumber(E::EMFILE);
pub const EMLINK: PosixErrorNumber = negativePosixErrorNumber(E::EMLINK);
pub const EMSGSIZE: PosixErrorNumber = negativePosixErrorNumber(E::EMSGSIZE);
pub const ENAMETOOLONG: PosixErrorNumber = negativePosixErrorNumber(E::ENAMETOOLONG);
pub const ENEEDAUTH: PosixErrorNumber = negativePosixErrorNumber(E::ENEEDAUTH);
pub const ENETDOWN: PosixErrorNumber = negativePosixErrorNumber(E::ENETDOWN);
pub const ENETRESET: PosixErrorNumber = negativePosixErrorNumber(E::ENETRESET);
pub const ENETUNREACH: PosixErrorNumber = negativePosixErrorNumber(E::ENETUNREACH);
pub const ENFILE: PosixErrorNumber = negativePosixErrorNumber(E::ENFILE);
pub const ENOATTR: PosixErrorNumber = negativePosixErrorNumber(E::ENOATTR);
pub const ENOBUFS: PosixErrorNumber = negativePosixErrorNumber(E::ENOBUFS);
pub const ENODEV: PosixErrorNumber = negativePosixErrorNumber(E::ENODEV);
pub const ENOENT: PosixErrorNumber = negativePosixErrorNumber(E::ENOENT);
pub const ENOEXEC: PosixErrorNumber = negativePosixErrorNumber(E::ENOEXEC);
pub const ENOLCK: PosixErrorNumber = negativePosixErrorNumber(E::ENOLCK);
pub const ENOMEDIUM: PosixErrorNumber = negativePosixErrorNumber(E::ENOMEDIUM);
pub const ENOMEM: PosixErrorNumber = negativePosixErrorNumber(E::ENOMEM);
pub const ENOMSG: PosixErrorNumber = negativePosixErrorNumber(E::ENOMSG);
pub const ENOPROTOOPT: PosixErrorNumber = negativePosixErrorNumber(E::ENOPROTOOPT);
pub const ENOSPC: PosixErrorNumber = negativePosixErrorNumber(E::ENOSPC);
pub const ENOSYS: PosixErrorNumber = negativePosixErrorNumber(E::ENOSYS);
pub const ENOTBLK: PosixErrorNumber = negativePosixErrorNumber(E::ENOTBLK);
pub const ENOTCONN: PosixErrorNumber = negativePosixErrorNumber(E::ENOTCONN);
pub const ENOTDIR: PosixErrorNumber = negativePosixErrorNumber(E::ENOTDIR);
pub const ENOTEMPTY: PosixErrorNumber = negativePosixErrorNumber(E::ENOTEMPTY);
pub const ENOTSOCK: PosixErrorNumber = negativePosixErrorNumber(E::ENOTSOCK);
pub const ENOTSUP: PosixErrorNumber = negativePosixErrorNumber(E::ENOTSUP);
pub const ENOTTY: PosixErrorNumber = negativePosixErrorNumber(E::ENOTTY);
pub const ENXIO: PosixErrorNumber = negativePosixErrorNumber(E::ENXIO);
pub const EOPNOTSUPP: PosixErrorNumber = negativePosixErrorNumber(E::EOPNOTSUPP);
pub const EOVERFLOW: PosixErrorNumber = negativePosixErrorNumber(E::EOVERFLOW);
pub const EPERM: PosixErrorNumber = negativePosixErrorNumber(E::EPERM);
pub const EPFNOSUPPORT: PosixErrorNumber = negativePosixErrorNumber(E::EPFNOSUPPORT);
pub const EPIPE: PosixErrorNumber = negativePosixErrorNumber(E::EPIPE);
pub const EPROCLIM: PosixErrorNumber = negativePosixErrorNumber(E::EPROCLIM);
pub const EPROCUNAVAIL: PosixErrorNumber = negativePosixErrorNumber(E::EPROCUNAVAIL);
pub const EPROGMISMATCH: PosixErrorNumber = negativePosixErrorNumber(E::EPROGMISMATCH);
pub const EPROGUNAVAIL: PosixErrorNumber = negativePosixErrorNumber(E::EPROGUNAVAIL);
pub const EPROTONOSUPPORT: PosixErrorNumber = negativePosixErrorNumber(E::EPROTONOSUPPORT);
pub const EPROTOTYPE: PosixErrorNumber = negativePosixErrorNumber(E::EPROTOTYPE);
pub const ERANGE: PosixErrorNumber = negativePosixErrorNumber(E::ERANGE);
pub const EREMOTE: PosixErrorNumber = negativePosixErrorNumber(E::EREMOTE);
pub const EROFS: PosixErrorNumber = negativePosixErrorNumber(E::EROFS);
pub const ERPCMISMATCH: PosixErrorNumber = negativePosixErrorNumber(E::ERPCMISMATCH);
pub const ESHUTDOWN: PosixErrorNumber = negativePosixErrorNumber(E::ESHUTDOWN);
pub const ESOCKTNOSUPPORT: PosixErrorNumber = negativePosixErrorNumber(E::ESOCKTNOSUPPORT);
pub const ESPIPE: PosixErrorNumber = negativePosixErrorNumber(E::ESPIPE);
pub const ESRCH: PosixErrorNumber = negativePosixErrorNumber(E::ESRCH);
pub const ESTALE: PosixErrorNumber = negativePosixErrorNumber(E::ESTALE);
pub const ETIMEDOUT: PosixErrorNumber = negativePosixErrorNumber(E::ETIMEDOUT);
pub const ETOOMANYREFS: PosixErrorNumber = negativePosixErrorNumber(E::ETOOMANYREFS);
pub const ETXTBSY: PosixErrorNumber = negativePosixErrorNumber(E::ETXTBSY);
pub const EUSERS: PosixErrorNumber = negativePosixErrorNumber(E::EUSERS);
pub const EWOULDBLOCK: PosixErrorNumber = negativePosixErrorNumber(E::EWOULDBLOCK);
pub const EXDEV: PosixErrorNumber = negativePosixErrorNumber(E::EXDEV);
