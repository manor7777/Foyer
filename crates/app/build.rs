fn main() {
    let cfg = slint_build::CompilerConfiguration::new().with_style("fluent".into());
    slint_build::compile_with_config("ui/main.slint", cfg).unwrap();

    // Embeds the app icon and version details in the exe (shown in Explorer and the taskbar).
    #[cfg(windows)]
    {
        let _ = embed_resource::compile("app.rc", embed_resource::NONE).manifest_optional();
    }
}
