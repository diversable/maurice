pub mod config_file;
pub mod global_paths;
pub mod jsonstructs_versionsdb;
pub mod julialauncher;
pub mod versions_file;

pub fn get_juliaup_target() -> &'static str {
    JULIAUP_TARGET
}

pub fn get_bundled_julia_version() -> &'static str {
    BUNDLED_JULIA_VERSION
}
