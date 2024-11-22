fn main() {
    let my_number = 8;
    dbg!(my_number);

    let mut _my_number = 9;
    dbg!(_my_number += 10);
    let new_vec = dbg!(vec![8, 9, 10]);
    let double_vec = dbg!(new_vec.iter().map(|x| x * 2).collect::<Vec<i32>>());
    dbg!(double_vec);
}
