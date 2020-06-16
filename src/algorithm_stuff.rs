pub fn linear_search(sample: &[usize; 10], target: &usize) -> Option<String> {
    for (index, element) in sample.iter().enumerate() {
        if element == target {
            return Some(format!("We found {} at index {}.", target, index));
        }
    }
    return None;
}

pub fn binary_search(sample: &[usize; 10], target: usize) -> usize {
    let mut lower: f32 = 0.0;
    let mut upper: f32 = sample.len() as f32 - 1.0;
    let denominator: f32 = 2.0;

    while lower <= upper {
        let middle: usize = ((lower + upper) / denominator) as usize;
        if target == sample[middle] {
            return sample[middle];
        }

        if target > sample[middle] {
            lower = (middle + 1) as f32
        } else {
            upper = (middle - 1) as f32
        }

        println!("Looping again.");
    }

    return 0;
}
