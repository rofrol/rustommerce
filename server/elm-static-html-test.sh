#!/bin/bash
# http://stackoverflow.com/questions/3795470/how-do-i-get-just-real-time-value-from-time-command/3795634#3795634
TIMEFORMAT=%R

END=$1
if [ -z "$END" ]; then
  END=2
fi

SSR=$2
if [ -z "$SSR" ]; then
  SSR=true
fi

# http://stackoverflow.com/questions/169511/how-do-i-iterate-over-a-range-of-numbers-defined-by-variables-in-bash/169602#169602
typeset -i i END # Let's be explicit
for ((i=1;i<=END;++i)); do
   time curl http://localhost:8000/template/${SSR} > /dev/null
done
