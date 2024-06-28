#!/bin/sh
i=0
while [ $i -ne 9 ]
do
        i=$(($i+1))
        mv "day-0$i" "day_0$i"    
done
while [ $i -ne 25 ]
do
        i=$(($i+1))
        mv "day-$i" "day_$i" 
        
done