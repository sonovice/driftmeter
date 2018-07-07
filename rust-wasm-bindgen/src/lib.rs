#![feature(proc_macro, wasm_custom_section, wasm_import_module)]

extern crate wasm_bindgen;
extern crate chfft;
extern crate ndarray;

use chfft::RFft1D;
use ndarray::{prelude::*, Array2};
use wasm_bindgen::prelude::*;

mod filterbank;

#[wasm_bindgen]
pub struct DriftMeter {
    fft: RFft1D<f32>,
    filterbank: Array2<f32>,
    hpcp: [f32; 12],
}

#[wasm_bindgen]
impl DriftMeter {
    pub fn new() -> DriftMeter {
        let filterbank_length = filterbank::FILTER_MATRIX.len();
        DriftMeter {
            fft: RFft1D::<f32>::new((filterbank_length / 36 - 1) * 2),
            filterbank: Array::from_shape_vec((36, filterbank_length / 36), filterbank::FILTER_MATRIX.to_vec()).unwrap(),
            hpcp: [0.0; 12],
        }
    }

    pub fn calc_offset(&mut self, buf: &[f32]) -> f32 {
        // Do FFT
        let bins = self.fft.forward(&buf);

        // Square FFT coeffs
        let bins = bins.iter().map(|x| x.norm_sqr()).collect();
        let bins = Array::from_vec(bins);

        // Load filter matrix as ndarray
        let mut chroma = self.filterbank.dot(&bins);

        // Normalize
        let chroma_length = f32::sqrt(chroma.mapv(|x| x.powi(2)).scalar_sum());
        chroma /= chroma_length;


        // Calculate HPCPs
        let reshaped_chroma = chroma.into_shape((12, 3)).unwrap();
        let energy = reshaped_chroma.sum_axis(Axis(0)).to_vec();

        let (b_index, b) = argmax(&energy);
        // REMEMBER: '%' is the remainder operator, NOT modulo!
        let a_index = (b_index + 3 - 1) % 3;
        let c_index = (b_index + 3 + 1) % 3;

        let a = energy[a_index];
        let c = energy[c_index];

        let p = 0.5 * (a - c) / (a - (2. * b) + c);

        for i in 0..12 {
            self.hpcp[i] = reshaped_chroma[[i, b_index]] - (0.25 * (reshaped_chroma[[i, a_index]] - reshaped_chroma[[i, c_index]]) * p);
        }

        let offset = ((b_index as f32 + 1.0) * 0.25 + (p / 2.0)) - 0.5;

        offset
    }

    pub fn hpcp_ptr(&self) -> *const f32 {
        self.hpcp.as_ptr()
    }

    pub fn fft_window(&self) -> usize {
        (filterbank::FILTER_MATRIX.len() / 36 - 1) * 2
    }
}

fn argmax<T>(u: &[T]) -> (usize, T)
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