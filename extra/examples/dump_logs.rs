use std::fs::File;

use slog2_extra::{DumpFlags, dump_logs_to_file};

fn main() {
    let file = File::create("dumped_logs.log").expect("Couldn't create file");
    dump_logs_to_file(&file, Some(DumpFlags::DUMP_LOGS_ALL)).expect("Couldn't dump logs to file");
}
