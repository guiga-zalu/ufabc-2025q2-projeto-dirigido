use core::{ops::Add, time::Duration};
use std::{fs, io::BufReader, path::Path, process::Command, time::Instant};

use image::{DynamicImage, ImageEncoder, ImageFormat};

#[derive(Clone)]
pub struct Codec {
    pub name: String,
    pub func: Box<fn(&DynamicImage, &Path) -> Compression>,
}

impl Codec {
    pub fn new(name: String, func: fn(&DynamicImage, &Path) -> Compression) -> Codec {
        Codec {
            name,
            func: Box::new(func),
        }
    }
    pub fn apply(&self, image: &DynamicImage, temp_file: &Path) -> Compression {
        (self.func)(image, temp_file)
    }
}

impl Add<(&DynamicImage, &Path)> for &Codec {
    type Output = Compression;

    fn add(self, (img, temp_file): (&DynamicImage, &Path)) -> Self::Output {
        (self.func)(img, temp_file)
    }
}

pub struct Compression {
    pub stream_size: u64,
    pub time_spent: Duration,
    pub image_if_lossy: Option<DynamicImage>,
}

pub fn png(img: &DynamicImage, temp_file: &Path) -> Compression {
    let temp_file = temp_file.with_extension("png");
    let now = Instant::now();
    img.save_with_format(&temp_file, ImageFormat::Png).unwrap();
    let time_spent = now.elapsed();

    let stream_size = fs::metadata(&temp_file).unwrap().len();
    fs::remove_file(temp_file).unwrap();
    Compression {
        stream_size,
        time_spent,
        image_if_lossy: None,
    }
}

pub fn qoi(img: &DynamicImage, temp_file: &Path) -> Compression {
    let temp_file = temp_file.with_extension("qoi");
    let now = Instant::now();
    img.save_with_format(&temp_file, ImageFormat::Qoi).unwrap();
    let time_spent = now.elapsed();

    let stream_size = fs::metadata(&temp_file).unwrap().len();
    fs::remove_file(temp_file).unwrap();
    Compression {
        stream_size,
        time_spent,
        image_if_lossy: None,
    }
}

pub fn avif(img: &DynamicImage, temp_file: &Path, quality: u8) -> Compression {
    let temp_file = temp_file.with_extension("avif");
    use image::{
        codecs::avif::{AvifDecoder, AvifEncoder},
        ExtendedColorType,
    };

    let time_spent = {
        let file = fs::File::options()
            .write(true)
            .create(true)
            .open(&temp_file)
            .unwrap();
        let encoder = AvifEncoder::new_with_speed_quality(&file, 5, quality);

        let now = Instant::now();
        encoder
            .write_image(
                img.to_rgb8().as_raw(),
                img.width(),
                img.height(),
                ExtendedColorType::Rgb8,
            )
            .unwrap();
        now.elapsed()
    };

    let (stream_size, img) = {
        let file = fs::File::open(&temp_file).unwrap();
        let stream_size = file.metadata().unwrap().len();
        let decoder = AvifDecoder::new(file).unwrap();
        let img = DynamicImage::from_decoder(decoder).unwrap();
        (stream_size, img)
    };
    fs::remove_file(temp_file).unwrap();

    Compression {
        stream_size,
        time_spent,
        image_if_lossy: Some(img),
    }
}

pub fn webp(img: &DynamicImage, quality: Option<f32>) -> Compression {
    use webp::{Decoder, Encoder};

    let now = Instant::now();
    let encoder = Encoder::from_image(img).unwrap();
    let encoded = if let Some(q) = quality {
        encoder.encode(q)
    } else {
        encoder.encode_lossless()
    };
    let time_spent = now.elapsed();

    let stream_size = encoded.len() as u64;
    Compression {
        stream_size,
        time_spent,
        image_if_lossy: quality.map(|_| Decoder::new(&encoded).decode().unwrap().to_image()),
    }
}

pub fn jpeg(img: &DynamicImage, temp_file: &Path, quality: u8) -> Compression {
    let temp_file = temp_file.with_extension("jpg");
    use image::codecs::jpeg::{JpegDecoder, JpegEncoder};
    let time_spent = {
        fs::remove_file(&temp_file).unwrap_or_default();
        let file = fs::File::create_new(&temp_file).unwrap();
        let mut encoder = JpegEncoder::new_with_quality(&file, quality);

        let now = Instant::now();
        encoder.encode_image(img).unwrap();
        now.elapsed()
    };

    let file = fs::File::open(&temp_file).unwrap();
    let stream_size = file.metadata().unwrap().len();

    let buf = BufReader::new(file);
    let decoder = JpegDecoder::new(buf).unwrap();
    let img = DynamicImage::from_decoder(decoder).unwrap();

    fs::remove_file(temp_file).unwrap();
    Compression {
        stream_size,
        time_spent,
        image_if_lossy: Some(img),
    }
}

pub fn png_quant(img: &DynamicImage, temp_file: &Path, ncolors: u16) -> Compression {
    let temp_file = temp_file.with_extension("png");
    let temp_file_2 = temp_file.with_extension("new.png");

    let time_spent = Instant::now();
    img.save_with_format(&temp_file, ImageFormat::Png).unwrap();
    {
        let mut cmd = Command::new("pngquant");
        cmd.args([
            "--speed",
            "1",
            "--ext",
            ".new.png",
            ncolors.to_string().as_str(),
            temp_file.to_str().unwrap(),
        ]);
        let mut child = cmd.spawn().expect("Expected pngquant to be installed");
        child.wait().unwrap();
    }
    let time_spent = time_spent.elapsed();

    // dbg!(&temp_file);
    // dbg!(&temp_file_2);
    let stream_size = fs::metadata(&temp_file_2).unwrap().len();
    let img = image::open(&temp_file_2).unwrap();
    fs::remove_file(temp_file).unwrap();
    fs::remove_file(temp_file_2).unwrap();
    Compression {
        stream_size,
        time_spent,
        image_if_lossy: Some(img),
    }
}
