import librosa.filters
import numpy as np

SAMPLE_RATE = 48000
N_BINS = 4096

filterbank = librosa.filters.chroma(SAMPLE_RATE, N_BINS, n_chroma=36)  # , octwidth=None)
filterbank = np.roll(filterbank, -5, 0)  # Necessary due to a bug in librosa

with open('../src/filterbank.rs', 'w') as f:
    f.write(f'pub const FILTER_MATRIX: [f32; {filterbank.shape[0]} * {filterbank.shape[1]}] = ')
    f.write(list(filterbank.flatten()).__repr__() + ';')
