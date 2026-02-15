// AeroFlow Runtime - Capability Guard
// Sandboxing and privilege enforcement

use bitflags::bitflags;

bitflags! {
    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    pub struct Capabilities: u32 {
        const NONE = 0;
        const NET_RECV = 1 << 0;
        const NET_SEND = 1 << 1;
        const FS_READ = 1 << 2;
        const FS_WRITE = 1 << 3;
        const GPU_ACCEL = 1 << 4;
        const SYS_ADMIN = 1 << 31;
    }
}

pub struct CapabilityGuard {
    allowed: Capabilities,
}

impl CapabilityGuard {
    pub fn new(allowed: Capabilities) -> Self {
        Self { allowed }
    }

    pub fn check(&self, required: Capabilities) -> bool {
        self.allowed.contains(required)
    }

    pub fn enforce(&self, required: Capabilities) -> Result<(), String> {
        if self.check(required) {
            Ok(())
        } else {
            Err(format!("Security Violation: Actor requires {:?} capabilities", required))
        }
    }
}
