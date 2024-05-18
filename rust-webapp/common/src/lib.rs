pub mod config;

pub mod build {
    include!(concat!(env!("OUT_DIR"), "/built.rs"));
}
