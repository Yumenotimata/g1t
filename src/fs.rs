pub trait FileSystem {
    fn mkdir(&mut self, directory_name: impl Into<String>);
    fn touch(
        &mut self,
        file_name: impl Into<String>,
        content: impl Into<String>,
    );
}

// pub enum ByteStorage {
//     Index { entries: Vec<Entry> },
// }

// #[derive(Debug)]
// pub struct FileSystem {
//     datas: Vec<Data>,
// }

// impl FileSystem {
//     pub fn new() -> Self {
//         Self { datas: Vec::new() }
//     }

//     pub fn mkdir(
//         &mut self,
//         directory_name: impl Into<String>,
//         child_builder: impl FnOnce(&mut FileSystem),
//     ) -> &mut Self {
//         let directory_name = directory_name.into();
//         let mut child_file_system = FileSystem::new();
//         child_builder(&mut child_file_system);

//         self.datas
//             .push(Data::directory(directory_name, child_file_system.datas));

//         self
//     }

//     pub fn touch(
//         &mut self,
//         file_name: impl Into<String>,
//         content: impl Into<String>,
//     ) -> &mut Self {
//         self.datas
//             .push(Data::file(file_name.into(), content.into()));
//         self
//     }

//     pub fn build(self) -> Self {
//         self
//     }
// }

// #[derive(Debug)]
// enum Data {
//     File { name: String, content: String },
//     Directory { name: String, contents: Vec<Data> },
// }

// impl Data {
//     pub fn file(name: String, content: String) -> Self {
//         Self::File { name, content }
//     }

//     pub fn directory(name: String, contents: Vec<Data>) -> Self {
//         Self::Directory { name, contents }
//     }
// }
