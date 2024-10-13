#![doc = include_str!("../README.md")]

pub use adapters::*;

mod adapters;

/// Provides extension methods on types that can be adapted to different write
/// interfaces.
pub trait WriteExtension<T> {
    type Adapter;

    fn write_adapter(self) -> Self::Adapter;
}
