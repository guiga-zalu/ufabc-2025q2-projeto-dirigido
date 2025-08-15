#![feature(thread_id_value)]
#![allow(unused_imports)]
use comparador::{
    codecs::{self, Codec},
    metrics::{
        self,
        hash::{self, HashMetric, ImageHash},
        Metric,
    },
    traits::Comparison,
    utils::{RwHashMap, Writer},
};

use std::{
    cell::{Cell, UnsafeCell},
    collections::HashMap,
    fs,
    io::{self, BufReader, BufWriter, Write},
    num::NonZeroU64,
    ops::Deref,
    path::{Path, PathBuf},
    sync::{Arc, LazyLock, Mutex, RwLock},
    thread,
};

use clap::Parser;
use globwalk::glob;
use image::{DynamicImage, ImageReader};
use rayon::prelude::*;
use simple_tqdm::{Config, Tqdm};

#[derive(Parser, Debug, Default)]
#[command(author, version, about, long_about = None)]
pub struct CliArgs {
    // Files
    /// The dataset folder
    #[arg(short, long)]
    dataset: PathBuf,
    /// The temp. folder
    #[arg(short, long, default_value_t = String::from("temp"))]
    temp_folder: String,
}

static HASHES: LazyLock<Arc<[HashMetric<u64>]>> = LazyLock::new(|| {
    use comparador::metrics::hash::*;
    let hash_a = HashMetric::new(String::from("A Hash"), {
        move |image: &DynamicImage| AHash::hash(image)
    });
    let hash_d = HashMetric::new(String::from("D Hash"), {
        move |image: &DynamicImage| DHash::hash(image)
    });
    let hash_p = HashMetric::new(String::from("P Hash"), {
        move |image: &DynamicImage| PHash::hash(image)
    });
    [hash_a, hash_d, hash_p].into_iter().collect()
});

static METRICS: LazyLock<Arc<[Metric<f64>]>> = LazyLock::new(|| {
    use comparador::metrics::*;
    let mae = Metric::new(String::from("MAE"), {
        move |original: &DynamicImage, other: &DynamicImage| MAE::compare(original, other)
    });
    let mse = Metric::new(String::from("MSE"), {
        move |original: &DynamicImage, other: &DynamicImage| MSE::compare(original, other)
    });
    let ssim = Metric::new(String::from("SSIM"), {
        move |original: &DynamicImage, other: &DynamicImage| {
            SSIM::compare(&original.to_luma16(), &other.to_luma16())
        }
    });
    let ms_ssim = Metric::new(String::from("MS SSIM"), {
        move |original: &DynamicImage, outra: &DynamicImage| {
            MultiScaleSSIM::compare(original, outra)
        }
    });
    let gmsm = Metric::new(String::from("GMSM"), {
        move |original: &DynamicImage, outra: &DynamicImage| GMSM::compare(original, outra)
    });
    let gmsd = Metric::new(String::from("GMSD"), {
        move |original: &DynamicImage, outra: &DynamicImage| GMSD::compare(original, outra)
    });
    [mae, mse, ssim, ms_ssim, gmsm, gmsd].into_iter().collect()
});

static CODECS: LazyLock<Arc<[Codec]>> = LazyLock::new(|| {
    use comparador::codecs::*;

    // Lossless
    let png = Codec::new(String::from("PNG"), codecs::png);
    let qoi = Codec::new(String::from("QOI"), codecs::qoi);
    let webp_lossless = Codec::new(String::from("WEBP"), |image, _| codecs::webp(image, None));

    // Lossy
    // - 90%
    let webp_90 = Codec::new(String::from("WEBP (90%)"), |image, _| {
        codecs::webp(image, Some(90.0))
    });
    let jpeg_90 = Codec::new(String::from("JPEG (90%)"), |image, temp_file| {
        codecs::jpeg(image, temp_file, 90)
    });
    let avif_90 = Codec::new(String::from("AVIF (90%)"), |image, temp_file| {
        codecs::avif(image, temp_file, 90)
    });
    // - 80%
    let webp_80 = Codec::new(String::from("WEBP (80%)"), |image, _| {
        codecs::webp(image, Some(80.0))
    });
    let jpeg_80 = Codec::new(String::from("JPEG (80%)"), |image, temp_file| {
        codecs::jpeg(image, temp_file, 80)
    });
    let avif_80 = Codec::new(String::from("AVIF (80%)"), |image, temp_file| {
        codecs::avif(image, temp_file, 80)
    });
    // - 50%
    let webp_50 = Codec::new(String::from("WEBP (50%)"), |image, _| {
        codecs::webp(image, Some(50.0))
    });
    let jpeg_50 = Codec::new(String::from("JPEG (50%)"), |image, temp_file| {
        codecs::jpeg(image, temp_file, 50)
    });
    let avif_50 = Codec::new(String::from("AVIF (50%)"), |image, temp_file| {
        codecs::avif(image, temp_file, 50)
    });
    // - 15%
    let webp_15 = Codec::new(String::from("WEBP (15%)"), |image, _| {
        codecs::webp(image, Some(15.0))
    });
    let jpeg_15 = Codec::new(String::from("JPEG (15%)"), |image, temp_file| {
        codecs::jpeg(image, temp_file, 15)
    });
    let avif_15 = Codec::new(String::from("AVIF (15%)"), |image, temp_file| {
        codecs::avif(image, temp_file, 15)
    });

    let png_quant_128 = Codec::new(String::from("PNG (pngquant 128)"), |image, temp_file| {
        codecs::png_quant(image, temp_file, 128)
    });
    let png_quant_256 = Codec::new(String::from("PNG (pngquant 256)"), |image, temp_file| {
        codecs::png_quant(image, temp_file, 256)
    });

    [
        png,
        qoi,
        webp_lossless,
        webp_90,
        webp_80,
        webp_50,
        webp_15,
        jpeg_90,
        jpeg_80,
        jpeg_50,
        jpeg_15,
        avif_90,
        avif_80,
        avif_50,
        avif_15,
        png_quant_128,
        png_quant_256,
    ]
    .into_iter()
    .collect()
});

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = CliArgs::parse();
    let CliArgs {
        dataset,
        temp_folder,
    } = args;
    let log_folder: &str = "./logs";
    let dataset = dataset.to_str().unwrap().to_owned();
    let dataset = dataset + "/**/*.{avif,bmp,exr,gif,jpeg,jpg,ico,png,pnm,tga,tiff,qoi,webp}";

    // Create temp folder if not exists
    fs::create_dir_all(&temp_folder).unwrap_or_default();
    fs::create_dir_all(&log_folder).unwrap_or_default();

    let mut image_names = vec![];
    let glob_walker = glob(&dataset)?.filter_map(Result::ok);
    for entry in glob_walker {
        // dbg!(&entry);
        let path = entry.path();
        if path.is_file() {
            image_names.push(path.to_path_buf());
        }
    }
    assert_ne!(image_names.len(), 0, "No images found in dataset");
    let compiled = process_images(image_names, &temp_folder, log_folder);
    println!("{:?}", compiled);

    // Clean temp folder
    fs::remove_dir(&temp_folder).unwrap();

    Ok(())
}

fn process_images(image_names: Vec<PathBuf>, temp_folder: &str, log_folder: &str) {
    let _ = LazyLock::force(&HASHES);
    let _ = LazyLock::force(&METRICS);
    let _ = LazyLock::force(&CODECS);

    let writers: RwHashMap<u8, Writer> = RwLock::new(HashMap::new());

    let temp_folder: Arc<str> = Arc::from(temp_folder);
    let log_folder: Arc<str> = Arc::from(log_folder);
    image_names
        .into_iter()
        .map(|image_name| {
            // println!("Processing image: {}", image_name.display());
            (image_name, HASHES.clone())
        })
        .tqdm_config(
            Config::new()
                .with_unit("img")
                .with_desc("Processing images")
                .with_progress_chars("@%#987654321 "),
        )
        .par_bridge()
        .for_each(|(image_name, hashes)| {
            // dbg!(&image_name);
            let temp_folder = temp_folder.clone();
            let writer: Writer = {
                let thread_num: u8 = u64::from(thread::current().id().as_u64()) as u8;
                // dbg!(thread_num);

                if !writers.read().unwrap().contains_key(&thread_num) {
                    // println!("Writer for thread {thread_num} not found. Creating...");
                    let log_file = log_folder.to_string() + &format!("/thread-{}.log", thread_num);
                    let file = fs::File::options()
                        .read(false)
                        .write(true)
                        .create(true)
                        .open(log_file)
                        .unwrap();
                    let writer: Writer = Arc::new(Mutex::new(BufWriter::new(file)));
                    // println!("Writer for thread {thread_num} successfully created.");
                    let mut binding_w = writers.write().unwrap();
                    binding_w.insert(thread_num, writer);
                    // println!("Writer for thread {thread_num} successfully inserted.");
                }
                writers.read().unwrap().get(&thread_num).unwrap().clone()
            };
            // dbg!(&writer);
            process_image(image_name, hashes, &temp_folder, writer).unwrap()
        });
}

fn process_image(
    image_name: PathBuf,
    hash_metrics: Arc<[HashMetric<u64>]>,
    temp_folder: &str,
    w: Writer,
) -> Result<(), io::Error> {
    let mut w = w.lock().unwrap();
    writeln!(w, "Image [{}]", image_name.display())?;
    let file = fs::File::options().read(true).open(&image_name)?;
    let original_size = file.metadata().unwrap().len() as f64;
    let original = ImageReader::new(BufReader::new(file))
        .with_guessed_format()
        .unwrap()
        .decode()
        .unwrap();

    let original_hashes = hash_metrics
        .iter()
        .map(|format| {
            let name: &str = &format.name;
            (name, format + &original)
        })
        .collect::<Vec<(&str, u64)>>();

    let codecs = CODECS.clone();
    for codec in codecs.into_iter() {
        let temp_file =
            temp_folder.to_string() + "/" + image_name.file_name().unwrap().to_str().unwrap();
        let compression = codec.apply(&original, &PathBuf::from(temp_file));
        if compression.is_none() {
            continue;
        }
        let compression = compression.unwrap();

        if let Some(other) = compression.image_if_lossy {
            writeln!(
                w,
                "Codec,{} (Lossy),{}b,{}mcs,{}%",
                codec.name,
                compression.stream_size,
                compression.time_spent.as_micros(),
                100.0 * compression.stream_size as f64 / original_size
            )?;

            for ((hash_name, hash_original), hash_metric) in
                original_hashes.iter().zip(hash_metrics.iter())
            {
                let hash_other = hash_metric + &other;
                writeln!(
                    w,
                    "Hash,{},{}%",
                    hash_name,
                    100.0 * u64::compare(hash_original, &hash_other)
                )?;
            }

            for metric in METRICS.iter() {
                let value = metric.apply(&original, &other);
                writeln!(w, "Metric,{},{}", metric.name, value)?;
            }
        } else {
            writeln!(
                w,
                "Codec,{} (Lossless),{}b,{}mcs,{}%",
                codec.name,
                compression.stream_size,
                compression.time_spent.as_micros(),
                100.0 * compression.stream_size as f64 / original_size
            )?;
        }
    }
    Ok(())
}
