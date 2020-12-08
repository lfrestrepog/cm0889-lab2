use rand::Rng;
use std::env;
use std::fs::OpenOptions;
use std::io::prelude::*;
use std::time::Instant;

fn main() -> std::io::Result<()> {
    let args: Vec<String> = env::args().collect();
    let output = &args[1];
    let sample_size = &args[2];
    let algorithm = &args[3];
    let layout = &args[4];

    let sample_size: i32 = match sample_size.trim().parse() {
        Ok(n) => n,
        Err(_) => 1_000,
    };
    let order: f64 = (sample_size as f64).log(10.0);

    let time_unit = if order < 6.0 {
        "microseconds"
    } else {
        "milliseconds"
    };

    let mut results = String::new();

    let sample = create_sample(layout_from_name(layout), sample_size);
    let start = Instant::now();
    let mut to_sort = sample.clone();
    if "bubble" == algorithm {
        bubble_sort(&mut to_sort);
    } else if "selection" == algorithm {
        selection_sort(&mut to_sort);
    } else if "insertion" == algorithm {
        insertion_sort(&mut to_sort);
    } else if "shell" == algorithm {
        shell_sort(&mut to_sort);
    } else if "merge" == algorithm {
        merge_sort(&mut to_sort);
    } else if "quick" == algorithm {
        quick_sort(&mut to_sort);
    } else if "tail" == algorithm {
        tail_quicksort(&mut to_sort, 0, sample_size - 1);
    } else {
        to_sort.sort();
    }

    let duration = if order < 6.0 {
        start.elapsed().as_micros()
    } else {
        start.elapsed().as_millis()
    };
    results.push_str(&format!(
        "{}\t{}\t{}\t{}\t{}\n",
        layout, sample_size, algorithm, duration, time_unit
    ));
    let mut file = OpenOptions::new().append(true).open(output).unwrap();

    file.write_all(results.as_bytes())?;

    Ok(())
}

enum SampleLayout {
    Random,
    NearlySorted,
    Reversed,
    FewUnique,
}

fn layout_from_name(layout: &str) -> SampleLayout {
    match layout {
        "random" => SampleLayout::Random,
        "nearly sorted" => SampleLayout::NearlySorted,
        "reversed" => SampleLayout::Reversed,
        "few unique" => SampleLayout::FewUnique,
        &_ => SampleLayout::Random,
    }
}

fn create_sample(layout: SampleLayout, size: i32) -> Vec<i32> {
    match layout {
        SampleLayout::Random => random_sample(size),
        SampleLayout::NearlySorted => nearly_sorted_sample(size),
        SampleLayout::Reversed => reversed_sample(size),
        SampleLayout::FewUnique => few_unique_sample(size),
    }
}

fn random_sample(size: i32) -> Vec<i32> {
    let mut nums: Vec<i32> = Vec::with_capacity(size as usize);
    for _ in 0..size {
        nums.push(rand::thread_rng().gen_range(0, size));
    }
    nums
}

fn nearly_sorted_sample(size: i32) -> Vec<i32> {
    let mut nums: Vec<i32> = Vec::with_capacity(size as usize);
    let mut rng = rand::thread_rng();
    for i in 0..size {
        let t: f32 = rng.gen();
        if t < 0.2 {
            nums.push(rng.gen_range(0, size));
            continue;
        }
        nums.push(i);
    }
    nums
}

fn reversed_sample(size: i32) -> Vec<i32> {
    let mut nums: Vec<i32> = Vec::with_capacity(size as usize);
    for i in 0..size {
        nums.push(size - i);
    }
    nums
}

fn few_unique_sample(size: i32) -> Vec<i32> {
    let mut nums: Vec<i32> = Vec::with_capacity(size as usize);
    let mut rng = rand::thread_rng();
    let unique = size / 100;
    for _ in 0..size {
        nums.push(rng.gen_range(0, unique))
    }
    nums
}

fn bubble_sort(numbers: &mut [i32]) -> &[i32] {
    // Stephens, R. (n.d.). Essential Algorithms: A Practical Approach to Computer Algorithms. Wiley.
    loop {
        let mut sorted = true;
        let l = numbers.len() - 1;
        let mut i = 0;
        while i < l {
            if numbers[i] > numbers[i + 1] {
                numbers.swap(i, i + 1);
                sorted = false;
            }
            i += 1;
        }
        if sorted {
            return numbers;
        }
    }
}

fn selection_sort(numbers: &mut [i32]) -> &[i32] {
    // Stephens, R. (n.d.). Essential Algorithms: A Practical Approach to Computer Algorithms. Wiley.
    let l = numbers.len();
    let mut i = 0;
    while i < l - 1 {
        let mut min = i;
        let mut j = i + 1;
        while j < l {
            if numbers[j] < numbers[min] {
                min = j;
            }
            j += 1;
        }
        numbers.swap(min, i);
        i += 1;
    }
    numbers
}

fn insertion_sort(numbers: &mut [i32]) -> &[i32] {
    // Stephens, R. (n.d.). Essential Algorithms: A Practical Approach to Computer Algorithms. Wiley.
    let l = numbers.len();
    let mut i = 0;
    while i < l {
        let mut j = i;
        while j > 0 && numbers[j] < numbers[j - 1] {
            numbers.swap(j - 1, j);
            j -= 1;
        }
        i += 1;
    }
    numbers
}

fn shell_sort(numbers: &mut [i32]) -> &[i32] {
    // https://en.wikipedia.org/wiki/Shellsort
    let l = numbers.len();
    let leap = 2;
    let mut h = l / leap;
    while h > 0 {
        let mut i = h;
        while i < l {
            let aux = numbers[i];
            let mut j = i;
            while j >= h && numbers[j - h] > aux {
                numbers[j] = numbers[j - h];
                j -= h;
            }
            numbers[j] = aux;
            i += 1;
        }
        h /= leap;
    }
    numbers
}

fn merge_sort(numbers: &[i32]) -> Vec<i32> {
    // Stephens, R. (n.d.). Essential Algorithms: A Practical Approach to Computer Algorithms. Wiley.
    let l = numbers.len();
    if l == 1 {
        return numbers.to_vec();
    }

    let h = l / 2;
    let (left, right) = numbers.split_at(h);
    let left = merge_sort(left);
    let right = merge_sort(right);

    let mut li = 0;
    let mut ri = 0;
    let mut mi = 0;
    let mut merged: Vec<i32> = vec![0; l];
    while li < h && ri < l - h {
        if left[li] <= right[ri] {
            merged[mi] = left[li];
            li += 1;
        } else {
            merged[mi] = right[ri];
            ri += 1;
        }
        mi += 1;
    }

    for i in &left[li..] {
        merged[mi] = *i;
        mi += 1;
    }
    for i in &right[ri..] {
        merged[mi] = *i;
        mi += 1;
    }

    merged
}

fn quick_sort(numbers: &mut [i32]) -> &[i32] {
    // Stephens, R. (n.d.). Essential Algorithms: A Practical Approach to Computer Algorithms. Wiley.
    let l = numbers.len();
    if l <= 1 {
        return numbers;
    }

    let divider = numbers[0];
    let mut lo = 0;
    let mut hi = l - 1;

    loop {
        while numbers[hi] >= divider {
            hi = hi - 1;
            if hi <= lo {
                break;
            }
        }
        if hi <= lo {
            numbers[lo] = divider;
            break;
        }
        numbers[lo] = numbers[hi];
        lo += 1;
        while numbers[lo] < divider {
            lo += 1;
            if lo >= hi {
                break;
            }
        }
        if lo >= hi {
            lo = hi;
            numbers[hi] = divider;
            break;
        }
        numbers[hi] = numbers[lo];
    }
    // When lo <= 1 the left part is trivially sorted
    if lo > 1 {
        quick_sort(&mut numbers[..lo]);
    }
    // (l - 1 is the last index of the array)
    // When lo >= (l - 1) - 1 the right part is trivially sorted
    if lo < l - 1 - 1 {
        quick_sort(&mut numbers[lo + 1..]);
    }
    numbers
}

fn tail_quicksort(mut numbers: &mut [i32], mut left: i32, mut right: i32) -> &[i32] {
    // https://writeach.com/posts/-MA2pAq6svg0-F-Lhe4b/Quicksort-in-C%2B%2B-with-Tail-Recursion
    while left < right {
        let pivot = partition(&mut numbers, left, right);
        if pivot - left <= right - pivot {
            tail_quicksort(&mut numbers, left, pivot - 1);
            left = pivot + 1;
        } else {
            tail_quicksort(&mut numbers, pivot + 1, right);
            right = pivot - 1;
        }
    }

    numbers
}

fn partition(numbers: &mut [i32], left: i32, right: i32) -> i32 {
    // https://writeach.com/posts/-MA2pAq6svg0-F-Lhe4b/Quicksort-in-C%2B%2B-with-Tail-Recursion
    let mut low = left as usize;
    let mut high = right as usize;
    let pivot = high;
    while low < high {
        while numbers[low] < numbers[pivot] && low <= pivot {
            low += 1;
        }
        while numbers[high] >= numbers[pivot] && high > low {
            high -= 1;
        }

        if low < high {
            numbers.swap(low, high);
            low += 1;
        }
    }
    numbers.swap(low, pivot);
    low as i32
}
