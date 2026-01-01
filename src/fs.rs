pub trait FileSystem {
    fn mkdir(&mut self, directory_name: impl Into<String>);
    fn touch(
        &mut self,
        file_name: impl Into<String>,
        content: impl Into<String>,
    );
}

// impl Data {
//     pub fn file(name: String, content: String) -> Self {
//         Self::File { name, content }
//     }

//     pub fn directory(name: String, contents: Vec<Data>) -> Self {
//         Self::Directory { name, contents }
//     }
// }
