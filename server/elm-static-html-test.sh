#!/bin/bash
# https://www.cyberciti.biz/faq/bash-for-loop/
# http://stackoverflow.com/questions/3795470/how-do-i-get-just-real-time-value-from-time-command/3795634#3795634
TIMEFORMAT=%R
for i in {1..20}
do
   #echo "Welcome $i times"
   time curl http://localhost:8000/template > /dev/null
done
