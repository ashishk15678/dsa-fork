fn delete(mut arr: Vec<usize>, element: usize) -> Vec<usize> {
    for i in 0..arr.len() - 1 {
        if arr[i] == element {
            arr.remove(i);
        }
    }
    return arr;
}

fn main() {
    println!("{:?}", delete([1, 2, 3, 4].to_vec(), 2));
}
