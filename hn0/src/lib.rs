pub mod hna;
pub mod sensors;
mod status;
pub use crate::status::*;

/// Return own crate version. Used in API responses.
pub fn get_version() -> &'static str {
    env!("CARGO_PKG_VERSION")
}

#[cfg(test)]
mod test {
    use super::get_version;

    #[test]
    fn test_get_version() {
        let version = get_version();
        assert_eq!(3, version.split('.').count());
    }
}
