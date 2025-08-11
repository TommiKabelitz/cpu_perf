#[derive(Clone, Copy)]
pub struct PerfEventFlags(u64);

#[allow(dead_code)]
impl PerfEventFlags {
    // Comment for each flag ripped straight from definition in C
    pub const DISABLED: Self = Self(1 << 0); /* off by default */
    pub const INHERIT: Self = Self(1 << 1); /* children inherit it */
    pub const PINNED: Self = Self(1 << 2); /* must always be on PMU */
    pub const EXCLUSIVE: Self = Self(1 << 3); /* only group on PMU */
    pub const EXCLUDE_USER: Self = Self(1 << 4); /* don't count user */
    pub const EXCLUDE_KERNEL: Self = Self(1 << 5); /* don't count kernel */
    pub const EXCLUDE_HV: Self = Self(1 << 6); /* don't count hypervisor */
    pub const EXCLUDE_IDLE: Self = Self(1 << 7); /* don't count when idle */
    pub const MMAP: Self = Self(1 << 8); /* include mmap data */
    pub const COMM: Self = Self(1 << 9); /* include comm data */
    pub const FREQ: Self = Self(1 << 10); /* use freq, not period */
    pub const INHERIT_STAT: Self = Self(1 << 11); /* per task counts */
    pub const ENABLE_ON_EXEC: Self = Self(1 << 12); /* next exec enables */
    pub const TASK: Self = Self(1 << 13); /* trace fork/exit */
    pub const WATERMARK: Self = Self(1 << 14); /* wakeup_watermark */
    pub const PRECISE_IP: Self = Self(1 << 15); /* skid constraint */
    pub const MMAP_DATA: Self = Self(1 << 17); /* non-exec mmap data */
    pub const SAMPLE_ID_ALL: Self = Self(1 << 18); /* sample_type all events */
    pub const EXCLUDE_HOST: Self = Self(1 << 19); /* don't count in host */
    pub const EXCLUDE_GUEST: Self = Self(1 << 20); /* don't count in guest */
    pub const EXCLUDE_CALLCHAIN_KERNEL: Self = Self(1 << 21); /* exclude kernel callchains */
    pub const EXCLUDE_CALLCHAIN_USER: Self = Self(1 << 22); /* exclude user callchains */
    pub const MMAP2: Self = Self(1 << 23); /* include mmap with inode data */
    pub const COMM_EXEC: Self = Self(1 << 24); /* flag comm events that are due to exec */
    pub const USE_CLOCKID: Self = Self(1 << 25); /* use clockid for time fields */
    pub const CONTEXT_SWITCH: Self = Self(1 << 26); /* context switch data */
    pub const WRITE_BACKWARD: Self = Self(1 << 27); /* Write ring buffer from end to beginning */
    pub const NAMESPACES: Self = Self(1 << 28); /* include namespaces data */
    pub const KSYMBOL: Self = Self(1 << 29); /* include ksymbol events */
    pub const BPF_EVENT: Self = Self(1 << 30); /* include bpf events */
    pub const AUX_OUTPUT: Self = Self(1 << 31); /* generate AUX records instead of events */
    pub const CGROUP: Self = Self(1 << 32); /* include cgroup events */
    pub const TEXT_POKE: Self = Self(1 << 33); /* include text poke events */
    pub const BUILD_ID: Self = Self(1 << 34); /* use build id in mmap2 events */
    pub const INHERIT_THREAD: Self = Self(1 << 35); /* children only inherit if cloned with CLONE_THREAD */
    pub const REMOVE_ON_EXEC: Self = Self(1 << 36); /* event is removed from task on exec */
    pub const SIGTRAP: Self = Self(1 << 37); /* send synchronous SIGTRAP on event */

    pub fn bits(self) -> u64 {
        self.0
    }

    pub fn contains(self, other: Self) -> bool {
        (self.0 & other.0) == other.0
    }

    pub fn with_precise_ip(self, level: PreciseIp) -> Self {
        const PRECISE_IP_MASK: u64 = 0b11 << 15;
        Self((self.0 & !PRECISE_IP_MASK) | ((level as u64) << 15))
    }
}

impl std::ops::BitOr for PerfEventFlags {
    type Output = Self;
    fn bitor(self, rhs: Self) -> Self {
        Self(self.0 | rhs.0)
    }
}

impl std::ops::BitOrAssign for PerfEventFlags {
    fn bitor_assign(&mut self, rhs: Self) {
        self.0 |= rhs.0
    }
}

#[allow(dead_code)]
#[derive(Debug, Clone, Copy)]
#[repr(u8)]
pub enum PreciseIp {
    Arbitrary = 0,
    Constant = 1,
    RequestZeroSkid = 2,
    ForceZeroSkid = 3,
}
