fn main() {
    // Rust's target configuration selects the static CRT for MSVC. Disable
    // Tauri's empty msvcrt.lib shim: its native search path leaks into other
    // workspace crates' rustdoc invocations without the matching linker args.
    tauri_build::try_build(
        tauri_build::Attributes::new()
            .windows_attributes(tauri_build::WindowsAttributes::new().static_vc_runtime(false)),
    )
    .expect("failed to build Tauri application resources");
}
