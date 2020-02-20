
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

macro_rules! prepare_full_path {

    ($root:expr, $i:ident) => {
        {
            if !$i.is_relative() {
                return Err( io::Error::new(
                    io::ErrorKind::InvalidInput,
                    format!("\"{}\" : Is not a relative path (BufferLoader requires relative path)!", $i.display())
                ));
            }

            $root.join($i)
        }
    }

}

impl BufferLoader {




    pub fn relative_to_exe() -> io::Result<Self> {
        let exe_path = env::current_exe()?;
        Ok (Self {root: exe_path.parent().unwrap().to_path_buf()})
    }

    pub fn with_root(root: &Path) -> io::Result<Self> {

        if ! root.is_dir() {
            return Err(io::Error::new (
                io::ErrorKind::NotFound,
                format!("\"{}\" : Invalid or non-existing directory path!", root.display())
            ));
        }

        Ok(Self {root: root.to_path_buf()})
    }


    pub fn load_bytes(&self, file_path: &Path) -> io::Result<Vec<u8>> {

        let full_path = prepare_full_path!(self.root, file_path);

        let full_path = self.root.join(file_path);
        let mut file = File::open(full_path)?;
        let mut data = vec![];
        file.read_to_end(&mut data)?;
        Ok(data)

    }

    pub fn load_string(&self, file_path: &Path) -> io::Result<String> {

        let full_path = prepare_full_path!(self.root, file_path);

        let mut file = File::open(full_path)?;
        let mut data = String::new();
        file.read_to_string(&mut data)?;
        Ok(data)
    }

    pub fn load_cstring(&self, file_path: &Path) -> io::Result<CString> {

        // FIXME: Optimize this
        let data = {
            let s = self.load_string(file_path)?;
             CString::new(s.as_str())?
        };

        Ok(data)
    }

}
