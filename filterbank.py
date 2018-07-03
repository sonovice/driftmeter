import json

import librosa.filters
import numpy as np

SAMPLE_RATE = [48000]
N_BINS = [2048, 4096, 8192]
N_CHROMA = [36]


class NumpyEncoder(json.JSONEncoder):
    def default(self, obj):
        if isinstance(obj, np.ndarray):
            return obj.tolist()
        return json.JSONEncoder.default(self, obj)


for sample_rate in SAMPLE_RATE:
    for n_bins in N_BINS:
        for n_chroma in N_CHROMA:
            filterbank = librosa.filters.chroma(
                sample_rate, n_bins, n_chroma=n_chroma)  # , octwidth=None)
            filterbank = filterbank[:, :-1]  # exlude nyquist
            with open(f'filterbank_{sample_rate}_{n_bins}_{n_chroma}.js', 'w') as f:
                f.write("fb = " + json.dumps(filterbank, cls=NumpyEncoder))
