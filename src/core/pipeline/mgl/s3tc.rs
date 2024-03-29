pub enum Format {
    DXT1,
    DXT3,
    DXT5,
}

impl Format {
    pub fn gl_format(&self) -> u32 {
        match self {
            Self::DXT1 => gl::COMPRESSED_RGB_S3TC_DXT1_EXT,
            Self::DXT3 => gl::COMPRESSED_RGBA_S3TC_DXT3_EXT,
            Self::DXT5 => gl::COMPRESSED_RGBA_S3TC_DXT5_EXT,
        }
    }
}

pub struct S3MipmapDesc {
    pub offset: usize,
    pub size: usize,
    pub width: i32,
    pub height: i32,
}

pub struct S3MipmapView<'a> {
    pub width: i32,
    pub height: i32,
    pub data: &'a [u8],
}

pub struct Image {
    pub width: i32,
    pub height: i32,
    pub linear_size: i32,
    pub format: Format,
    pub block_size: u32,
    pub data: Vec<u8>,
    pub mipmaps: Vec<S3MipmapDesc>,
}

pub type MipmapDescIter<'a> = std::slice::Iter<'a, S3MipmapDesc>;

pub struct S3MipmapIter<'a> {
    pub data: &'a Vec<u8>,
    pub desc_iter: MipmapDescIter<'a>,
}

impl<'a> S3MipmapIter<'a> {
    pub fn new(data: &'a Vec<u8>, desc_iter: MipmapDescIter<'a>) -> Self {
        Self {
            data: data,
            desc_iter: desc_iter,
        }
    }
}

impl<'a> Iterator for S3MipmapIter<'a> {
    type Item = S3MipmapView<'a>;

    fn next(&mut self) -> Option<S3MipmapView<'a>> {
        match self.desc_iter.next() {
            Some(d) => Some(S3MipmapView {
                width: d.width,
                height: d.height,
                data: &self.data[d.offset..d.offset + d.size],
            }),
            None => None,
        }
    }
}

#[derive(Debug)]
#[allow(dead_code)]
pub enum ImageError {
    UnsupportedCompression,
    InvalidData(String),
}

impl Image {
    pub fn from_dds_buffer(mut header: Vec<u8>) -> Result<Image, ImageError> {
        macro_rules! get_u32 {
            ($buffer:ident, $idx:expr) => {
                u32::from_le_bytes([
                    $buffer[$idx],
                    $buffer[$idx + 1],
                    $buffer[$idx + 2],
                    $buffer[$idx + 3],
                ])
            };
        }

        // FIXME: MOVE THIS BACK
        // benchmark! {
        //     "loading_buffer";
        //     let buffer = &app.buffer_loader.load_bytes(Path::new(uri)).unwrap()
        // }
        // TODO:
        // * Check if file ident matches
        // * Generate mipmap descriptors

        let ident = String::from_utf8(header[0..3].to_vec()).unwrap();
        if ident.as_str() != "DDS" {
            return Err(ImageError::InvalidData(
                "DDS Ident does not much!".to_owned(),
            ));
        }

        let buffer = header.drain(128..).collect();

        let width = get_u32!(header, 12);
        let height = get_u32!(header, 16);
        let linear_size = get_u32!(header, 20);
        let mipmap_count = get_u32!(header, 28);
        let four_cc = String::from_utf8(header[84..88].to_vec()).unwrap();

        println!("DXT format: {}", four_cc);
        println!("DXT image size: {},{}", width, height);
        println!("DXT mipmap count: {}", mipmap_count);

        let (format, block_size) = match four_cc.as_str() {
            "DXT1" => (Format::DXT1, 8),
            "DXT2" => (Format::DXT3, 16),
            "DXT5" => (Format::DXT5, 16),
            _ => panic!("Unsupported DXT format! ({})", four_cc),
        };

        let mut mipmaps = vec![];
        let mut mip_w = width;
        let mut mip_h = height;
        let mut mip_offset = 0;

        for _ in 0..mipmap_count {
            let mip_size = (((mip_w + 3) / 4) * ((mip_h + 3) / 4) * block_size) as usize;

            mipmaps.push(S3MipmapDesc {
                offset: mip_offset,
                size: mip_size,
                width: mip_w as i32,
                height: mip_h as i32,
            });

            mip_offset += mip_size;
            mip_w /= 2;
            mip_h /= 2;
        }

        Ok(Image {
            width: width as i32,
            height: height as i32,
            linear_size: linear_size as i32,
            format: format,
            block_size: block_size,
            data: buffer,
            mipmaps: mipmaps,
        })
    }

    pub fn mipmap_iter(&self) -> S3MipmapIter {
        S3MipmapIter::new(&self.data, self.mipmaps.iter())
    }
}
