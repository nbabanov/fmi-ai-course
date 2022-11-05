use std::{io, ops::Sub, time::Instant};

use strum::IntoEnumIterator;
use strum_macros::EnumIter;

#[derive(Copy, Clone)]
struct Point2D {
    x: u32,
    y: u32,
}

fn get_manhatten_distance(a: &Point2D, b: &Point2D) -> u32 {
    return u32::abs_diff(a.x, b.x) + u32::abs_diff(a.y, b.y);
}

#[derive(Clone)]
struct PuzzleNode {
    cost: u32,
    board: Vec<u32>,
    path_to_puzzle_node: Vec<String>,
}

fn index_1d_to_index_2d(board_side: u32, index: usize) -> Point2D {
    return Point2D {
        x: index as u32 % board_side,
        y: index as u32 / board_side,
    };
}

fn get_zero_index(vector: &Vec<u32>) -> usize {
    return vector.iter().position(|&r| r == 0).unwrap();
}

fn board_to_point_2d(board_side: u32, board: &Vec<u32>) -> Point2D {
    return index_1d_to_index_2d(board_side, get_zero_index(board));
}

fn default_step_cost(board_side: u32, a: &PuzzleNode, b: &PuzzleNode) -> u32 {
    return get_manhatten_distance(
        &board_to_point_2d(board_side, &a.board),
        &board_to_point_2d(board_side, &b.board),
    );
}

type Heuristic = Box<dyn Fn(&PuzzleNode) -> u32>;
type GoalPredicate = Box<dyn Fn(&Option<PuzzleNode>) -> bool>;

#[derive(Clone)]
struct SearchResult {
    cost: u32,
    node: Option<PuzzleNode>,
}

#[derive(Debug, EnumIter)]
enum BoardMove {
    Up,
    Right,
    Down,
    Left,
}

fn board_move_to_string(board_move: &BoardMove) -> String {
    return String::from(match &board_move {
        BoardMove::Up => "up",
        BoardMove::Right => "right",
        BoardMove::Down => "down",
        BoardMove::Left => "left",
    });
}

fn get_move_index(board_side: u32, current_index: usize, board_mode: &BoardMove) -> usize {
    match board_mode {
        BoardMove::Down => current_index - board_side as usize,
        BoardMove::Up => current_index + board_side as usize,
        BoardMove::Right => {
            if current_index % board_side as usize == 0 {
                return usize::MAX;
            }
            return current_index - 1;
        }
        BoardMove::Left => {
            if current_index % board_side as usize == (board_side - 1) as usize {
                return (board_side * board_side) as usize;
            }

            return current_index + 1;
        }
    }
}

fn get_successors(board_side: u32, node: &PuzzleNode) -> Vec<PuzzleNode> {
    let zero_index = get_zero_index(&node.board);
    let board_length = node.board.len();

    let mut result: Vec<PuzzleNode> = vec![];

    for i in BoardMove::iter() {
        let move_index = get_move_index(board_side, zero_index, &i);

        if move_index < board_length {
            let mut board = node.board.to_vec();
            let mut path = node.path_to_puzzle_node.to_vec();
            let board_move_string = board_move_to_string(&i);

            path.push(board_move_string);
            board[zero_index] = board[move_index];
            board[move_index] = 0;
            result.push(PuzzleNode {
                cost: default_step_cost(
                    board_side,
                    node,
                    &PuzzleNode {
                        cost: 0,
                        board: board.clone(),
                        path_to_puzzle_node: vec![],
                    },
                ),
                board,
                path_to_puzzle_node: path,
            });
        }
    }

    return result;
}

type StepCost = Box<dyn Fn(&PuzzleNode, &PuzzleNode) -> u32>;

fn do_vecs_match<T: PartialEq>(a: &Vec<T>, b: &Vec<T>) -> bool {
    let matching = a.iter().zip(b.iter()).filter(|&(a, b)| a == b).count();
    matching == a.len() && matching == b.len()
}

fn is_node_in_path(node: &PuzzleNode, path: &Vec<PuzzleNode>) -> bool {
    return !Option::is_none(
        &path
            .into_iter()
            .find(|&element| do_vecs_match(&element.board, &node.board)),
    );
}

fn search(
    board_side: u32,
    heuristic: &Heuristic,
    get_step_cost: &StepCost,
    is_goal: &GoalPredicate,
    path: &mut Vec<PuzzleNode>,
    current_cost: u32,
    threshold: u32,
) -> SearchResult {
    let node = path.last().unwrap().clone();
    let cheapest_path_cost = current_cost + heuristic(&node);

    if cheapest_path_cost > threshold {
        return SearchResult {
            cost: cheapest_path_cost,
            node: None,
        };
    }

    if is_goal(&Some(node.clone())) {
        return SearchResult {
            cost: cheapest_path_cost,
            node: Some(PuzzleNode {
                cost: node.cost,
                board: node.board.clone(),
                path_to_puzzle_node: node.path_to_puzzle_node.clone(),
            }),
        };
    }

    let mut min = u32::MAX;

    let successors = get_successors(board_side, &node);

    for successor in successors {
        let is_in_path = is_node_in_path(&successor, path);

        if !is_in_path {
            path.push(successor.clone());

            let step_cost = get_step_cost(&node, &successor);

            let search_result = search(
                board_side,
                heuristic,
                get_step_cost,
                is_goal,
                path,
                current_cost + step_cost,
                threshold,
            );

            if !Option::is_none(&search_result.node) {
                return search_result;
            }

            if search_result.cost < min {
                min = search_result.cost;
            }

            path.pop();
        }
    }

    return SearchResult {
        cost: min,
        node: None,
    };
}

struct IDAResult {
    path: Vec<String>,
    threshold: u32,
}

fn ida_star(
    board_side: u32,
    heuristic: &Heuristic,
    step_cost: &StepCost,
    is_goal: &GoalPredicate,
    root: PuzzleNode,
) -> Option<IDAResult> {
    let mut successors = get_successors(board_side, &root);
    successors.sort_by(|a, b| a.cost.cmp(&b.cost));
    let min_threshold = successors[0].cost;
    let mut threshold = min_threshold;
    let mut path = vec![root];

    loop {
        let result = search(
            board_side, heuristic, step_cost, is_goal, &mut path, 0, threshold,
        );
        if is_goal(&result.node.clone()) {
            return Some(IDAResult {
                path: result
                    .node
                    .unwrap_or(PuzzleNode {
                        cost: 0,
                        board: vec![],
                        path_to_puzzle_node: vec![],
                    })
                    .path_to_puzzle_node,
                threshold,
            });
        }

        if Option::is_some(&result.node) && is_node_in_path(&result.node.unwrap(), &path) {
            return None;
        }

        threshold = result.cost;
    }
}

fn is_goal_creator(goal_index: usize) -> Box<dyn Fn(&Option<PuzzleNode>) -> bool> {
    Box::new(move |node: &Option<PuzzleNode>| -> bool {
        if Option::is_none(node) {
            return false;
        }

        let mut goal_board = node.as_ref().unwrap().board.clone();
        goal_board.sort();
        goal_board.remove(0);
        goal_board.insert(goal_index, 0);

        return do_vecs_match(&goal_board, &node.as_ref().unwrap().board);
    })
}

fn solve_puzzle(board_size: u32, goal_index_raw: i32, board: Vec<u32>) {
    let goal_index: usize = if goal_index_raw == -1 {
        board_size as usize
    } else {
        goal_index_raw as usize
    };
    let board_side = board_size_to_board_side(board_size);
    let is_goal = is_goal_creator(goal_index);
    let heuristic: Heuristic = Box::new(move |node: &PuzzleNode| -> u32 {
        get_manhatten_distance(
            &board_to_point_2d(board_side, &node.board),
            &index_1d_to_index_2d(board_side, goal_index),
        )
    });

    let step_cost: StepCost =
        Box::new(move |a: &PuzzleNode, b: &PuzzleNode| -> u32 { default_step_cost(board_side, a, b) });

    let root = PuzzleNode {
        cost: 0,
        board,
        path_to_puzzle_node: vec![],
    };

    let ida_start = Instant::now();
    let result = ida_star(board_side, &heuristic, &step_cost, &is_goal, root).unwrap();
    let ida_end = Instant::now();

    println!("{}", result.threshold);
    println!("{}", result.path.join("\n"));
    println!("IDA* took: {:?}s", ida_end.sub(ida_start).as_secs_f64());
}

fn board_size_to_board_side(size: u32) -> u32 {
    return f32::sqrt((size + 1) as f32) as u32;
}

fn is_solvable(board_size: u32, board: &Vec<u32>) -> bool {
    let mut inversions = 0;

    for i in 0..board_size {
        for j in i + 1..board_size {
            if board[j as usize] > board[i as usize] {
                inversions += 1;
            }
        }
    }

    return inversions % 2 == 0;
}

fn main() {
    let mut input = String::new();

    let board_size: u32;

    println!("What is the board size (N)?");
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read a line!");

    board_size = input.trim().parse().expect("Please type a number!");
    let goal_index: i32;

    input.clear();

    println!("Where is the goal?");
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read a line!");
    goal_index = input.trim().parse().expect("Please type a number!");

    let mut initial_board = vec![];

    input.clear();

    println!("What is the initial board?");
    for _row in 0..board_size_to_board_side(board_size) {
        io::stdin().read_line(&mut input).expect("");
        for value in input.split_whitespace() {
            initial_board.push(value.parse::<u32>().unwrap())
        }
        input.clear();
    }

    if !is_solvable(board_size, &initial_board) {
        println!("The given board is not solvable!");
        return;
    }

    solve_puzzle(board_size, goal_index, initial_board);
}
