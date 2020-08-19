
use std::path::{Path, PathBuf};
use std::env;
use std::io;
use std::fs::File;
use std::io::Read;
use std::string::String;
use std::ffi::CString;

pub struct BufferLoader {
    root: PathBuf,
}

#[derive(Debug)]
pub enum BufferLoaderError {
    IoError {
        io_error: io::Error,
        file_path: Option<PathBuf>,
    },
    NulError(std::ffi::NulError)
}

type BufferLoaderResult<T> = Result<T, BufferLoaderError>;

fn buffer_load_err<T>(p: Option<PathBuf>, e: io::Error) -> BufferLoaderResult<T> {
    Err( BufferLoaderError::IoError {
        file_path: p.map_or(None, |a| Some(PathBuf::from(a))),
        io_error: e,
    })
}

impl From<std::io::Error> for BufferLoaderError {
    fn from(err: std::io::Error) -> BufferLoaderError {
        Self::IoError {
            io_error: err,
            file_path: None,
        }
    }
}

impl From<std::ffi::NulError> for BufferLoaderError {
    fn from(err: std::ffi::NulError) -> BufferLoaderError {
        Self::NulError (err)
    }
}

macro_rules! prepare_full_path {

    ($root:expr, $i:ident) => {
        {
            if !$i.is_relative() {
                return buffer_load_err(Some($root), io::Error::new(
                    io::ErrorKind::InvalidInput,
                    format!("\"{}\" : Is not a relative path (BufferLoader requires relative path)!", $i.display())
                ));
            }

            $root.join($i)
        }
    }

}

impl BufferLoader {

    pub fn relative_to_exe() -> BufferLoaderResult<Self> {
        let exe_path = env::current_exe().unwrap();
        Ok (Self {root: exe_path.parent().unwrap().to_path_buf()})
    }

    #[allow(dead_code)]
    pub fn with_root(r: PathBuf) -> BufferLoaderResult<Self> {

        if ! r.is_dir() {
            let msg = format!("\"{}\" : Invalid or non-existing directory path!", r.display());
            return buffer_load_err(Some(r), io::Error::new (
                io::ErrorKind::NotFound,
                msg
            ));
        }

        Ok(Self {root: r})
    }


    pub fn load_bytes(&self, file_path: &Path) -> BufferLoaderResult<Vec<u8>> {

        let full_path = self.root.join(prepare_full_path!(self.root.clone(), file_path));
        let mut file = File::open(full_path).unwrap();
        let mut data = vec![];
        file.read_to_end(&mut data).unwrap();
        Ok(data)

    }

    pub fn load_string(&self, file_path: &Path) -> BufferLoaderResult<String> {

        let full_path = prepare_full_path!(self.root.clone(), file_path);

        let mut file = {
            let f = File::open(full_path);

            match f {
                Err(e) => {
                    let mut pb = PathBuf::new();
                    pb.push(file_path);
                    return buffer_load_err(Some(pb), e);
                },
                Ok(file) => file
            }
        };

        let mut data = String::new();
        file.read_to_string(&mut data).unwrap();
        Ok(data)
    }

    pub fn load_cstring(&self, file_path: &Path) -> BufferLoaderResult<CString> {

        // FIXME: Optimize this
        let data = {
            let s = self.load_string(file_path)?;
             CString::new(s.as_str()).unwrap()
        };

        Ok(data)
    }

}
