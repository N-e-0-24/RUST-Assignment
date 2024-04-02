fn find_first_occurrence(arr: &[i32], target: i32) -> Option<usize> {
    let mut low = 0;
    let mut high = arr.len() - 1;
    let mut result = None;

    while low <= high {
        let mid = low + (high - low) / 2;
        if arr[mid] == target {
            result = Some(mid);
            high = mid - 1; // Keep searching for the first occurrence
        } else if arr[mid] < target {
            low = mid + 1;
        } else {
            high = mid - 1;
        }
    }

    result
}

fn main() {
    let sorted_array = [1, 3, 5, 5, 7, 9, 11];
    let target = 5;

    if let Some(index) = find_first_occurrence(&sorted_array, target) {
        println!("First occurrence of {} is at index {}", target, index);
    } else {
        println!("{} not found in the array", target);
    }
}
