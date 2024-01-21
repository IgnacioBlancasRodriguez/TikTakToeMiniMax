use std::io;
use colored::Colorize;

enum GameStatus {
    Win(usize),
    Tie,
    Unfinished,
}

enum MiniMaxState {
    Minimize,
    Maximize,
}

fn minimax(b : [[usize; 3]; 3], state : MiniMaxState, player : usize, depth : i32)
    -> (Option<i32>, Option<(usize, usize)>)
{
    // Using the Minimax algorithm it calculates the bot's next move
    // by exploring all of the diferent possible moves
    
    match check_game_status(b) {
        GameStatus::Tie => (Some(0), None),
        GameStatus::Win(p) => {
            match p {
                2 => (Some(10 - depth), None),
                1 => (Some(-10 + depth), None),
                _ => (None, None),
            }
        },
        GameStatus::Unfinished => {
            // Logic
            let mut best_score: Option<i32> = None;
            let mut best_posn: Option<(usize, usize)> = None;
            for i in 0..3
            {
                for j in 0..3
                {
                    if b[i][j] == 0
                    {
                        let mut b_mut = b;
                        b_mut[i][j] = player;
                        match state {
                            MiniMaxState::Minimize => {
                                let (current_score, _) =
                                    minimax(b_mut, MiniMaxState::Maximize, 2, depth + 1);
                                match current_score {
                                    Some(v) => {
                                        match best_score {
                                            Some(v_b) => {
                                                if v < v_b
                                                {
                                                    best_score = current_score;
                                                    best_posn = Some((i, j));
                                                }
                                            },
                                            None => {
                                                best_score = current_score;
                                                best_posn = Some((i, j));
                                            },
                                        }
                                    }
                                    None => continue,
                                }
                            },
                            MiniMaxState::Maximize => {
                                let mut b_mut = b;
                                b_mut[i][j] = player;
                                let (current_score, _) = 
                                    minimax(b_mut, MiniMaxState::Minimize, 1, depth + 1);
                                match current_score {
                                    Some(v) => {
                                        match best_score {
                                            Some(v_b) => {
                                                if v > v_b
                                                {
                                                    best_score = current_score;
                                                    best_posn = Some((i, j));
                                                }
                                            },
                                            None => {
                                                best_score = current_score;
                                                best_posn = Some((i, j));
                                            },
                                        }
                                    }
                                    None => continue,
                                }
                            },
                        };
                    }
                }
            }
            return (best_score, best_posn);
        }
    }  
}

fn check_path(
    b : [[usize; 3]; 3],
    path : [(usize, usize); 3], p : usize) -> bool
{
    // Checks whether a certain path is made up soley of one player

    for (row, col) in path
    {
        if b[row][col] == p
        {
            continue;
        } else {
            return false
        }
    }
    return true
}

fn check_game_status(b : [[usize; 3]; 3]) -> GameStatus
{
    // Check the status of the game

    // Defines all of the possible paths for a player to win
    let diag1 = [(0,0), (1,1), (2,2)];
    let diag2 = [(0,2), (1,1), (2,0)];
    let horiz1 = [(0, 0), (1, 0), (2, 0)];
    let horiz2 = [(0, 1), (1, 1), (2, 1)];
    let horiz3 = [(0, 2), (1, 2), (2, 2)];
    let ver1 = [(0, 0), (0, 1), (0, 2)];
    let ver2 = [(1, 0), (1, 1), (1, 2)];
    let ver3 = [(2, 0), (2, 1), (2, 2)];

    // Check whether player 1 has managed to complete any of the paths
    let p1_won = check_path(b, diag1, 1)
        | check_path(b, diag2, 1) | check_path(b, horiz1, 1)
        | check_path(b, horiz2, 1) | check_path(b, horiz3, 1)
        | check_path(b, ver1, 1) | check_path(b, ver2, 1)
        | check_path(b, ver3, 1);
    match p1_won
    {
        true => GameStatus::Win(1),
        false => {
            // Check whether player 2 has completed any of the paths
            let p2_won = check_path(b, diag1, 2)
                | check_path(b, diag2, 2) | check_path(b, horiz1, 2)
                | check_path(b, horiz2, 2) | check_path(b, horiz3, 2)
                | check_path(b, ver1, 2) | check_path(b, ver2, 2)
                | check_path(b, ver3, 2);
            match p2_won {
                true => GameStatus::Win(2),
                false => {
                    // Check whether its a tie
                    for i in 0..3
                    {
                        for j in 0..3
                        {
                            match b[i][j] {
                                1 => continue,
                                2 => continue,
                                _ => return GameStatus::Unfinished,
                            }
                        }
                    };
                    return GameStatus::Tie;
                }
            }
        },
    }
}

fn make_move(b : &mut [[usize; 3]; 3],
    (pos_x, pos_y) : (usize, usize), p : usize)
{
    // Given two coordinates in the board, it makes a move
    // for the given player if the spot is available

    if b[pos_y][pos_x] == 0
    {
        b[pos_y][pos_x] = p; 
    } else
    {
        println!("The specified position was already taken");
    }
}

fn draw_board(b : [[usize; 3]; 3])
{
    // Given a board matrix it draws it on the screen

    for i in 0..3
    {
        let mut line = String::new();
        for j in 0..3
        {
            let to_draw = {
                match b[i][j]
                {
                    1 => format!("| {} |", "x".green()),
                    2 => format!("| {} |", "o".blue()),
                    _ => format!("| {} |", " "),
                }
            };
            line.push_str(&to_draw);
        }
        println!("{line}");
    }
}

fn main() 
{
    let mut board : [[usize; 3]; 3] = [[0,0,0],[0,0,0],[0,0,0]];

    loop
    {
        print!("\x1B[2J\x1B[1;1H");
        draw_board(board);
        println!("Input the row:");
        let mut y = String::new();
        io::stdin()
            .read_line(&mut y)
            .expect("Failed to read line");
        let y: usize = match y.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Please input a number");
                continue;
            }
        };
        println!("Input the col:");
        let mut x: String = String::new();
        io::stdin()
            .read_line(&mut x)
            .expect("Failed to read line");
        let x: usize = match x.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Please input a number");
                continue;
            }
        };
        if board[y][x] == 0
        {
            make_move(&mut board, (x, y), 1);

            let (_, machine_move) = 
                minimax(board, MiniMaxState::Maximize, 2, 0);
            match machine_move {
                Some((row, col)) => make_move(&mut board, (col, row), 2),
                None => {// Check the status of the game
                    match check_game_status(board)
                    {
                        GameStatus::Win(p) => {
                            print!("\x1B[2J\x1B[1;1H");
                            draw_board(board); 
                            println!("Player {p} won!");
                            break;
                        },
                        GameStatus::Tie => {
                            println!("It's a tie!");
                            break;
                        },
                        GameStatus::Unfinished => continue,
                    }},
            };
        } else {
            match check_game_status(board)
            {
                GameStatus::Win(p) => {
                    print!("\x1B[2J\x1B[1;1H");
                    draw_board(board); 
                    println!("Player {p} won!");
                    break;
                },
                GameStatus::Tie => {
                    println!("It's a tie!");
                    break;
                },
                GameStatus::Unfinished => continue,
            }
        }
    }
}