//! 64-bit specific definitions for linux-like values

pub type clock_t = i64;
pub type time_t = i64;
pub type ino_t = u64;
pub type off_t = i64;
pub type blkcnt_t = i64;
pub type __fsword_t = i64;
pub type shmatt_t = u64;
pub type msgqnum_t = u64;
pub type msglen_t = u64;
pub type fsblkcnt_t = u64;
pub type fsfilcnt_t = u64;
pub type rlim_t = u64;

s! {
    pub struct sigset_t {
        #[cfg(target_pointer_width = "32")]
        __val: [u32; 32],
        #[cfg(target_pointer_width = "64")]
        __val: [u64; 16],
    }

    pub struct sysinfo {
        pub uptime: i64,
        pub loads: [u64; 3],
        pub totalram: u64,
        pub freeram: u64,
        pub sharedram: u64,
        pub bufferram: u64,
        pub totalswap: u64,
        pub freeswap: u64,
        pub procs: ::c_ushort,
        pub pad: ::c_ushort,
        pub totalhigh: u64,
        pub freehigh: u64,
        pub mem_unit: ::c_uint,
        pub _f: [::c_char; 0],
    }

    pub struct msqid_ds {
        pub msg_perm: ::ipc_perm,
        pub msg_stime: ::time_t,
        pub msg_rtime: ::time_t,
        pub msg_ctime: ::time_t,
        __msg_cbytes: u64,
        pub msg_qnum: ::msgqnum_t,
        pub msg_qbytes: ::msglen_t,
        pub msg_lspid: ::pid_t,
        pub msg_lrpid: ::pid_t,
        __glibc_reserved4: u64,
        __glibc_reserved5: u64,
    }
}

pub const __SIZEOF_PTHREAD_RWLOCKATTR_T: usize = 8;

pub const O_LARGEFILE: ::c_int = 0;

cfg_if! {
    if #[cfg(target_arch = "aarch64")] {
        mod aarch64;
        pub use self::aarch64::*;
    } else if #[cfg(any(target_arch = "powerpc64"))] {
        mod powerpc64;
        pub use self::powerpc64::*;
    } else if #[cfg(any(target_arch = "sparc64"))] {
        mod sparc64;
        pub use self::sparc64::*;
    } else if #[cfg(any(target_arch = "x86_64"))] {
        mod x86_64;
        pub use self::x86_64::ipc_perm;
        pub use self::x86_64::NCCS;
        pub use self::x86_64::EOPNOTSUPP;
        pub use self::x86_64::pthread_attr_t;
        pub use self::x86_64::c_char;
        pub use self::x86_64::stat64;
        pub use self::x86_64::__SIZEOF_PTHREAD_CONDATTR_T;
        pub use self::x86_64::O_NONBLOCK;
        pub use self::x86_64::ENODATA;
        pub use self::x86_64::O_CLOEXEC;
        pub use self::x86_64::__u64;
        pub use self::x86_64::blksize_t;
        pub use self::x86_64::statvfs64;
        pub use self::x86_64::wchar_t;
        pub use self::x86_64::stat;
        pub use self::x86_64::shmid_ds;
        pub use self::x86_64::suseconds_t;
        pub use self::x86_64::SO_TIMESTAMP;
        pub use self::x86_64::nlink_t;
        pub use self::x86_64::__SIZEOF_PTHREAD_MUTEXATTR_T;
        pub use self::x86_64::statfs64;
        pub use self::x86_64::ioperm;
        pub use self::x86_64::iopl;
        pub use self::x86_64::swapcontext;
        pub use self::x86_64::makecontext;
        pub use self::x86_64::setcontext;
        pub use self::x86_64::getcontext;
        pub use self::x86_64::GS;
        pub use self::x86_64::FS;
        pub use self::x86_64::ES;
        pub use self::x86_64::DS;
        pub use self::x86_64::GS_BASE;
        pub use self::x86_64::FS_BASE;
        pub use self::x86_64::SS;
        pub use self::x86_64::RSP;
        pub use self::x86_64::EFLAGS;
        pub use self::x86_64::CS;
        pub use self::x86_64::RIP;
        pub use self::x86_64::ORIG_RAX;
        pub use self::x86_64::RDI;
        pub use self::x86_64::RSI;
        pub use self::x86_64::RCX;
        pub use self::x86_64::RDX;
        pub use self::x86_64::RAX;
        pub use self::x86_64::R8;
        pub use self::x86_64::R9;
        pub use self::x86_64::R10;
        pub use self::x86_64::R11;
        pub use self::x86_64::R12;
        pub use self::x86_64::R13;
        pub use self::x86_64::R14;
        pub use self::x86_64::R15;
        pub use self::x86_64::RBX;
        pub use self::x86_64::RBP;
        pub use self::x86_64::FIONREAD;
        pub use self::x86_64::TIOCSWINSZ;
        pub use self::x86_64::TIOCGWINSZ;
        pub use self::x86_64::TIOCOUTQ;
        pub use self::x86_64::TIOCSPGRP;
        pub use self::x86_64::TIOCGPGRP;
        pub use self::x86_64::TIOCINQ;
        pub use self::x86_64::TCFLSH;
        pub use self::x86_64::TCXONC;
        pub use self::x86_64::TCSBRK;
        pub use self::x86_64::TCSETAF;
        pub use self::x86_64::TCSETAW;
        pub use self::x86_64::TCSETA;
        pub use self::x86_64::TCSETS;
        pub use self::x86_64::TCGETA;
        pub use self::x86_64::TCGETS;
        pub use self::x86_64::TCSETSF;
        pub use self::x86_64::TCSETSW;
        pub use self::x86_64::EXTPROC;
        pub use self::x86_64::FLUSHO;
        pub use self::x86_64::TOSTOP;
        pub use self::x86_64::IEXTEN;
        pub use self::x86_64::VMIN;
        pub use self::x86_64::VEOL;
        pub use self::x86_64::VEOL2;
        pub use self::x86_64::B4800;
        pub use self::x86_64::B460800;
        pub use self::x86_64::B4000000;
        pub use self::x86_64::B3500000;
        pub use self::x86_64::B3000000;
        pub use self::x86_64::B2500000;
        pub use self::x86_64::B200;
        pub use self::x86_64::B2400;
        pub use self::x86_64::B230400;
        pub use self::x86_64::B2000000;
        pub use self::x86_64::B150;
        pub use self::x86_64::B1500000;
        pub use self::x86_64::B1152000;
        pub use self::x86_64::B1000000;
        pub use self::x86_64::B921600;
        pub use self::x86_64::B57600;
        pub use self::x86_64::B576000;
        pub use self::x86_64::B500000;
        pub use self::x86_64::B115200;
        pub use self::x86_64::BOTHER;
        pub use self::x86_64::EXTB;
        pub use self::x86_64::EXTA;
        pub use self::x86_64::B38400;
        pub use self::x86_64::B9600;
        pub use self::x86_64::B1800;
        pub use self::x86_64::B1200;
        pub use self::x86_64::B600;
        pub use self::x86_64::B300;
        pub use self::x86_64::B134;
        pub use self::x86_64::B110;
        pub use self::x86_64::B75;
        pub use self::x86_64::B50;
        pub use self::x86_64::B0;
        pub use self::x86_64::XTABS;
        pub use self::x86_64::VTDLY;
        pub use self::x86_64::FFDLY;
        pub use self::x86_64::BSDLY;
        pub use self::x86_64::TAB1;
        pub use self::x86_64::TABDLY;
        pub use self::x86_64::CRDLY;
        pub use self::x86_64::NLDLY;
        pub use self::x86_64::OLCUC;
        pub use self::x86_64::VSWTC;
        pub use self::x86_64::CBAUD;
        pub use self::x86_64::CBAUDEX;
        pub use self::x86_64::SO_REUSEADDR;
        pub use self::x86_64::SO_TYPE;
        pub use self::x86_64::SO_ERROR;
        pub use self::x86_64::SO_DONTROUTE;
        pub use self::x86_64::SO_BROADCAST;
        pub use self::x86_64::SO_SNDBUF;
        pub use self::x86_64::SO_RCVBUF;
        pub use self::x86_64::SO_SNDBUFFORCE;
        pub use self::x86_64::SO_RCVBUFFORCE;
        pub use self::x86_64::SO_KEEPALIVE;
        pub use self::x86_64::SO_OOBINLINE;
        pub use self::x86_64::SO_NO_CHECK;
        pub use self::x86_64::SO_PRIORITY;
        pub use self::x86_64::SO_LINGER;
        pub use self::x86_64::SO_BSDCOMPAT;
        pub use self::x86_64::SO_REUSEPORT;
        pub use self::x86_64::SO_PASSCRED;
        pub use self::x86_64::SO_PEERCRED;
        pub use self::x86_64::SO_RCVLOWAT;
        pub use self::x86_64::SO_SNDLOWAT;
        pub use self::x86_64::SO_RCVTIMEO;
        pub use self::x86_64::SO_SNDTIMEO;
        pub use self::x86_64::SO_SECURITY_AUTHENTICATION;
        pub use self::x86_64::SO_SECURITY_ENCRYPTION_TRANSPORT;
        pub use self::x86_64::SO_SECURITY_ENCRYPTION_NETWORK;
        pub use self::x86_64::SO_BINDTODEVICE;
        pub use self::x86_64::SO_ATTACH_FILTER;
        pub use self::x86_64::SO_DETACH_FILTER;
        pub use self::x86_64::SO_GET_FILTER;
        pub use self::x86_64::SO_PEERNAME;
        pub use self::x86_64::SO_ACCEPTCONN;
        pub use self::x86_64::SO_PEERSEC;
        pub use self::x86_64::SO_PASSSEC;
        pub use self::x86_64::SO_TIMESTAMPNS;
        pub use self::x86_64::SCM_TIMESTAMPNS;
        pub use self::x86_64::SO_MARK;
        pub use self::x86_64::SO_TIMESTAMPING;
        pub use self::x86_64::SCM_TIMESTAMPING;
        pub use self::x86_64::SO_PROTOCOL;
        pub use self::x86_64::SO_DOMAIN;
        pub use self::x86_64::SO_RXQ_OVFL;
        pub use self::x86_64::SO_WIFI_STATUS;
        pub use self::x86_64::SCM_WIFI_STATUS;
        pub use self::x86_64::SO_PEEK_OFF;
        pub use self::x86_64::SO_NOFCS;
        pub use self::x86_64::SO_LOCK_FILTER;
        pub use self::x86_64::SO_SELECT_ERR_QUEUE;
        pub use self::x86_64::SO_BUSY_POLL;
        pub use self::x86_64::SO_MAX_PACING_RATE;
        pub use self::x86_64::SO_BPF_EXTENSIONS;
        pub use self::x86_64::SO_INCOMING_CPU;
        pub use self::x86_64::SO_ATTACH_BPF;
        pub use self::x86_64::SO_DETACH_BPF;
        pub use self::x86_64::SA_ONSTACK;
        pub use self::x86_64::SA_SIGINFO;
        pub use self::x86_64::SA_NOCLDWAIT;
        pub use self::x86_64::SIGCHLD;
        pub use self::x86_64::SIGBUS;
        pub use self::x86_64::SIGUSR1;
        pub use self::x86_64::SIGUSR2;
        pub use self::x86_64::SIGCONT;
        pub use self::x86_64::SIGSTOP;
        pub use self::x86_64::SIGTSTP;
        pub use self::x86_64::SIGURG;
        pub use self::x86_64::SIGIO;
        pub use self::x86_64::SIGSYS;
        pub use self::x86_64::SIGSTKFLT;
        pub use self::x86_64::SIGUNUSED;
        pub use self::x86_64::SIGPOLL;
        pub use self::x86_64::SIGPWR;
        pub use self::x86_64::SIG_SETMASK;
        pub use self::x86_64::SIG_BLOCK;
        pub use self::x86_64::SIG_UNBLOCK;
        pub use self::x86_64::POLLWRNORM;
        pub use self::x86_64::POLLWRBAND;
        pub use self::x86_64::O_ASYNC;
        pub use self::x86_64::O_NDELAY;
        pub use self::x86_64::PTRACE_DETACH;
        pub use self::x86_64::EFD_NONBLOCK;
        pub use self::x86_64::F_GETLK;
        pub use self::x86_64::F_GETOWN;
        pub use self::x86_64::F_SETOWN;
        pub use self::x86_64::F_SETLK;
        pub use self::x86_64::F_SETLKW;
        pub use self::x86_64::CIBAUD;
        pub use self::x86_64::NOFLSH;
        pub use self::x86_64::PENDIN;
        pub use self::x86_64::ICANON;
        pub use self::x86_64::ISIG;
        pub use self::x86_64::ECHOCTL;
        pub use self::x86_64::ECHOPRT;
        pub use self::x86_64::ECHONL;
        pub use self::x86_64::ECHOK;
        pub use self::x86_64::ECHOE;
        pub use self::x86_64::ECHOKE;
        pub use self::x86_64::CLOCAL;
        pub use self::x86_64::HUPCL;
        pub use self::x86_64::PARODD;
        pub use self::x86_64::PARENB;
        pub use self::x86_64::CREAD;
        pub use self::x86_64::CSTOPB;
        pub use self::x86_64::CS8;
        pub use self::x86_64::CS7;
        pub use self::x86_64::CS6;
        pub use self::x86_64::CSIZE;
        pub use self::x86_64::ONLCR;
        pub use self::x86_64::IXOFF;
        pub use self::x86_64::IXON;
        pub use self::x86_64::VTIME;
        pub use self::x86_64::VDISCARD;
        pub use self::x86_64::VSTOP;
        pub use self::x86_64::VSTART;
        pub use self::x86_64::VSUSP;
        pub use self::x86_64::VREPRINT;
        pub use self::x86_64::VWERASE;
        pub use self::x86_64::VT1;
        pub use self::x86_64::BS1;
        pub use self::x86_64::FF1;
        pub use self::x86_64::CR1;
        pub use self::x86_64::CR2;
        pub use self::x86_64::CR3;
        pub use self::x86_64::TAB2;
        pub use self::x86_64::TAB3;
        pub use self::x86_64::MINSIGSTKSZ;
        pub use self::x86_64::SIGSTKSZ;
        pub use self::x86_64::MCL_FUTURE;
        pub use self::x86_64::MCL_CURRENT;
        pub use self::x86_64::PTRACE_GETREGS;
        pub use self::x86_64::PTRACE_SETREGS;
        pub use self::x86_64::PTRACE_GETFPREGS;
        pub use self::x86_64::PTRACE_GETFPXREGS;
        pub use self::x86_64::PTRACE_SETFPREGS;
        pub use self::x86_64::PTRACE_PEEKSIGINFO_SHARED;
        pub use self::x86_64::PTRACE_SETFPXREGS;
        pub use self::x86_64::FIONBIO;
        pub use self::x86_64::FIOCLEX;
        pub use self::x86_64::EDEADLK;
        pub use self::x86_64::EDEADLOCK;
        pub use self::x86_64::MAP_32BIT;
        pub use self::x86_64::MAP_LOCKED;
        pub use self::x86_64::MAP_NORESERVE;
        pub use self::x86_64::O_NOFOLLOW;
        pub use self::x86_64::O_DIRECTORY;
        pub use self::x86_64::O_DIRECT;
        pub use self::x86_64::EFD_CLOEXEC;
        pub use self::x86_64::EPOLL_CLOEXEC;
        pub use self::x86_64::SA_NODEFER;
        pub use self::x86_64::SA_RESTART;
        pub use self::x86_64::SA_RESETHAND;
        pub use self::x86_64::SA_NOCLDSTOP;
        pub use self::x86_64::EADV;
        pub use self::x86_64::ECOMM;
        pub use self::x86_64::EDOTDOT;
        pub use self::x86_64::EPROTO;
        pub use self::x86_64::ESRMNT;
        pub use self::x86_64::ENOLINK;
        pub use self::x86_64::EREMOTE;
        pub use self::x86_64::ENOPKG;
        pub use self::x86_64::ETIME;
        pub use self::x86_64::ENOSR;
        pub use self::x86_64::ENONET;
        pub use self::x86_64::ENOSTR;
        pub use self::x86_64::EBFONT;
        pub use self::x86_64::O_TRUNC;
        pub use self::x86_64::SFD_CLOEXEC;
        pub use self::x86_64::TIOCSTI;
        pub use self::x86_64::TIOCCONS;
        pub use self::x86_64::TIOCMBIC;
        pub use self::x86_64::TIOCMBIS;
        pub use self::x86_64::TIOCNXCL;
        pub use self::x86_64::TIOCMSET;
        pub use self::x86_64::TIOCMGET;
        pub use self::x86_64::SOL_SOCKET;
        pub use self::x86_64::ERFKILL;
        pub use self::x86_64::SFD_NONBLOCK;
        pub use self::x86_64::TIOCSCTTY;
        pub use self::x86_64::TIOCEXCL;
        pub use self::x86_64::EHWPOISON;
        pub use self::x86_64::ENOTCONN;
        pub use self::x86_64::ENOTRECOVERABLE;
        pub use self::x86_64::EOWNERDEAD;
        pub use self::x86_64::EKEYEXPIRED;
        pub use self::x86_64::EKEYREVOKED;
        pub use self::x86_64::EKEYREJECTED;
        pub use self::x86_64::ENOKEY;
        pub use self::x86_64::ECANCELED;
        pub use self::x86_64::EMEDIUMTYPE;
        pub use self::x86_64::ENOMEDIUM;
        pub use self::x86_64::EDQUOT;
        pub use self::x86_64::ESTALE;
        pub use self::x86_64::EALREADY;
        pub use self::x86_64::EINPROGRESS;
        pub use self::x86_64::EHOSTDOWN;
        pub use self::x86_64::EHOSTUNREACH;
        pub use self::x86_64::ETIMEDOUT;
        pub use self::x86_64::ECONNRESET;
        pub use self::x86_64::ECONNREFUSED;
        pub use self::x86_64::ETOOMANYREFS;
        pub use self::x86_64::ESHUTDOWN;
        pub use self::x86_64::EISCONN;
        pub use self::x86_64::ENOBUFS;
        pub use self::x86_64::ECONNABORTED;
        pub use self::x86_64::ENETRESET;
        pub use self::x86_64::ENETDOWN;
        pub use self::x86_64::ENETUNREACH;
        pub use self::x86_64::EDESTADDRREQ;
        pub use self::x86_64::EADDRINUSE;
        pub use self::x86_64::EADDRNOTAVAIL;
        pub use self::x86_64::EAFNOSUPPORT;
        pub use self::x86_64::EPFNOSUPPORT;
        pub use self::x86_64::EUSERS;
        pub use self::x86_64::EILSEQ;
        pub use self::x86_64::EMSGSIZE;
        pub use self::x86_64::ELIBSCN;
        pub use self::x86_64::ERESTART;
        pub use self::x86_64::ENOTSOCK;
        pub use self::x86_64::ELIBBAD;
        pub use self::x86_64::ELIBMAX;
        pub use self::x86_64::ELIBACC;
        pub use self::x86_64::ELIBEXEC;
        pub use self::x86_64::ESTRPIPE;
        pub use self::x86_64::EBADFD;
        pub use self::x86_64::ENOTUNIQ;
        pub use self::x86_64::EBADSLT;
        pub use self::x86_64::EMULTIHOP;
        pub use self::x86_64::ENOANO;
        pub use self::x86_64::EXFULL;
        pub use self::x86_64::EBADR;
        pub use self::x86_64::EBADRQC;
        pub use self::x86_64::EL2HLT;
        pub use self::x86_64::EBADE;
        pub use self::x86_64::ESOCKTNOSUPPORT;
        pub use self::x86_64::EPROTONOSUPPORT;
        pub use self::x86_64::ENOPROTOOPT;
        pub use self::x86_64::EREMCHG;
        pub use self::x86_64::EBADMSG;
        pub use self::x86_64::EL3HLT;
        pub use self::x86_64::EIDRM;
        pub use self::x86_64::EPROTOTYPE;
        pub use self::x86_64::ENOCSI;
        pub use self::x86_64::EUNATCH;
        pub use self::x86_64::EL3RST;
        pub use self::x86_64::EOVERFLOW;
        pub use self::x86_64::ELOOP;
        pub use self::x86_64::ENOTEMPTY;
        pub use self::x86_64::ENOLCK;
        pub use self::x86_64::ENOSYS;
        pub use self::x86_64::O_PATH;
        pub use self::x86_64::O_FSYNC;
        pub use self::x86_64::O_DSYNC;
        pub use self::x86_64::ELNRNG;
        pub use self::x86_64::O_TMPFILE;
        pub use self::x86_64::O_SYNC;
        pub use self::x86_64::O_EXCL;
        pub use self::x86_64::O_NOCTTY;
        pub use self::x86_64::O_CREAT;
        pub use self::x86_64::O_APPEND;
        pub use self::x86_64::O_RSYNC;
        pub use self::x86_64::O_NOATIME;
        pub use self::x86_64::RLIMIT_NPROC;
        pub use self::x86_64::RLIMIT_NOFILE;
        pub use self::x86_64::TIOCSSOFTCAR;
        pub use self::x86_64::TIOCGSOFTCAR;
        pub use self::x86_64::EL2NSYNC;
        pub use self::x86_64::ENOMSG;
        pub use self::x86_64::ECHRNG;
        pub use self::x86_64::ENAMETOOLONG;
        pub use self::x86_64::MAP_GROWSDOWN;
        cfg_if! {
            if #[cfg(target_pointer_width = "32")] {
                mod x32;
                pub use self::x32::*;
            } else {
                mod not_x32;
                pub use self::not_x32::*;
            }
        }
    } else {
        // Unknown target_arch
    }
}
