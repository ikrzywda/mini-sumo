use core::fmt::Write;
use heapless::String;

pub fn dupa(s: &mut String<64>) {
    writeln!(s, "DUPA FUPA").unwrap();
}
