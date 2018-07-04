import librosa.filters

SAMPLE_RATE = 48000
N_BINS = 2048
N_CHROMA = 36

filterbank = librosa.filters.chroma(SAMPLE_RATE, N_BINS, n_chroma=N_CHROMA)  # , octwidth=None)
print(filterbank.shape)
with open('filterbank.rs', 'w') as f:
    f.write(f'pub const FILTER_MATRIX: [f32; {filterbank.shape[0]} * {filterbank.shape[1]}] = ')
    f.write(list(filterbank.flatten()).__repr__() + ';')
