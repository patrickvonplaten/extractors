#!/usr/bin/env python3
import torchaudio
import sys
from os import listdir
from os.path import isfile, join

audio_dir = sys.argv[1]
paths = [join(audio_dir, f) for f in listdir(audio_dir) if isfile(join(audio_dir, f))]

resampler = torchaudio.transforms.Resample(48000, 16000)

samples = []
for i, path in enumerate(paths):
    sample, sr = torchaudio.load(path)
    sample = resampler(sample)
    samples.append(sample)

print("Len", len(samples))
