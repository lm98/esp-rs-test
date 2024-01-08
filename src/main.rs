use rf_core::context::Context;

fn main() {
    // It is necessary to call this function once. Otherwise some patches to the runtime
    // implemented by esp-idf-sys might not link properly. See https://github.com/esp-rs/esp-idf-template/issues/71
    esp_idf_svc::sys::link_patches();

    // Bind the log crate to the ESP Logging facilities
    esp_idf_svc::log::EspLogger::initialize_default();

    let context = Context::new(
        0, 
        Default::default(), 
        Default::default(), 
        Default::default()
    );
    log::info!("Hello, world! Context id is {}", context.self_id());
}
