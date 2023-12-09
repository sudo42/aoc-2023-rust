use std::env;

const DEBUG_ENV_NAME: &str = "DBG";

fn debug_from_env() -> &'static str {
    println!("{}={:?}", DEBUG_ENV_NAME, env::var(DEBUG_ENV_NAME));
    match env::var(DEBUG_ENV_NAME) {
        Ok(string) => match string.to_lowercase().as_str() {
            "1" | "true" | "yes" | "on" => "true",
            "0" | "false" | "no" | "off" => "false",
            _ => "default",
        },
        _ => "default",
    }
}

fn main() {
    println!("cargo:rerun-if-env-changed={}", DEBUG_ENV_NAME);
    println!(r#"cargo:rustc-cfg=WITH_DEBUG_SECTIONS="{}""#, debug_from_env());
    //println!(r#"cargo:rustc-cfg=WITH_DEBUG_SECTIONS="{}""#, "false");
}
