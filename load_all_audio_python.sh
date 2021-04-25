#!/usr/bin/env bash
args=""
for f in $(ls ./audio_files/wav/*); do
	args="${args}${f} "
done

#args=$(python -c "print('${args}'.strip())")
#
#echo ${args}
#
#exit

python load.py "${args}"
