#![feature(proc_macro, wasm_custom_section, wasm_import_module)]

extern crate wasm_bindgen;
extern crate chfft;
#[macro_use]
extern crate ndarray;

use chfft::RFft1D;
use ndarray::prelude::*;
use ndarray::Array2;
use wasm_bindgen::prelude::*;

mod filterbank;

#[wasm_bindgen]
pub struct Context {
    fft: RFft1D<f32>,
    filter: Array2<f32>,
    result: [f32; 12],
}

#[wasm_bindgen]
impl Context {
    pub fn new() -> Context {
        Context {
            fft: RFft1D::<f32>::new(2048),
            filter: Array::from_shape_vec((36, 1025), filterbank::FILTER_MATRIX.to_vec()).unwrap(),
            result: [0.0; 12],
        }
    }

    pub fn process_audio(&mut self, buf: &[f32]) {
        // Do FFT
        let bins = self.fft.forward(&buf);

        // Square FFT coeffs
        let bins = bins.iter().map(|x| x.norm_sqr()).collect();
        let bins = Array::from_vec(bins);

        // Load filter matrix as ndarray
        let chroma = self.filter.dot(&bins);

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

        for i in 0..12 {
            self.result[i] = reshaped_chroma[[i, b_index]] - (0.25 * (reshaped_chroma[[i, a_index]] - reshaped_chroma[[i, c_index]]) * p);
        }
    }

    pub fn result_ptr(&self) -> *const f32 {
        self.result.as_ptr()
    }

    pub fn result_len(&self) -> usize {
        self.result.len()
    }
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