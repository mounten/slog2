use std::env;

use slog2_parse::{LogFile, PacketInfo};

fn main() {
    let log_file = LogFile::open(&env::args().nth(1).expect("Couldn't not get argument."))
        .expect("Could not open file");

    let log_info = log_file
        .info()
        .expect("Couldn't get information about the log");
    println!("{:#?}", log_info);

    println!("Buffers:");
    for (index, buffer_info) in log_file.into_iter().enumerate() {
        let buffer_info = buffer_info.expect("Couldn't get info about buffer {index}.");
        println!("{:#?}", buffer_info);

        log_file
            .parse_static(index as i32, my_callback)
            .expect("LogFile::parse_static() failed");
    }
}

fn my_callback(info: PacketInfo) -> Result<(), i32> {
    println!(
        "Processing packet {} of size {}",
        info.sequence_number(),
        info.size()
    );
    println!("\t{:?}: {}", info.severity(), info.message().unwrap());
    Ok(())
}
