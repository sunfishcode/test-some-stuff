use std::fs::File;

pub fn main() {
    let _ = dbg!(File::open("tmp/file"));
    let _ = dbg!(File::open("tmp/dir/file"));
    let _ = dbg!(File::open("tmp/dir\\file"));
    let _ = dbg!(File::open("tmp/dir\\\\file"));
    let _ = dbg!(File::open("/tmp/file"));
    let _ = dbg!(File::open("\\tmp/file"));
    let _ = dbg!(File::open("\\\\tmp/file"));
}
