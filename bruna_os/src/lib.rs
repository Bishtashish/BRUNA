// bruna_os/src/lib.rs
pub mod kernel;
pub mod hal;
pub mod drivers;
pub mod services;
pub mod comms;
pub mod utils;

// Add placeholder content or basic tests if idiomatic for new modules
#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}
