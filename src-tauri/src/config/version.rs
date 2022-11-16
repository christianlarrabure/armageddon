/*
  GENERAL SOFTWARE VERSION

  In this section, we will store only functions and
  constants related to the version of the software.
*/

pub const SOFTWARE_NAME: &str = "The Unofficial Armageddon Client";
pub const MAJOR_VERSION: i32 = 0;
pub const MINOR_VERSION: i32 = 2;

pub fn get_version() -> String {
    return format!("{}.{}", MAJOR_VERSION, MINOR_VERSION);
}

pub fn get_software_name() -> String {
    let version = get_version();
    return format!("{} v{}", SOFTWARE_NAME, version);
}
