// use rand::Rng;

// pub fn print_map(map: &Vec<Vec<u8>>) {
//     for row in map {
//         for &cell in row {
//             if cell == 0 {
//                 print!("□ "); // 空格表示通道
//             } else if cell == 1 {
//                 print!("■ "); // 实心方块表示障碍
//             } else if cell == 2 {
//                 print!("* "); // 星号表示路径
//             }
//         }
//         println!();
//     }
// }

// pub fn mark_path_on_map(map: &mut Vec<Vec<u8>>, path: &Vec<Vec<usize>>) {
//     for point in path {
//         if point.len() == 2 && point[0] < map.len() && point[1] < map[0].len() {
//             // Check if the point is within the bounds of the map
//             map[point[0]][point[1]] = 2; // Set the value to 2 times the original value
//         }
//     }
// }

// // 设置障碍的函数
// pub fn set_random_obstacles(map: &mut Vec<Vec<u8>>, num_obstacles: usize) {
//     let mut rng = rand::thread_rng();

//     for _ in 0..num_obstacles {
//         let row = rng.gen_range(0..50);
//         let col = rng.gen_range(0..50);
//         map[row][col] = 1;
//     }
// }
