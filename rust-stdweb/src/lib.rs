#![feature(proc_macro)]

#[macro_use]
extern crate stdweb;

extern crate chfft;

#[macro_use]
extern crate ndarray;

use stdweb::js_export;
use stdweb::web::TypedArray;
use chfft::RFft1D;

use ndarray::prelude::*;

mod filterbank;

#[js_export]
fn process_audio(buf: TypedArray<f32>) -> Vec<f32> {
    // Create FFT processor
    let mut fft: RFft1D<f32> = RFft1D::<f32>::new(2048);

    // Do FFT
    let bins = fft.forward(&buf.to_vec());

    // Square FFT coeffs
    let bins = bins.iter().map(|x| x.norm_sqr()).collect();
    let bins = Array::from_vec(bins);

    // Load filter matrix as ndarray
    let filter = Array::from_shape_vec((36, 1025), filterbank::FILTER_MATRIX.to_vec()).unwrap();
    let chroma = filter.dot(&bins);

    // Normalize
    let chroma_length = f32::sqrt(chroma.mapv(|x| x.powi(2)).scalar_sum());
    let chroma = chroma / chroma_length;

    // Reorder
    let chroma = stack![Axis(0), chroma.slice(s![5..]), chroma.slice(s![..5])];

    // Calculate HPCPs
    let reshaped_chroma = Array::from_shape_vec((12, 3), chroma.to_vec()).unwrap();
    let energy = reshaped_chroma.sum_axis(Axis(0)).to_vec();

    let (b_index, b) = argmax(&energy);
    // REMEMBER: '%' is the remainder operator, NOT modulo!
    let a_index = (b_index + 3 - 1) % 3;
    let c_index = (b_index + 3 + 1) % 3;

    let a = energy[a_index];
    let c = energy[c_index];

    let p = 0.5 * (a - c) / (a - (2. * b) + c);

    let mut hpcp: Vec<f32> = Vec::with_capacity(12);
    for i in 0..12 {
        hpcp.push(reshaped_chroma[[i, b_index]] - (0.25 * (reshaped_chroma[[i, a_index]] - reshaped_chroma[[i, c_index]]) * p));
    }

    hpcp
}


pub fn argmax<T>(u: &[T]) -> (usize, T)
    where T: Copy + PartialOrd
{
    assert_ne!(u.len(), 0);

    let mut max_index = 0;
    let mut max = u[max_index];

    for (i, v) in u.iter().enumerate().skip(1) {
        if max < *v {
            max_index = i;
            max = *v;
        }
    }

    (max_index, max)
}