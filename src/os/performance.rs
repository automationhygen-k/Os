#[derive(Clone, Debug)]
pub enum PowerProfile {
    Silent,
    Balanced,
    Creator,
    Beast,
}

#[derive(Clone, Debug)]
pub struct RuntimeTuning {
    pub worker_threads: usize,
    pub io_priority_boost: bool,
    pub frame_latency_target_ms: u16,
}

#[derive(Clone)]
pub struct PerformanceDirector {
    pub profile: PowerProfile,
    pub thread_budget: usize,
}

impl PerformanceDirector {
    pub fn new() -> Self {
        let logical = std::thread::available_parallelism()
            .map(|n| n.get())
            .unwrap_or(4);
        Self {
            profile: PowerProfile::Balanced,
            thread_budget: logical,
        }
    }

    pub fn apply_profile(&mut self, profile: PowerProfile) -> RuntimeTuning {
        self.profile = profile;
        let cores = self.thread_budget.max(2);

        match self.profile {
            PowerProfile::Silent => RuntimeTuning {
                worker_threads: (cores / 2).max(1),
                io_priority_boost: false,
                frame_latency_target_ms: 24,
            },
            PowerProfile::Balanced => RuntimeTuning {
                worker_threads: (cores * 3 / 4).max(2),
                io_priority_boost: false,
                frame_latency_target_ms: 16,
            },
            PowerProfile::Creator => RuntimeTuning {
                worker_threads: (cores - 1).max(2),
                io_priority_boost: true,
                frame_latency_target_ms: 12,
            },
            PowerProfile::Beast => RuntimeTuning {
                worker_threads: cores,
                io_priority_boost: true,
                frame_latency_target_ms: 8,
            },
        }
    }

    pub fn describe(&self) -> String {
        format!(
            "profile={:?}, logical_threads={}, strategy=efficient saturation",
            self.profile, self.thread_budget
        )
    }
}
