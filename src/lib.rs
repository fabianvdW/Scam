pub mod general_stuff;

#[cfg(feature = "varianta")]
pub mod varianta;
#[cfg(feature = "varianta")]
pub use varianta::*;
#[cfg(feature = "variantb")]
pub mod variantb;
#[cfg(feature = "variantb")]
pub use variantb::*;
