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
    let mut buffer_config = slog2::BufferSetConfig::<2>::default();
    buffer_config
        .set_name(&__PROGNAME)
        .expect("could not set name");

    buffer_config.set_verbosity(slog2::Verbosity::Info);

    buffer_config[0]
        .config_name("hi_rate_logging")
        .expect("could not set name");
    buffer_config[0].set_num_pages(7);

    buffer_config[1]
        .config_name("lo_rate_logging")
        .expect("could not set name");
    buffer_config[1].set_num_pages(1);

    let buffer_handle = buffer_config
        .register(None)
        .expect("Could not register buffer config");

    info!(
        buffer = buffer_handle[0],
        "Writing a formatted string into the buffer: {}",
        env::args().next().unwrap_or_default()
    );

    info!(
        buffer = buffer_handle[0],
        code = 2000,
        "Writing a formatted string into the buffer: {}",
        env::args().next().unwrap_or_default()
    );

    info!(buffer = buffer_handle[0], "Writing a constant string.");

    let some_number = 5108;

    warning!(
        code = 3000,
        buffer = buffer_handle[0],
        "string: {}, some_number: {}",
        "hello world",
        some_number
    );

    // /* Write something to the 'lo rate' buffer (i.e., buffer 1). */
    notice!(buffer = buffer_handle[1], "This string will be logged.");

    debug1!(
        buffer = buffer_handle[0],
        "This string should not be logged."
    );
}
