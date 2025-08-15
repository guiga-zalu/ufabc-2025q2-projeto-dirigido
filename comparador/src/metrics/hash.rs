/// Vou ignorar esse daqui: https://github.com/jaehl/blockhash
///
/// Os códigos presentes contam com conversão para escala de cinza,
///  e então redimensionamento, pois de acordo com a [evidência anedótica do autor](https://www.hackerfactor.com/blog/index.php?/archives/529-Kind-of-Like-That.html#c2094),
///  essa é a ordem mais rápida.
///
use crate::{traits::Comparison, utils::dct};

use core::ops::Add;
use std::sync::Arc;

use image::{imageops::FilterType, DynamicImage};

pub trait ImageHash {
    fn hash(image: &DynamicImage) -> u64;
}

impl Comparison<u64> for u64 {
    fn compare(original_hash: &u64, other_hash: &u64) -> f64 {
        // 0 ^ 0 = 0
        // 0 ^ 1 = 1
        // 1 ^ 0 = 1
        // 1 ^ 1 = 0
        let difference = original_hash ^ other_hash;
        difference.count_ones() as f64 / 64.
    }
}

#[derive(Clone)]
pub struct HashMetric<Result> {
    pub name: String,
    pub func: Arc<fn(&DynamicImage) -> Result>,
}

impl<Result> HashMetric<Result>
where
    Result: 'static,
{
    pub fn new(name: String, func: fn(&DynamicImage) -> Result) -> HashMetric<Result> {
        HashMetric {
            name,
            func: Arc::new(func.clone()),
        }
    }
    pub fn apply(&self, image: &DynamicImage) -> Result {
        (self.func)(image)
    }
}

impl<Result> Add<&DynamicImage> for &HashMetric<Result> {
    type Output = Result;

    fn add(self, rhs: &DynamicImage) -> Self::Output {
        (self.func)(rhs)
    }
}

/// https://www.hackerfactor.com/blog/index.php?/archives/432-Looks-Like-It.html
pub struct AHash;

impl ImageHash for AHash {
    fn hash(image: &DynamicImage) -> u64 {
        const TAMANHO: usize = 8;
        let luma8 = image
            .grayscale()
            .resize_exact(TAMANHO as u32, TAMANHO as u32, FilterType::Lanczos3)
            .into_luma8();
        let array: [[u8; TAMANHO]; TAMANHO] = luma8
            .into_raw()
            .chunks_exact(TAMANHO)
            .map(|c| c.try_into().unwrap())
            .collect::<Vec<[u8; TAMANHO]>>()
            .try_into()
            .unwrap();
        let mut sum: f64 = 0.0;
        for j in 0..TAMANHO {
            sum += array[j][0..TAMANHO].iter().map(|v| *v as f64).sum::<f64>();
        }
        let avg = sum / (TAMANHO * TAMANHO) as f64;
        let mut hash = 0u64;
        for j in 0..TAMANHO {
            for i in 0..TAMANHO {
                if array[j][i] as f64 > avg {
                    hash |= 1 << (j * TAMANHO + i);
                }
            }
        }
        hash
    }
}

/// https://www.hackerfactor.com/blog/index.php?/archives/432-Looks-Like-It.html
pub struct PHash;

impl ImageHash for PHash {
    fn hash(image: &DynamicImage) -> u64 {
        const TAMANHO: usize = 32;
        const TAMANHO_MENOR: usize = 8;
        let luma8 = image
            .grayscale()
            .resize_exact(TAMANHO as u32, TAMANHO as u32, FilterType::Lanczos3)
            .into_luma8();
        let array: [[u8; TAMANHO]; TAMANHO] = luma8
            .into_raw()
            .chunks_exact(TAMANHO)
            .map(|c| c.try_into().unwrap())
            .collect::<Vec<[u8; TAMANHO]>>()
            .try_into()
            .unwrap();
        let low_freqs: [[f64; TAMANHO_MENOR]; TAMANHO_MENOR] = dct(array);
        let mut sum: f64 = 0.0;
        for j in 0..TAMANHO_MENOR {
            sum += low_freqs[j][0..TAMANHO_MENOR].iter().sum::<f64>();
        }
        let avg = sum / (TAMANHO_MENOR * TAMANHO_MENOR) as f64;
        let mut hash = 0u64;
        for j in 0..TAMANHO_MENOR {
            for i in 0..TAMANHO_MENOR {
                if low_freqs[j][i] > avg {
                    hash |= 1 << (j * TAMANHO_MENOR + i);
                }
            }
        }
        hash
    }
}

/// https://www.hackerfactor.com/blog/index.php?/archives/529-Kind-of-Like-That.html
/// A publicação contém comentários do `marcan`, o mesmo que fez parte do Asahi Linux!
pub struct DHash;

impl ImageHash for DHash {
    fn hash(image: &DynamicImage) -> u64 {
        const TAMANHO: usize = 8;
        let luma8 = image
            .grayscale()
            .resize_exact(TAMANHO as u32 + 1, TAMANHO as u32, FilterType::Lanczos3)
            .into_luma8();
        let array: Vec<[u8; TAMANHO]> = luma8
            .into_raw()
            .chunks_exact(TAMANHO)
            .map(|c| c.try_into().unwrap())
            .collect::<Vec<[u8; TAMANHO]>>();
        let mut hash = 0u64;
        for j in 0..TAMANHO {
            for i in 0..TAMANHO - 1 {
                if array[j][i] > array[j][i + 1] {
                    hash |= 1 << (j * TAMANHO + i);
                }
            }
        }
        hash
    }
}
