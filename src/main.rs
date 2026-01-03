use g1t::VfsWrapper;
use std::path::Path;

fn main() {
    let mut safe_fs = VfsWrapper::new(vfs::PhysicalFS::new("./"));
    let mut file = safe_fs.create_file(&Path::new("hello/world/test.txt"));
    file.write(b"test");
    println!("already_created: {}", file.already_created());
}
