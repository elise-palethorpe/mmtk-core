pub mod block;
pub mod chunk;
pub mod defrag;
pub mod immixspace;
pub mod line;

pub use immixspace::*;

/// Mark/sweep memory for block-level only
pub const BLOCK_ONLY: bool = true;

/// Opportunistic copying
pub const DEFRAG: bool = false;

/// Mark lines when scanning objects.
/// Otherwise, do it at mark time.
pub const MARK_LINE_AT_SCAN_TIME: bool = false;

macro_rules! validate {
    ($x: expr) => { assert!($x, stringify!($x)) };
    ($x: expr => $y: expr) => { if $x { assert!($y, stringify!($x implies $y)) } };
}

fn validate_features() {
    validate!(DEFRAG => !BLOCK_ONLY);
}
