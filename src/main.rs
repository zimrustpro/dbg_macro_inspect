fn main() {
    let my_number = 8;
    dbg!(my_number);

    let mut _my_number = 9;
    dbg!(_my_number += 10);
    let new_vec = dbg!(vec![8, 9, 10]);
    let double_vec = dbg!(new_vec.iter().map(|x| x * 2).collect::<Vec<i32>>());
    dbg!(double_vec);

    let new_vec = vec![8, 9, 10];
    let _d_vec = new_vec
        .iter()
        .inspect(|item| {
            println!("The item is: {}", item);
            match **item % 2 {
                0 => println!("It is even."),
                _ => println!("It is odd."),
            }
            println!("In binary it is {:b}.", item);
        })
        .map(|x| x * 2)
        .inspect(|next_item| println!("Then it is: {}", next_item))
        .collect::<Vec<i32>>();
}
