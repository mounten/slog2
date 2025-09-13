use slog2_parse::{ParseFlags, parse_all};

fn main() {
    parse_all(Some(ParseFlags::DYNAMIC), None, None, |info| {
        println!("{:?}", info);
        Ok(())
    })
    .expect("oh no :/");
}
