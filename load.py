#!/usr/bin/env python3
import torchaudio
import sys

paths = sys.argv[1].split(" ")

samples = []
for i, path in enumerate(paths):
    sample, sr = torchaudio.load(path)
    samples.append(sample)

for sample in samples:
    print(f"{i}: {sample.shape}")
