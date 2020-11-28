#[cfg(feature = "yep")]
pub mod test;

fn main() {
    println!("Hello, world!");
    #[cfg(feature = "yep")]
    println!("{}", test::test());
}
