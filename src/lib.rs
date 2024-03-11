pub mod api;
pub mod foxbit;
pub mod helpers;
pub mod types;

pub use foxbit::Foxbit;

/// Creates a new instance of Foxbit.
pub fn new_hello_crate() -> Foxbit {
    Foxbit::new("base_url".into(), "api_key".into(), "access_key".into())
}
