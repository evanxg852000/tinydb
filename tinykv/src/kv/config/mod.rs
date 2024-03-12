use std::time::Duration;

#[derive(Debug, Clone, Copy)]
pub enum DiskSize {
    KiB(u64),
    MiB(u64),
}

impl DiskSize {
    pub fn num_bytes(&self) -> u64 {
        match self {
            DiskSize::KiB(n) => n * 1024,
            DiskSize::MiB(n) => n * 1024 * 1024,
        }
    }
}

pub struct Config {
    pub store_addr: String,
    pub is_raft: bool,
    pub scheduler_addr: String,

    // directory to store the data in.
    // should exist and be writable.
    pub db_path: String,

    // raft_base_tick_interval is a base tick interval (ms).
    pub raft_base_tick_interval: Duration,
    pub raft_heart_beat_ticks: Duration,
    pub raft_election_timeout_ticks: Duration,

    // interval to garbage collect unnecessary raft log (ms).
    pub raft_log_gc_tick_interval: Duration,
    // when entry count exceed this value, gc will be forced trigger.
    pub raft_log_gc_count_limit: u64, 

    // interval (ms) to check wether a region need to be split of not.
    pub split_region_check_tick_interval: Duration,
    // delay time before deleting a stale peer
    pub scheduler_heartbeat_tick_interval: Duration,
    pub scheduler_store_heartbeat_tick_interval: Duration,

    // when region [a, e) size reaches region_max_size, it will be split into
    // several regions [a, b), [b,c), [c, d), [d, e), and the size [a, b), [b,c),
    // [c, d) will be region_split_size (maybe a little larger).
    pub region_max_size: DiskSize,
    pub region_split_size: DiskSize,
}

impl Default for Config {
    fn default() -> Self {
        Self { 
            store_addr: "127.0.0.1:20160".to_string(), 
            is_raft: true, 
            scheduler_addr: "127.0.0.1:2379".to_string(), 
            db_path: "/tmp/data".to_string(), 
            raft_base_tick_interval: Duration::from_secs(1), 
            raft_heart_beat_ticks: Duration::from_secs(2), 
            raft_election_timeout_ticks: Duration::from_millis(10), 
            raft_log_gc_tick_interval: Duration::from_secs(10), 
            raft_log_gc_count_limit: 128_000, // assume the average size of entries is 1k.
            split_region_check_tick_interval: Duration::from_secs(10), 
            scheduler_heartbeat_tick_interval: Duration::from_secs(10),  
            scheduler_store_heartbeat_tick_interval: Duration::from_secs(10),
            region_max_size: DiskSize::MiB(144), 
            region_split_size: DiskSize::MiB(96), 
        }
    }
}

// Not necessary but exercising my macro skill
macro_rules! bail {
    ($arg:expr) => { 
        return Err($arg.to_string())
    };
}

impl Config {
    pub fn validate(&self) -> Result<(), String> {
        if self.raft_heart_beat_ticks.is_zero() {
           bail!("heartbeat tick must be greater that 0.");
        }

        if self.raft_election_timeout_ticks != Duration::from_secs(10) {
            bail!(
                "Election timeout ticks needs to be same across all the cluster, otherwise it may lead to inconsistency."
            );
        }

        if self.raft_election_timeout_ticks <= self.raft_heart_beat_ticks {
            bail!("election tick must be greater than heartbeat tick.")
        }
        Ok(())
    }

    #[cfg(test)]
    pub fn for_test() -> Self {
        Self {
            store_addr: "127.0.0.1:20160".to_string(), 
            is_raft: true, 
            scheduler_addr: "127.0.0.1:2379".to_string(), 
            db_path: "/tmp/data".to_string(), 
            raft_base_tick_interval: Duration::from_millis(50), 
            raft_heart_beat_ticks: Duration::from_secs(2), 
            raft_election_timeout_ticks: Duration::from_secs(10), 
            raft_log_gc_tick_interval: Duration::from_millis(50), 
            raft_log_gc_count_limit: 128_000, // assume the average size of entries is 1k.
            split_region_check_tick_interval: Duration::from_millis(100), 
            scheduler_heartbeat_tick_interval: Duration::from_millis(100),  
            scheduler_store_heartbeat_tick_interval: Duration::from_millis(500),
            region_max_size: DiskSize::MiB(144), 
            region_split_size: DiskSize::MiB(96), 
        }
    }
}


