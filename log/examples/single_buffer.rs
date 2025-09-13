use std::{env, sync::LazyLock};

use slog2::{debug1, info, notice, warning};

pub static __PROGNAME: LazyLock<String> = LazyLock::new(|| {
    env::current_exe()
        .ok()
        .and_then(|path| {
            path.file_name()
                .map(|name| name.to_string_lossy().into_owned())
        })
        .unwrap_or_else(|| "unknown".to_string())
});

fn main() {
    let mut buffer_config = slog2::BufferSetConfig::default();
    buffer_config
        .set_name(&__PROGNAME)
        .expect("could not set name");

    buffer_config.set_verbosity(slog2::Verbosity::Info);

    buffer_config
        .config_name("default")
        .expect("could not set name");
    buffer_config.set_num_pages(7);

    let [buffer_handle] = buffer_config
        .register(None)
        .expect("Could not register buffer config");

    // When setting a default buffer the log macros will use this one when no buffer argument is passed.
    slog2::Buffer::set_default_buffer(Some(buffer_handle));

    info!(
        code = 2000,
        "Writing a formatted string into the buffer: {}",
        env::args().next().unwrap_or_default()
    );

    info!("Writing a constant string.");

    let some_number = 5108;

    warning!("string: {}, some_number: {}", "hello world", some_number);

    // we still can pass a bufferr.
    notice!(buffer = buffer_handle, "This string will be logged.");
    notice!(
        buffer = slog2::Buffer::USE_DEFAULT,
        "This string will be logged aswell."
    );

    debug1!("This string should not be logged.");
}
