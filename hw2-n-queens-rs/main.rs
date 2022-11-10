use rand::Rng;
use std::{collections::HashMap, io, ops::Sub, time::Instant, vec};

// const BOARD_SIZE_COEFF: u32 = 8000;
const BOARD_SIZE_COEFF: u32 = 200;
// const BOARD_SIZE_COEFF: u32 = 3;

// _ 0 1 2
// 0 _ _ _  [1, 3] -> [0, 1, 1, 1]
// 1 Q _ _
// 2 _ _ _
// 3 _ Q _

fn cell_coordinates_to_d1_key(column_index: &u32, row_index: &u32) -> i64 {
    i64::from(*column_index) - i64::from(*row_index)
}

fn cell_coordinates_to_d2_key(column_index: &u32, row_index: &u32) -> i64 {
    i64::from(*column_index) + i64::from(*row_index)
}

fn get_initial_conflicts_per_cell(
    column_index: &u32,
    row_index: &u32,
    queens_per_row: &Vec<u32>,
    queens_per_d1: &HashMap<i64, u32>,
    queens_per_d2: &HashMap<i64, u32>,
) -> u32 {
    let d1_key = cell_coordinates_to_d1_key(column_index, row_index);
    let d2_key = cell_coordinates_to_d2_key(column_index, row_index);

    return queens_per_row[*row_index as usize]
        .checked_add(
            *queens_per_d1
                .get(&d1_key)
                .unwrap_or(&0)
        )
        .unwrap_or(u32::MAX)
        .checked_add(
            *queens_per_d2
                .get(&d2_key)
                .unwrap_or(&0)
        )
        .unwrap_or(u32::MAX);
}

fn get_conflicts_per_cell(
    column_index: &u32,
    row_index: &u32,
    queens_per_row: &Vec<u32>,
    queens_per_d1: &HashMap<i64, u32>,
    queens_per_d2: &HashMap<i64, u32>,
) -> u32 {
    let d1_key = cell_coordinates_to_d1_key(column_index, row_index);
    let d2_key = cell_coordinates_to_d2_key(column_index, row_index);

    return queens_per_row[*row_index as usize]
        .checked_sub(1)
        .unwrap_or(0)
        .checked_add(
            queens_per_d1
                .get(&d1_key)
                .unwrap_or(&0)
                .checked_sub(1)
                .unwrap_or(0),
        )
        .unwrap_or(u32::MAX)
        .checked_add(
            queens_per_d2
                .get(&d2_key)
                .unwrap_or(&0)
                .checked_sub(1)
                .unwrap_or(0),
        )
        .unwrap_or(u32::MAX);
}

#[derive(Debug)]
struct Point2D {
    x: u32,
    y: u32
}

fn print_board(board_size: &u32, board: &Vec<u32>) {
    for row_index in 0..*board_size {
        for column_index in 0..*board_size {
            if row_index == board[column_index as usize] {
                print!("*");
            } else {
                print!("_");
            }
            print!(" ");
        }
        print!("\n");
    }
    print!("\n");
    print!("\n");
}

fn initialize(
    board_size: &u32,
    board: &mut Vec<u32>,
) -> (Vec<u32>, HashMap<i64, u32>, HashMap<i64, u32>) {
    let mut rng = rand::thread_rng();

    let mut queens_per_row: Vec<u32> = vec![0; *board_size as usize];
    let mut queens_per_d1: HashMap<i64, u32> = HashMap::new();
    let mut queens_per_d2: HashMap<i64, u32> = HashMap::new();

    // let mut colnfl_per_column: Vec<Vec<u32>> = vec![];


    // let static_data: Vec<Point2D> = vec![
    //     // Point2D {
    //     //     x: 0,
    //     //     y: 0
    //     // },
    //     // Point2D {
    //     //     x: 1,
    //     //     y: 2
    //     // },
    //     // Point2D {
    //     //     x: 2,
    //     //     y: 0
    //     // },
    //     // Point2D {
    //     //     x: 3,
    //     //     y: 2
    //     // }

    //     Point2D {
    //         x: 0,
    //         y: 0
    //     },
    //     Point2D {
    //         x: 1,
    //         y: 0
    //     },
    //     Point2D {
    //         x: 2,
    //         y: 0
    //     },
    //     Point2D {
    //         x: 3,
    //         y: 0
    //     }
    // ];

    // for point in static_data {
    //     board.push(point.y);
    //     let row_index = point.y;
    //     let column_index = point.x;
    //     queens_per_row[row_index as usize] += 1;
    
    //     let d1_key = cell_coordinates_to_d1_key(&column_index, &row_index);
    //     let d1_value = queens_per_d1.get(&d1_key).unwrap_or(&0);
    //     queens_per_d1.insert(d1_key, d1_value + 1);
    
    //     let d2_key = cell_coordinates_to_d2_key(&column_index, &row_index);
    //     let d2_value = queens_per_d2.get(&d2_key).unwrap_or(&0);
    //     queens_per_d2.insert(d2_key, d2_value + 1);
    // }

    // println!("Per row: {:?}", queens_per_row);

    // return (queens_per_row, queens_per_d1, queens_per_d2);

    for column_index in 0..*board_size {
        // let mut column_conflicts: Vec<u32> = vec![];

        if board.len() == 0 {
            let first = rng.gen_range(1..*board_size);
            // println!("First is {}", &first);
            board.push(first);
        } else {
            let mut min_conflicts = u32::MAX;
            let mut min_conflicts_ids: Vec<u32> = vec![];

            for row_index in 0..*board_size {
                let conflicts = get_initial_conflicts_per_cell(
                    &column_index,
                    &row_index,
                    &queens_per_row,
                    &queens_per_d1,
                    &queens_per_d2,
                );

                if conflicts < min_conflicts {
                    min_conflicts = conflicts;
                    min_conflicts_ids.clear();
                    min_conflicts_ids.push(row_index);
                } else if conflicts == min_conflicts {
                    min_conflicts_ids.push(row_index);
                }
                // column_conflicts.push(conflicts);
                // column_conflicts.push();
            }

            // println!("Current conflicts: {:?}", &column_conflicts);
            // colnfl_per_column.push(column_conflicts);

            let move_index = rng.gen_range(0..min_conflicts_ids.len());
            // println!("Move index: {}", &move_index);

            board.push(min_conflicts_ids[move_index]);
        }


        let row_index = board[column_index as usize];
        queens_per_row[row_index as usize] += 1;

        let d1_key = cell_coordinates_to_d1_key(&column_index, &row_index);
        let d1_value = queens_per_d1.get(&d1_key).unwrap_or(&0);
        queens_per_d1.insert(d1_key, d1_value + 1);

        let d2_key = cell_coordinates_to_d2_key(&column_index, &row_index);
        let d2_value = queens_per_d2.get(&d2_key).unwrap_or(&0);
        queens_per_d2.insert(d2_key, d2_value + 1);
    }

    // println!("All conflicts: {:?}", colnfl_per_column);

    return (queens_per_row, queens_per_d1, queens_per_d2);
}

fn get_col_with_queen_with_max_conflicts(
    board: &Vec<u32>,
    queens_per_row: &Vec<u32>,
    queens_per_d1: &HashMap<i64, u32>,
    queens_per_d2: &HashMap<i64, u32>,
) -> (u32, u32) {
    let mut rng = rand::thread_rng();
    let mut max_conflicts: u32 = 0;
    let mut max_conflicts_ids: Vec<u32> = vec![];

    for column_index in 0..board.len() {
        let row_index = board[column_index];

        let conflicts = get_conflicts_per_cell(
            &(column_index as u32),
            &row_index,
            queens_per_row,
            queens_per_d1,
            queens_per_d2,
        );

        if conflicts > max_conflicts {
            max_conflicts = conflicts;
            max_conflicts_ids.clear();
            max_conflicts_ids.push(column_index as u32);
        } else if conflicts == max_conflicts {
            max_conflicts_ids.push(column_index as u32);
        }
    }

    let move_index = rng.gen_range(0..max_conflicts_ids.len());

    return (max_conflicts, max_conflicts_ids[move_index]);
}

fn get_row_with_min_conflicts(
    column_index: &u32,
    board: &Vec<u32>,
    queens_per_row: &Vec<u32>,
    queens_per_d1: &HashMap<i64, u32>,
    queens_per_d2: &HashMap<i64, u32>,
) -> u32 {
    let mut rng = rand::thread_rng();
    let mut min_conflicts = u32::MAX;
    let mut min_conflicts_ids: Vec<u32> = vec![];

    for row_index in 0..board.len() {
        let conflicts = get_conflicts_per_cell(
            &column_index,
            &(row_index as u32),
            &queens_per_row,
            &queens_per_d1,
            &queens_per_d2,
        );

        if conflicts < min_conflicts {
            min_conflicts = conflicts;
            min_conflicts_ids.clear();
            min_conflicts_ids.push(row_index as u32);
        } else if conflicts == min_conflicts {
            min_conflicts_ids.push(row_index as u32);
        }
        // column_conflicts.push();
    }

    let move_index = rng.gen_range(0..min_conflicts_ids.len());

    return min_conflicts_ids[move_index];
}

fn solve(board_size: &u32) -> Option<Vec<u32>> {
    let mut board: Vec<u32> = vec![];
    let (mut queens_per_row, mut queens_per_d1, mut queens_per_d2) =
        initialize(board_size, &mut board);

    // println!("Initial board:");
    // print_board(&board_size, &board);

    let mut has_conflicts = false;

    for ind in 0..BOARD_SIZE_COEFF * board_size {
        // println!("Try {}", ind);
        // randomly if two or more
        let (max_conflicts, max_conflicts_column) = get_col_with_queen_with_max_conflicts(
            &board,
            &queens_per_row,
            &queens_per_d1,
            &queens_per_d2,
        );

        // println!(
        //     "Max conflicts {} in column {}",
        //     max_conflicts, max_conflicts_column
        // );

        if max_conflicts == 0 {
            return Some(board);
        } else {
            has_conflicts = true;
        }

        // randomly if two or more
        let row_index: u32 = get_row_with_min_conflicts(
            &max_conflicts_column,
            &board,
            &queens_per_row,
            &queens_per_d1,
            &queens_per_d2,
        );

        let old_row_index = board[max_conflicts_column as usize];

        let d1_key = cell_coordinates_to_d1_key(&max_conflicts_column, &old_row_index);
        let d1_value = queens_per_d1.get(&d1_key).unwrap_or(&0);
        let d1_value_new = d1_value.checked_sub(1).unwrap_or(0);
        queens_per_d1.insert(d1_key, d1_value_new);

        let d2_key = cell_coordinates_to_d2_key(&max_conflicts_column, &old_row_index);
        let d2_value = queens_per_d2.get(&d2_key).unwrap_or(&0);
        let d2_value_new = d2_value.checked_sub(1).unwrap_or(0);
        queens_per_d2.insert(d2_key, d2_value_new);

        queens_per_row[old_row_index as usize] = queens_per_row[old_row_index as usize]
            .checked_sub(1)
            .unwrap_or(0);
        
        queens_per_row[row_index as usize] = queens_per_row[row_index as usize]
            .checked_add(1)
            .unwrap_or(u32::MAX);

        board[max_conflicts_column as usize] = row_index;

        let d1_key_after = cell_coordinates_to_d1_key(&max_conflicts_column, &row_index);
        let d1_value_after = queens_per_d1.get(&d1_key_after).unwrap_or(&0);
        let d1_value_new_after = d1_value_after.checked_add(1).unwrap_or(u32::MAX);
        queens_per_d1.insert(d1_key_after, d1_value_new_after);

        let d2_key_after = cell_coordinates_to_d2_key(&max_conflicts_column, &row_index);
        let d2_value_after = queens_per_d2.get(&d2_key_after).unwrap_or(&0);
        let d2_value_new_after = d2_value_after.checked_add(1).unwrap_or(u32::MAX);
        queens_per_d2.insert(d2_key_after, d2_value_new_after);

        // print_board(&board_size, &board);
    }

    if has_conflicts {
        // println!("Has conflicts!");
        // println!("Has conflicts:");
        // print_board(&board_size, &board);
        return None;
        // return solve(board_size);
    }

    return Some(board);
}

fn main() {
    let mut input = String::new();
    let board_size: u32;

    println!("What is the board size (N)?");
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read a line!");

    board_size = input.trim().parse().expect("Please type a number!");

    let solve_start = Instant::now();
    let board: Vec<u32>;
    loop {
        let result = solve(&board_size);

        if Option::is_some(&result) {
            board = result.unwrap();
            break;
        }
    }
    let solve_end = Instant::now();

    println!(
        "Solve took: {:?}s",
        solve_end.sub(solve_start).as_secs_f64()
    );

    if board_size < 30 {
        print_board(&board_size, &board);
    }
}
