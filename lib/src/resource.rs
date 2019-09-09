use crate::resource::ResourceError::ResourceStorageIsFile;
use image::{DynamicImage, ImageError, ImageFormat, ImageOutputFormat};
use rev_lines::RevLines;
use same_file::is_same_file;
use std::fs::{File, FileType, OpenOptions};
use std::future::Future;
use std::io;
use std::io::{BufReader, BufWriter, Read, Write};
use std::path::{Path, PathBuf};
use std::sync::Mutex;
use uuid::Uuid;

pub struct ResourceStorage {
    pub(crate) storage_loc: PathBuf,
    index_loc: PathBuf,
}

#[derive(Copy, Clone, Debug)]
pub enum Format {
    BMP,
    PNG,
    JPEG(u8),
}

fn safe_filename(path: &Path) -> Option<String> {
    if let Some(filename) = path.file_name() {
        match filename.to_str() {
            Some(f) => Some(f.to_string()),
            _ => None,
        }
    } else {
        None
    }
}

fn safe_extension(path: &Path) -> Option<String> {
    if let Some(extension) = path.extension() {
        match extension.to_str() {
            Some(ext) => Some(ext.to_string()),
            _ => None,
        }
    } else {
        None
    }
}

impl Format {
    fn from_path(path: &Path) -> Option<Self> {
        match safe_extension(path) {
            Some(e) => Self::from_extension(e.as_str()),
            None => None,
        }
    }

    fn from_extension(extension: &str) -> Option<Self> {
        let lowercase = extension.to_lowercase();
        match lowercase.as_str() {
            "bmp" => Some(Self::BMP),
            "png" => Some(Self::PNG),
            "jpg" => Some(Self::JPEG(75u8)),
            _ => None,
        }
    }

    fn is_valid_extension(extension: &str) -> bool {
        let lowercase = extension.to_lowercase();
        match lowercase.as_str() {
            "bmp" | "png" | "jpg" => true,
            _ => false,
        }
    }

    fn get_file_extension(&self) -> &str {
        match self {
            Self::BMP => "bmp",
            Self::PNG => "png",
            Self::JPEG(_) => "jpg",
        }
    }

    fn get_image_format(&self) -> image::ImageOutputFormat {
        match self {
            Self::BMP => ImageOutputFormat::BMP,
            Self::PNG => ImageOutputFormat::PNG,
            Self::JPEG(quality) => ImageOutputFormat::JPEG(*quality),
        }
    }
}

#[derive(Debug)]
pub enum ResourceError {
    ResourceStorageIsFile,
    CreateFailed(io::Error),
}

#[derive(Debug)]
pub enum GetRandomError {
    DirReadFailed(io::Error),
    FileReadFailed(io::Error),
}

#[derive(Debug)]
pub enum GetError {
    ReadFailed(image::ImageError),
}

#[derive(Debug)]
pub enum PostError {
    AlreadyExists(PathBuf),
    NotWritable(io::Error),
    WriteFailed(image::ImageError),
    IndexWriteFailed(io::Error),
    InvalidSource(PathBuf),
}

#[derive(Debug)]
pub enum ContainsError {
    NotFile(PathBuf),
    ComparisonError(io::Error),
    ReadError(io::Error),
}

impl ResourceStorage {
    pub(crate) fn new(storage_loc: PathBuf) -> Result<ResourceStorage, ResourceError> {
        use std::str::FromStr;

        if storage_loc.is_file() {
            return Err(ResourceError::ResourceStorageIsFile);
        }

        if !storage_loc.exists() {
            std::fs::create_dir_all(storage_loc.as_path())
                .map_err(|e| ResourceError::CreateFailed((e)))?
        }

        let index_loc = Self::index_loc(storage_loc.as_path());

        let storage = ResourceStorage {
            storage_loc,
            index_loc,
        };

        if !storage.index_loc.exists() {
            let open = OpenOptions::new()
                .write(true)
                .create(true)
                .append(true)
                .open(&storage.index_loc)
                .map_err(|e| ResourceError::CreateFailed(e))?;
            let mut open = BufWriter::new(open);

            // synchronize it with current contents of the dir
            for entry in storage
                .storage_loc
                .read_dir()
                .map_err(|e| ResourceError::CreateFailed(e))?
            {
                let path = match entry {
                    Ok(e) => e,
                    Err(e) => {
                        continue;
                    }
                }
                .path();

                let filename = match safe_filename(&path) {
                    Some(t) => t,
                    None => {
                        continue;
                    }
                };

                if Format::from_path(path.as_path()).is_some() {
                    let mut parts = filename.split(".");
                    let id = parts.next().unwrap();
                    if id.len() != 36 {
                        continue;
                    }

                    let write = filename.to_string() + "\n";
                    if Uuid::parse_str(&id).is_ok() {
                        open.write(write.as_bytes())
                            .map(|e| ())
                            .map_err(|e| ResourceError::CreateFailed(e))?
                    }
                }
            }
        }

        Ok(storage)
    }

    //    pub fn get_random(&self) -> Result<Uuid, GetRandomError> {
    //        let files = self.storage_loc.read_dir()
    //            .map_err(|e| GetRandomError::DirReadFailed(e))?;
    //
    //        let mut vec = Vec::new();
    //        for file in files {
    //            let file = file.map_err(|e| GetRandomError::FileReadFailed(e))?;
    //            let filename = file.file_name().into_string()
    //                .map_err(|e| GetRandomError::FileReadFailed(e))?;
    //
    //            vec.push()
    //        }
    //        let files = files
    //            .map(|f| )
    //    }

    pub fn id() -> Uuid {
        Uuid::new_v4()
    }

    pub fn stream<'a>(&'a self, name: &str) -> ResourceStream<'a> {
        ResourceStream::new(&self, name)
    }

    pub fn contains_file(&self, reference: &Path) -> Result<bool, ContainsError> {
        if !reference.exists() {
            return Ok(false);
        }

        if !reference.is_file() {
            return Err(ContainsError::NotFile(reference.to_path_buf()));
        }

        for file in self
            .storage_loc
            .read_dir()
            .map_err(|e| ContainsError::ReadError(e))?
        {
            let entry = file.map_err(|e| ContainsError::ReadError(e))?;
            if entry.path().is_file()
                && is_same_file(entry.path(), reference)
                    .map_err(|e| ContainsError::ComparisonError(e))?
            {
                return Ok(true);
            }
        }

        Ok(false)
    }

    pub fn accepts(&self, reference: &Path) -> bool {
        if !reference.exists() {
            return false;
        }

        if !reference.is_file() {
            return false;
        }

        match image::open(reference) {
            Ok(_) => true,
            Err(_) => false,
        }
    }

    pub(in crate::resource) fn contains(&self, key: Uuid, format: Format) -> bool {
        let path = self.path(key, format);
        path.is_file()
    }

    pub fn get_file(&self, filename: &str) -> Result<DynamicImage, GetError> {
        let mut path = self.storage_loc.clone();
        path.push(filename);
        image::open(path).map_err(|e| GetError::ReadFailed(e))
    }

    pub fn get(&self, key: Uuid, format: Format) -> Result<DynamicImage, GetError> {
        let path = self.path(key, format);
        image::open(path).map_err(|e| GetError::ReadFailed(e))
    }

    pub fn post_file(&self, key: Uuid, source: &Path) -> Result<PathBuf, PostError> {
        if !source.exists() || !source.is_file() {
            return Err(PostError::InvalidSource(source.to_path_buf()));
        }

        let format = Format::from_extension(source.extension().unwrap().to_str().unwrap());
        if !format.is_some() {
            return Err(PostError::InvalidSource(source.to_path_buf()));
        }

        let format = format.unwrap();

        let path = self.path(key, format);

        let result_path = std::fs::copy(source, path.as_path())
            .map(|u| path.clone())
            .map_err(|e| PostError::NotWritable(e));

        self.append_index(path.file_name().unwrap().to_str().unwrap())
            .map_err(|e| PostError::IndexWriteFailed(e))?;

        result_path
    }

    pub fn post(
        &self,
        key: Uuid,
        value: DynamicImage,
        format: Format,
    ) -> Result<PathBuf, PostError> {
        let path = self.path(key, format.clone());
        if path.exists() {
            return Err(PostError::AlreadyExists(path.clone()));
        }

        println!("Writing to path: {:?}", path.as_path());
        let mut file = File::create(path.as_path()).map_err(|e| PostError::NotWritable(e))?;

        value
            .write_to(&mut file, format.get_image_format())
            .map_err(|e| PostError::WriteFailed(e))?;

        self.append_index(path.file_name().unwrap().to_str().unwrap())
            .map_err(|e| PostError::IndexWriteFailed(e));

        Ok(path)
    }

    fn append_index(&self, filename: &str) -> Result<usize, std::io::Error> {
        let mut open = OpenOptions::new()
            .write(true)
            .append(true)
            .open(&self.index_loc)?;

        let filename = filename.to_string() + "\n";

        open.write(filename.as_bytes())
    }

    pub(in crate::resource) fn path(&self, uuid: Uuid, format: Format) -> PathBuf {
        let mut path = self.storage_loc.clone();
        path.push(uuid.to_hyphenated().to_string() + "." + format.get_file_extension());
        path
    }

    fn index_loc(path: &Path) -> PathBuf {
        let mut buf = path.to_path_buf();
        buf.push(".dali-resource.txt");
        buf
    }
}

pub struct ResourceStream<'a> {
    storage: &'a ResourceStorage,
    name: String,
}

#[derive(Debug)]
pub enum StreamUpdateError {
    InvalidFilename,
    WriteError(io::Error),
}

impl<'a> ResourceStream<'a> {
    pub(crate) fn new(storage: &'a ResourceStorage, name: &str) -> ResourceStream<'a> {
        ResourceStream {
            storage,
            name: name.to_string(),
        }
    }

    pub fn update(&self, uuid: Uuid, format: Format) -> Result<(), StreamUpdateError> {
        let path = self.storage.path(uuid, format);
        match safe_filename(path.as_path()) {
            Some(t) => self
                .update_file(t.as_str())
                .map(|e| ())
                .map_err(|e| StreamUpdateError::WriteError(e)),
            None => Err(StreamUpdateError::InvalidFilename),
        }
    }

    fn update_file(&self, filename: &str) -> Result<usize, io::Error> {
        let mut write = File::create(&self.index_loc())?;
        write.write(filename.as_bytes())
    }

    fn new_items(&self) -> Result<Vec<(Uuid, Format)>, io::Error> {
        let index_loc = self.index_loc();

        let mut filename = None;
        if index_loc.is_file() {
            let read = File::open(&index_loc)?;
            let mut reader = BufReader::new(read);
            let mut f = "".to_string();
            reader.read_to_string(&mut f)?;
            filename = Some(f);
        };

        self.new_items_since(filename)
    }

    fn new_items_since(&self, pos: Option<String>) -> Result<Vec<(Uuid, Format)>, io::Error> {
        let index = File::open(self.storage.index_loc.as_path())?;
        let rev_lines = RevLines::new(BufReader::new(index)).unwrap();

        let mut vec = Vec::new();
        let mut latest_file = None;

        for filename in rev_lines {
            if let Some(ref pos_file) = pos {
                if pos_file.trim() == filename.trim() {
                    break;
                }
            }

            if latest_file.is_none() {
                latest_file = Some(filename.clone());
            }

            let mut split = filename.split(".");
            let uuid = split.next().unwrap();
            let ext = split.next().unwrap();

            let uuid = match Uuid::parse_str(uuid) {
                Ok(uuid) => uuid,
                Err(e) => {
                    continue;
                }
            };

            let extension = match Format::from_extension(ext) {
                Some(f) => f,
                None => {
                    continue;
                }
            };

            if !self.storage.contains(uuid, extension) {
                // we allow users to delete files, so continue
                continue;
            }

            vec.push((uuid, extension));
        }

        vec.reverse();
        Ok(vec)
    }

    fn index_loc(&self) -> PathBuf {
        let filename = ".dali-stream-".to_string() + self.name.as_str() + ".txt";
        let mut index_loc = self.storage.storage_loc.clone();
        index_loc.push(filename);
        index_loc
    }
}

impl<'a> IntoIterator for &'a ResourceStream<'a> {
    type Item = (Uuid, Format);
    type IntoIter = std::vec::IntoIter<(Uuid, Format)>;

    fn into_iter(self) -> Self::IntoIter {
        let items = match self.new_items() {
            Ok(v) => v.into_iter(),
            Err(e) => {
                println!("Dali stream error: {}", e);
                Vec::new().into_iter()
            }
        };

        items.into_iter()
    }
}
