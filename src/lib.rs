mod roc_ffi {
    // Disable linter for generated code
    #![allow(non_camel_case_types, dead_code, non_upper_case_globals)]
    #![allow(clippy::all, clippy::pedantic, clippy::nursery)]
    include!(concat!(env!("OUT_DIR"), "/bindings.rs"));
}

pub mod sender;
mod context;
mod config;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
    }
}
