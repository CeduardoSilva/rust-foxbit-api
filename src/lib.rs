pub mod api;
pub mod foxbit;
pub mod helpers;
pub mod types;

pub use foxbit::Foxbit;

/// Creates a new instance of Foxbit.
pub fn new() -> Foxbit {
    Foxbit::new()
}
