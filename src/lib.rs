#[cfg(feature = "derive")]
pub use derive_my_trait::MyTrait;

pub trait MyTrait {
    fn as_i32(&self) -> i32 {
        42
    }
}
