#!/usr/bin/env bash
args=""
for f in $(ls ./audio_files/clips/*); do
	args="${args}${f} "
done

args=$(python -c "print('${args}'.strip())")

python load.py "${args}"
