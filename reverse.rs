fn reverse_string(word: &str) -> String {
    let mut reversed = String::new();
    for c in word.chars().rev() {
        reversed.push(c);
    }
    return reversed;
}

fn reverse_array(arr: &mut [i32]) -> &mut [i32] {
    for i in 0..arr.len() / 2 {
        let temp = arr[i];
        arr[i] = arr[arr.len() - 1 - i];
        arr[arr.len() - 1 - i] = temp;
    }
    return arr;
}

fn main() {
    println!("{}", reverse_string("hello"));
    println!("{:?}", reverse_array(&mut [1, 2, 3, 4, 5]));
}
