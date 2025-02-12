fn main() {
    let env = std::env::vars();
    
    // Begin checks
    println!(r#"cargo:rustc-check-cfg=cfg(CONFIG_DISABLE_MULTIBOOT2_SUPPORT, values("true", "false", none()))"#);

    println!(r#"cargo:rustc-check-cfg=cfg(CONFIG_HALT_ON_PANIC, values("true", "false", none()))"#);
    println!(r#"cargo:rustc-check-cfg=cfg(CONFIG_SPIN_ON_PANIC, values("true", "false", none()))"#);

    println!(r#"cargo:rustc-check-cfg=cfg(CONFIG_PREUSER_EXIT_LOOP_ON_INVALID_LENGTH, values("true", "false", none()))"#);
    println!(r#"cargo:rustc-check-cfg=cfg(CONFIG_PREUSER_PANIC_ON_INVALID_LENGTH, values("true", "false", none()))"#);
    println!(r#"cargo:rustc-check-cfg=cfg(CONFIG_PREUSER_WARN_ON_INVALID_LENGTH, values("true", "false", none()))"#);
    println!(r#"cargo:rustc-check-cfg=cfg(CONFIG_PREUSER_ERROR_ON_INVALID_LENGTH, values("true", "false", none()))"#);

    println!(r#"cargo:rustc-check-cfg=cfg(CONFIG_PREUSER_OUTPUT_DEBUG, values("true", "false", none()))"#);
    println!(r#"cargo:rustc-check-cfg=cfg(CONFIG_PREUSER_OUTPUT_INFO, values("true", "false", none()))"#);
    println!(r#"cargo:rustc-check-cfg=cfg(CONFIG_PREUSER_OUTPUT_WARN, values("true", "false", none()))"#);
    println!(r#"cargo:rustc-check-cfg=cfg(CONFIG_PREUSER_OUTPUT_ERROR, values("true", "false", none()))"#);
    println!(r#"cargo:rustc-check-cfg=cfg(CONFIG_PREUSER_OUTPUT_FATAL, values("true", "false", none()))"#);

    println!(r#"cargo:rustc-check-cfg=cfg(CONFIG_BUILD_GRUB, values("true", "false", none()))"#);
    // End checks

    // Configuration name used when a config is required but should always evaluate to true
    println!(r#"cargo:rustc-check-cfg=cfg(NONE, values("false", none()))"#);

    for (var, val) in env {
        if !var.starts_with("CONFIG_") {
            continue
        }
        println!("cargo:rerun-if-env-changed={}", var);
        println!("cargo:rustc-cfg={}=\"{}\"", var, val);
    }
}