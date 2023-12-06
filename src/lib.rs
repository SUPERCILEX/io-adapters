pub mod adapters;

/// Provides extension methods on types that can be adapted to different write
/// interfaces.
pub trait WriteExtension<T> {
    type Adapter<'a>
    where
        Self: 'a;

    fn write_adapter(&mut self) -> Self::Adapter<'_>;
}
