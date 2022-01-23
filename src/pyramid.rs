// use std::cmp::max;
//
// fn longest_slide_down(pyramid: &[Vec<u16>]) -> u16 {
//     pyramid.to_vec().into_iter().rev().reduce(|acc, level| {
//         level.into_iter().enumerate().map(|(index, value)| {
//             let left_under = acc[index];
//             let right_under = acc[index + 1];
//             max(left_under, right_under) + value
//         }).collect()
//     }).unwrap()[0]
// }
//
// fn solve_nonogram((top_clues, left_clues): ([&[u8]; 5], [&[u8]; 5])) -> [[u8; 5]; 5] {
//     solve(&top_clues, &left_clues, [[0; 5], [0; 5], [0; 5], [0; 5], [0; 5]]).expect_err("nonogram has solution")
// }
//
// type Board = [[u8; 5]; 5];
// type Clues<'a> = [&'a[u8]; 5];
//
// fn solve(top_clues: &[&[u8]; 5], left_clues: &[&[u8]; 5], board: Board) -> Result<(), Board> {
//
// }
//
// fn is_valid(top_clues: &Clues, left_clues: &Clues, board: Board) -> bool {
//     for (row, clues) in top_clues.iter().enumerate() {
//
//     }
//
//     true
// }
//
// fn check_clues(clues: &Clues, row: [u8; 5]) -> bool {
//
//     true
// }
