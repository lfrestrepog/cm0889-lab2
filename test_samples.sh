#!/usr/bin/bash
output=$1

if [ ! -f $output ]; then
    echo 'New results file.'
    echo 'Sample layout	Sample size	Algorithm	Duration	Time units' > $output
fi

for algorithm in 'merge' 'quick' 'tail' 'bubble' 'selection' 'insertion' 'shell'; do
    for layout in 'random' 'nearly sorted' 'reversed' 'few unique'; do
        for size in 1000 2000 3000 5000 10000 20000 50000 100000 150000 500000 1000000; do
            echo "Trying algorithm ${algorithm} with size ${size} and layout ${layout}."
            timeout --preserve-status 10m target/release/cm0889-lab2 ${output} ${size} ${algorithm} "${layout}"
            r=$?
            if [ $r -ne 0 ]; then
                echo "${layout}	${size}	${algorithm}	error	$r" >> $output
                break
            fi
        done
    done
done

