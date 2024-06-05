fn main() {
    let v = vec![
        vec![1, 2, 3, 4],
        vec![5, 6, 7, 8],
        vec![9, 10, 11, 12],
        vec![13, 14, 15, 16],
    ];
    printSnake(v);
}

pub fn printSnake(v: Vec<Vec<u8>>) {
    let mut i = 0;
    for j in 0..v.len() {
        if i % 2 == 0 {
            for k in 0..v[0].len() {
                print!("{}", v[j][k]);
                print!(" ");
            }
        } else {
            let mut t = v[j].len();
            while t != 0 {
                print!("{}", v[j][t - 1]);
                print!(" ");
                t -= 1;
            }
        }
        print!("\n");
        i += 1;
    }
}
