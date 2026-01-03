use g1t::SafeFsBuilder;

fn main() {
    let fs = SafeFsBuilder::open("./test").unwrap();
}
