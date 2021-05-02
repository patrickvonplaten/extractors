#!/usr/bin/env python3
from pydub import AudioSegment
from datasets import load_dataset

audio_ds = load_dataset("common_voice", "ab", split="other")


def create_map_fn(fmt):

    def map_fn(batch):
        sound = AudioSegment.from_mp3(batch['path'])
        sound.export(".".join(batch['path'].split(".")[:-1] + [fmt]), format=fmt)

    return map_fn


audio_ds.map(create_map_fn("flac"))
