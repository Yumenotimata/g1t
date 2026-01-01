#[derive(Debug)]
pub struct VFS {
    datas: Vec<Data>,
}

impl VFS {
    pub fn new() -> Self {
        Self { datas: Vec::new() }
    }
}

impl VFS {
    pub fn mkdir(
        &mut self,
        directory_name: impl Into<String>,
        child_builder: impl FnOnce(&mut VFS),
    ) -> &mut Self {
        let directory_name = directory_name.into();
        let mut child_file_system = VFS::new();
        child_builder(&mut child_file_system);

        self.datas
            .push(Data::directory(directory_name, child_file_system.datas));

        self
    }

    pub fn touch(
        &mut self,
        file_name: impl Into<String>,
        content: impl Into<String>,
    ) -> &mut Self {
        self.datas
            .push(Data::file(file_name.into(), content.into()));
        self
    }
}

#[derive(Debug)]
pub enum Data {
    File { name: String, content: String },
    Directory { name: String, contents: Vec<Data> },
}

impl Data {
    pub fn file(name: String, content: String) -> Self {
        Self::File { name, content }
    }

    pub fn directory(name: String, contents: Vec<Data>) -> Self {
        Self::Directory { name, contents }
    }
}
