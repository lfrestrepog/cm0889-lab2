# Sorting algorithms in Rust

Sorting algorithms implemented as homework in a course on algorithms analysis. I decided to use Rust as an excuse to learn it and for fun. *This is my second Rust application (right after **hello world**) and is most certainly not an idiomatic Rust implementation (maybe one day I'll come back and refactor the code here).*

The following algorithms where implemented:
- Bubble
- Selection
- Insertion
- Shell
- Merge sort
- Quick Sort (Regular recursive and tail recusive)

This application is intended to measure the running time of different algorithms on different input arrays. The following data distribution layouts are considered:
- Random
- Nearly sorted
- Reversed
- Few unique (about n/100 repetitions of each value)

Compile the program:
```
cargo build
```

Run the program:
```
echo 'Sample layout	Sample size	Algorithm	Duration    Time units' > RESULTS_FILE
cargo run RESULTS_FILE SAMPLE_SIZE algorithm layout
```
Where *algorithm* is one of:
- bubble (Bubble sort)
- selection (Selection sort)
- insertion (Insertion sort)
- shell (Shellsort)
- merge (Merge sort)
- quick (Quicksort)
- tail (Quicksort but tail recursive version)
- other (Rust's implementation of sort -Timsort-)

And *layout* is one of:
- random
- nearly
- sorted
- reversed
- "few unique"

Do notice the application expects the output file to exist, because that makes it easier (I'm lazy, sorry about that). An utility script is provided to run tests with all distribution layouts and different input sizes (see ```test_samples.sh```).

## Caveats of quicksort

Quicksort is embarrassingly slow when the input is in reverse order, that's well documented in the litterature and most production quality implementations randomize input to address this. But not my implementation here, I'm interested in actual worst case scenario.

Also in the case of revered input the recursive implementation of quicksort, without tail call optimisation, fails for large input sizes because it overflows the stack (luckily it fails really fast, so we don't waste much time).

