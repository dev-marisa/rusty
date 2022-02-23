// find the missing number in the array
fn missing_num(arr: &[i32]) -> i32 {
    if arr.len() < 1 {
        return 0;
    }
    let mut max = arr[0];
    let mut min = arr[0];
    let mut sum = 0;
    let mut sum2 = 0;
    for val in arr {
        if max < *val {
            max = *val;
        }
        if min > *val {
            min = *val;
        }
        sum += val;
    }
    for i in min..max+1 {
        sum2 += i;
    }
    return sum2 - sum;
}

fn main() {
    // -1 should be missing
    println!("{}", missing_num(&[2, -4, 0, -3, -2, 1]));
    // 32 should be missing
    println!("{}", missing_num(&[33, 29, 27, 31, 30, 28]));
}