fn bubble(arr: &mut[i32]) -> &[i32] {
    for i in 0..arr.len() {
        let mut changed = false;
        for j in 0..arr.len()-1-i {
            if arr[j] > arr[j+1] {
                let temp = arr[j];
                arr[j] = arr[j+1];
                arr[j+1] = temp;
                changed = true;
            }
        }
        if !changed {
            break;
        }
    }
    return arr;
}

fn select(arr: &mut[i32]) -> &[i32] {
    for i in 0..arr.len() {
        let mut min = i;
        for j in i..arr.len() {
            if arr[j] < arr[min] {
                min = j;
            }
        }
        let temp = arr[i];
        arr[i] = arr[min];
        arr[min] = temp;
    }
    return arr;
}

fn insert(arr: &mut[i32]) -> &[i32] {
    for i in 1..arr.len() {
        let mut j = i;
        while j > 0 {
            if arr[j] < arr[j-1] {
                let temp = arr[j];
                arr[j] = arr[j-1];
                arr[j-1] = temp;
                j -= 1;
            } else {
                break;
            }
        }
    }
    return arr;
}

// some basic sorts
fn main() {
    println!("{:?}", bubble(&mut [3, 1, 4, 2, 5]));
    println!("{:?}", select(&mut [3, 1, 4, 2, 5]));
    println!("{:?}", insert(&mut [3, 1, 4, 2, 5]));
}
