#!/usr/bin/bash
output=$1

if [ ! -f $output ]; then
    echo 'New results file.'
    echo 'Sample layout	Sample size	Algorithm	Duration	Time units' > $output
fi

for algorithm in 'merge' 'quick' 'tail' 'bubble' 'selection' 'insertion' 'shell'; do
    for layout in 'random' 'nearly sorted' 'reversed' 'few unique'; do
        for size in 1000 2000 3000 5000 10000 20000 50000 100000 150000 500000 1000000 1500000 2000000; do
            echo "Trying algorithm ${algorithm} with size ${size} and layout ${layout}."
            target/release/cm0889-lab2 ${output} ${size} ${algorithm} ${layout}
            if [ $? -ne 0 ]; then
                echo "Crashed!"
                break
            fi
        done
    done
done
