use core::ops::{Add, Mul};
use std::{
    collections::HashMap,
    fs::File,
    hash::Hash,
    io::BufWriter,
    sync::{Arc, Mutex, RwLock},
};

use image::{imageops::FilterType, DynamicImage};

#[inline]
pub fn multiply3x3<T, U>(a: &[[T; 3]; 3], b: &[[T; 3]; 3]) -> [[U; 3]; 3]
where
    T: Copy + Mul<T, Output = U>,
    U: Add<Output = U>,
{
    [
        [
            a[0][0] * b[0][0] + a[0][1] * b[1][0] + a[0][2] * b[2][0],
            a[0][0] * b[0][1] + a[0][1] * b[1][1] + a[0][2] * b[2][1],
            a[0][0] * b[0][2] + a[0][1] * b[1][2] + a[0][2] * b[2][2],
        ],
        [
            a[1][0] * b[0][0] + a[1][1] * b[1][0] + a[1][2] * b[2][0],
            a[1][0] * b[0][1] + a[1][1] * b[1][1] + a[1][2] * b[2][1],
            a[1][0] * b[0][2] + a[1][1] * b[1][2] + a[1][2] * b[2][2],
        ],
        [
            a[2][0] * b[0][0] + a[2][1] * b[1][0] + a[2][2] * b[2][0],
            a[2][0] * b[0][1] + a[2][1] * b[1][1] + a[2][2] * b[2][1],
            a[2][0] * b[0][2] + a[2][1] * b[1][2] + a[2][2] * b[2][2],
        ],
    ]
}

#[inline]
pub fn conv3x3<T, U>(a: &[[T; 3]; 3], b: &[[T; 3]; 3]) -> U
where
    T: Copy + Mul<T, Output = U>,
    U: Add<Output = U>,
{
    a[0][0] * b[0][0]
        + a[0][1] * b[1][0]
        + a[0][2] * b[2][0]
        + a[1][0] * b[0][1]
        + a[1][1] * b[1][1]
        + a[1][2] * b[2][1]
        + a[2][0] * b[0][2]
        + a[2][1] * b[1][2]
        + a[2][2] * b[2][2]
}

pub fn dct<const N: usize, const M: usize>(array: [[u8; N]; N]) -> [[f64; M]; M] {
    use core::f64::consts::PI;

    let mut f = [[0_f64; M]; M];
    for (i, fi) in f.iter_mut().enumerate() {
        let mut row = [0_f64; M];
        for j in 0..M {
            let mut sum = 0_f64;

            for y in 0..N {
                for x in 0..N {
                    sum += (PI * i as f64 * ((x << 1) + 1) as f64 / (2.0 * N as f64)).cos()
                        * (PI * j as f64 * ((y << 1) + 1) as f64 / (2.0 * N as f64)).cos()
                        * array[x][y] as f64;
                }
            }

            sum *= if i == 0 { 1. / f64::sqrt(2.0) } else { 1. }
                * if j == 0 { 1. / f64::sqrt(2.0) } else { 1. }
                / 4.0;

            row[j] = sum;
        }

        *fi = row;
    }

    f
}

/// https://arxiv.org/pdf/1308.3052
pub fn gradient_magnitude_similarity(original: &DynamicImage, other: &DynamicImage) -> Vec<f64> {
    const C: f64 = 0.0026;

    let nwidth = original.width() >> 1;
    let nheight = original.height() >> 1;
    let filter = FilterType::Triangle;
    let original = original.resize_exact(nwidth, nheight, filter).to_luma16();
    let other = other.resize_exact(nwidth, nheight, filter).to_luma16();

    const _1_3: f64 = 1.0 / 3.0;
    const H_X: [[f64; 3]; 3] = [[_1_3, 0., -_1_3]; 3];
    const H_Y: [[f64; 3]; 3] = [[_1_3; 3], [0.; 3], [-_1_3; 3]];

    let len = (nwidth - 2) * (nheight - 2);
    let mut gms: Vec<f64> = Vec::with_capacity(len as usize);
    for y in 1..(nheight - 1) {
        for x in 1..(nwidth - 1) {
            let r: [[f64; 3]; 3] = [
                [
                    original.get_pixel(x - 1, y - 1).0[0] as f64,
                    original.get_pixel(x, y - 1).0[0] as f64,
                    original.get_pixel(x + 1, y - 1).0[0] as f64,
                ],
                [
                    original.get_pixel(x - 1, y).0[0] as f64,
                    original.get_pixel(x, y).0[0] as f64,
                    original.get_pixel(x + 1, y).0[0] as f64,
                ],
                [
                    original.get_pixel(x - 1, y + 1).0[0] as f64,
                    original.get_pixel(x, y + 1).0[0] as f64,
                    original.get_pixel(x + 1, y + 1).0[0] as f64,
                ],
            ];
            let d: [[f64; 3]; 3] = [
                [
                    other.get_pixel(x - 1, y - 1).0[0] as f64,
                    other.get_pixel(x, y - 1).0[0] as f64,
                    other.get_pixel(x + 1, y - 1).0[0] as f64,
                ],
                [
                    other.get_pixel(x - 1, y).0[0] as f64,
                    other.get_pixel(x, y).0[0] as f64,
                    other.get_pixel(x + 1, y).0[0] as f64,
                ],
                [
                    other.get_pixel(x - 1, y + 1).0[0] as f64,
                    other.get_pixel(x, y + 1).0[0] as f64,
                    other.get_pixel(x + 1, y + 1).0[0] as f64,
                ],
            ];
            let m_r = conv3x3(&r, &H_X).powi(2) + conv3x3(&r, &H_Y).powi(2);
            let m_d = conv3x3(&d, &H_X).powi(2) + conv3x3(&d, &H_Y).powi(2);

            let gms_i = (2.0 * (m_r * m_d).sqrt() + C) / (m_r + m_d + C);
            gms.push(gms_i);
        }
    }

    gms
}

pub type RwHashMap<K, V> = RwLock<HashMap<K, V>>;
pub type Writer = Arc<Mutex<BufWriter<File>>>;

pub fn ensure_key<K, V>(map: &RwHashMap<K, V>, key: K, value: V)
where
    K: Eq + Hash,
{
    let reader = map.read().unwrap();
    if reader.contains_key(&key) {
        return;
    }
    drop(reader);
    let mut writer = map.write().unwrap();
    writer.insert(key, value);
}

pub fn ensure_key_with<K, V, F>(map: &RwHashMap<K, V>, key: K, func: &F)
where
    K: Eq + Hash,
    F: Fn() -> V,
{
    let reader = map.read().unwrap();
    if reader.contains_key(&key) {
        return;
    }
    drop(reader);
    let mut writer = map.write().unwrap();
    writer.insert(key, func());
}
