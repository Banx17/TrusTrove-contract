pub const TTL_THRESHOLD: u32 = trusttrove_ttl::THRESHOLD;
pub const TTL_EXTEND_TO: u32 = trusttrove_ttl::EXTEND_TO;

/// Minimum initial deposit the factory passes to a newly deployed pool
/// instance's `initialize`. Mirrors `trusttrove_pool::DEFAULT_MIN_INITIAL_DEPOSIT`;
/// duplicated here rather than imported because `trusttrove-pool` is a
/// dev-dependency only (see Cargo.toml), so production code cannot reference
/// its constants. Callers deploying a pool for an asset with different
/// decimals than this default assumes should register it via
/// `register_existing_pool` instead, after initializing it directly with a
/// suitable `min_initial_deposit`.
pub const DEFAULT_MIN_INITIAL_DEPOSIT: u128 = 10_000_000;
