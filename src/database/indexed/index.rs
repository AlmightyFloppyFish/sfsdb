pub enum File {
    Folder(Box<[File]>),
    File(FileData)
}

pub struct FileData {
    name: String,
    location: String,
    last_modified: String,
}
