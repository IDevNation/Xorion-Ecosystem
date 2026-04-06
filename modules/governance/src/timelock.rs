//! Timelock controller — enforces a delay between proposal success and execution.

use crate::{GovernanceError, Result};

/// Timelock configuration.
pub struct Timelock {
    /// Minimum delay in seconds between queuing and execution.
    delay: u64,
}

impl Timelock {
    /// Create a new timelock with the given delay (seconds).
    pub fn new(delay_secs: u64) -> Self {
        Self { delay: delay_secs }
    }

    /// Return the configured delay.
    pub fn delay(&self) -> u64 {
        self.delay
    }

    /// Check whether a proposal queued at `queued_at` can be executed at `now`.
    pub fn can_execute(&self, queued_at: u64, now: u64) -> Result<()> {
        if now < queued_at + self.delay {
            let remaining = (queued_at + self.delay) - now;
            return Err(GovernanceError::TimelockActive {
                remaining_secs: remaining,
            });
        }
        Ok(())
    }

    /// Return the earliest timestamp at which execution is allowed.
    pub fn execution_time(&self, queued_at: u64) -> u64 {
        queued_at + self.delay
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn execution_blocked_during_delay() {
        let tl = Timelock::new(3600); // 1 hour
        assert!(tl.can_execute(1000, 2000).is_err()); // only 1000s passed
    }

    #[test]
    fn execution_allowed_after_delay() {
        let tl = Timelock::new(3600);
        assert!(tl.can_execute(1000, 5000).is_ok()); // 4000s > 3600s
    }

    #[test]
    fn execution_allowed_at_exact_delay() {
        let tl = Timelock::new(100);
        assert!(tl.can_execute(1000, 1100).is_ok());
    }

    #[test]
    fn execution_time_calculation() {
        let tl = Timelock::new(7200);
        assert_eq!(tl.execution_time(5000), 12200);
    }

    #[test]
    fn zero_delay_always_executable() {
        let tl = Timelock::new(0);
        assert!(tl.can_execute(1000, 1000).is_ok());
    }
}
