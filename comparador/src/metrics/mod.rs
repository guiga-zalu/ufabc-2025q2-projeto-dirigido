pub mod hash;

use crate::{traits::Comparison, utils::gradient_magnitude_similarity};

use core::ops::Add;
use std::sync::Arc;

use image::{DynamicImage, GenericImageView, Luma, Pixel, Primitive};
use num_traits::cast::AsPrimitive;

#[derive(Clone)]
pub struct Metric<Result> {
    pub name: String,
    pub func: Arc<fn(&DynamicImage, &DynamicImage) -> Result>,
}

impl<Result> Metric<Result>
where
    Result: 'static,
{
    pub fn new(name: String, func: fn(&DynamicImage, &DynamicImage) -> Result) -> Metric<Result> {
        Metric {
            name,
            func: Arc::new(func.clone()),
        }
    }
    pub fn apply(&self, original: &DynamicImage, other: &DynamicImage) -> Result {
        (self.func)(original, other)
    }
}

impl<Result> Add<(&DynamicImage, &DynamicImage)> for Metric<Result> {
    type Output = Result;

    fn add(self, rhs: (&DynamicImage, &DynamicImage)) -> Self::Output {
        (self.func)(rhs.0, rhs.1)
    }
}

pub struct MAE;

impl<ImageType, PixelType, SubPixelType> Comparison<ImageType> for MAE
where
    ImageType: GenericImageView<Pixel = PixelType>,
    PixelType: Pixel<Subpixel = SubPixelType>,
    SubPixelType: AsPrimitive<f64> + Primitive,
{
    fn compare(original: &ImageType, other: &ImageType) -> f64 {
        let sum = original
            .pixels()
            .zip(other.pixels())
            .map(|((.., a), (.., b))| {
                a.channels()
                    .iter()
                    .zip(b.channels())
                    .map(|(&b, &a)| (b.as_() - a.as_()).abs())
                    .sum::<f64>()
            })
            .sum::<f64>();
        sum / ((original.width() * original.height()) as f64
            * SubPixelType::DEFAULT_MAX_VALUE.as_())
    }
}

pub struct MSE;

impl<ImageType, PixelType, SubPixelType> Comparison<ImageType> for MSE
where
    ImageType: GenericImageView<Pixel = PixelType>,
    PixelType: Pixel<Subpixel = SubPixelType>,
    SubPixelType: AsPrimitive<f64> + Primitive,
{
    fn compare(original: &ImageType, other: &ImageType) -> f64 {
        let sum = original
            .pixels()
            .zip(other.pixels())
            .map(|((.., a), (.., b))| {
                a.channels()
                    .iter()
                    .zip(b.channels())
                    .map(|(&b, &a)| (b.as_() - a.as_()).powi(2))
                    .sum::<f64>()
            })
            .sum::<f64>();
        sum / ((original.width() * original.height()) as f64
            * SubPixelType::DEFAULT_MAX_VALUE.as_().powi(2))
    }
}

pub struct SSIM;

impl<ImageType, SubPixelType> Comparison<ImageType> for SSIM
where
    ImageType: GenericImageView<Pixel = Luma<SubPixelType>>,
    SubPixelType: AsPrimitive<f64> + Primitive,
{
    fn compare(original: &ImageType, other: &ImageType) -> f64 {
        let c1 = (SubPixelType::DEFAULT_MAX_VALUE.as_() * 0.01).powi(2);
        let c2 = (SubPixelType::DEFAULT_MAX_VALUE.as_() * 0.03).powi(2);

        let len = original.width() * original.height();
        let k = 1.0 / len as f64;

        let mi_x = original
            .pixels()
            .fold(0f64, |acc, (.., p)| acc + p.0[0].as_())
            * k;
        let mi_y = other.pixels().fold(0f64, |acc, (.., p)| acc + p.0[0].as_()) * k;

        let var_x = original
            .pixels()
            .fold(0f64, |acc, (.., p)| acc + (p.0[0].as_() - mi_x).powi(2))
            * k;
        let var_y = other
            .pixels()
            .fold(0f64, |acc, (.., p)| acc + (p.0[0].as_() - mi_y).powi(2))
            * k;

        let cov_xy = original
            .pixels()
            .zip(other.pixels())
            .fold(0f64, |acc, ((.., p), (.., q))| {
                acc + (p.0[0].as_() - mi_x) * (q.0[0].as_() - mi_y)
            })
            * k;

        (2.0 * mi_x * mi_y + c1) * (2.0 * cov_xy + c2)
            / ((mi_x.powi(2) + mi_y.powi(2) + c1) * (var_x + var_y + c2))
    }
}

pub struct MultiScaleSSIM;

impl Comparison<DynamicImage> for MultiScaleSSIM {
    fn compare(original: &DynamicImage, other: &DynamicImage) -> f64 {
        let original = original.to_luma16();
        let other = other.to_luma16();
        const WINDOW_SIZE: u32 = 8;
        let width = original.width();
        let height = original.height();
        let bw = width.div_ceil(WINDOW_SIZE);
        let bh = height.div_ceil(WINDOW_SIZE);
        (0..bh)
            .map(|y| {
                let y = y * WINDOW_SIZE;
                let height = (height - y).min(WINDOW_SIZE);
                (0..bw)
                    .map(|x| {
                        let x = x * WINDOW_SIZE;
                        let width = (width - x).min(WINDOW_SIZE);

                        let sub_original = *original.view(x, y, width, height);
                        let sub_outra = *other.view(x, y, width, height);

                        SSIM::compare(&sub_original, &sub_outra)
                    })
                    .sum::<f64>()
            })
            .sum::<f64>()
            / (bw * bh) as f64
    }
}

/// https://arxiv.org/pdf/1308.3052
pub struct GMSM;

impl Comparison<DynamicImage> for GMSM {
    fn compare(original: &DynamicImage, other: &DynamicImage) -> f64 {
        let gms = gradient_magnitude_similarity(original, other);
        gms.iter().sum::<f64>() / gms.len() as f64
    }
}

/// https://arxiv.org/pdf/1308.3052
pub struct GMSD;

impl Comparison<DynamicImage> for GMSD {
    fn compare(original: &DynamicImage, other: &DynamicImage) -> f64 {
        let gms = gradient_magnitude_similarity(original, other);
        let _len = 1.0 / gms.len() as f64;
        let gmsm = gms.iter().sum::<f64>() * _len;
        (gms.iter().map(|gms_i| (gms_i - gmsm).powi(2)).sum::<f64>() * _len).sqrt()
    }
}
