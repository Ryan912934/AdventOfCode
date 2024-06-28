#!/bin/sh
i=0
while [ $i -ne 9 ]
do
        i=$(($i+1))
        echo "$i"
        cargo new "day-0$i" --bin
        
done
while [ $i -ne 25 ]
do
        i=$(($i+1))
        echo "$i"
        cargo new "day-$i" --bin
        
done