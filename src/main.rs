use std::{collections::HashMap, thread::{self, JoinHandle}, time::Instant};
use numtoa::NumToA;
use std::io::{stdin, stdout, Read, Write};
const DEPTH: u8 = 12;


#[cfg(test)]
mod tests {
    use super::*;
    fn speed_place(vec: Vec<u8>,board: &mut [u8; 42]){
        let mut team = 1;
        for spot in vec {
            *board = place(&board,spot,team).expect("no spots");
            if team == 1{
                team = 2;
            }else{
                team = 1;
            }
            
        }
    }
    #[test]
    fn solver_benchmark() {
        let mut times: [f32;2] = [0.0;2];
        let mut board: [u8; 42] = [0; 42];
        speed_place(vec![3,6,4,1],&mut board);
        display_board(&board);
        let start_time = Instant::now();
        let action = solve(&board,true);
        let lookup_time_elapsed = start_time.elapsed().as_secs_f32();
        times[0] = lookup_time_elapsed;
        println!("{} time elapsed action determined was {}",lookup_time_elapsed,action);
        drop(start_time);
        drop(lookup_time_elapsed);
        drop(action);
        drop(board);
        let mut board: [u8; 42] = [0; 42];
        speed_place(vec![0,2,0,1,0,1],&mut board);
        display_board(&board);
        let start_time = Instant::now();
        let action = solve(&board,false);
        let lookup_time_elapsed = start_time.elapsed().as_secs_f32();
        times[1] = lookup_time_elapsed;
        println!("{} time elapsed action determined was {}",lookup_time_elapsed,action);
        drop(start_time);
        drop(lookup_time_elapsed);
        drop(action);
        assert_eq!(2+2, 4);
    }
}
fn pause() {
    let mut stdout = stdout();
    stdout.write(b"Press Enter to continue...").unwrap();
    stdout.flush().unwrap();
    stdin().read(&mut [0]).unwrap();
}
fn grab_input() -> Result<u8,i32>{
    print!("INPUT ACTION:");
    let mut action = 0;
    let mut input_text = String::new();
    std::io::stdin()
        .read_line(&mut input_text)
        .expect("failed to read from stdin");

    let trimmed = input_text.trim();
    match trimmed.parse(){
        Ok(i) => action = i,
        Err(..) => {println!("this was not an integer: {}", trimmed);return Err(22)},
    };
    return Ok(action)
}
fn main() {
    println!("DEPTH:{}",DEPTH);
    let mut board: [u8; 42] = [0; 42];
    display_board(&board);
    let mut team = 1;
    let done = false;
    while !done {
        let mut action = 0;
        if team == 2{
            action = solve(&board,true);
            
        }else{
            action = solve(&board,false);
        }
        println!("ACTION: {}",action);
        
        match place(&board,action as u8,team){
            Ok(new_board) => {
                board = new_board;
                let won = check_win(&board);
                match won {
                    Some(w) => {display_board(&board);println!("{} team won",w);board = [0; 42];team=1;},
                    None => {if team == 1{team = 2}else{team = 1}}
                }
                display_board(&board);
            }
            Err(e) => println!("{}",e)
        }
        pause();
        
    }

    

}
fn place(b: &[u8; 42], place: u8,team:u8) -> Result<[u8; 42],&str>{
    let mut board = b.clone();
    for i in 0..6 {
        let spot:usize = (place+i*7).into();
        match board[spot] {
            0 => {
                if i == 5{
                    board[spot] = team;
                    return Ok(board)
                }
                continue;
            }
            _ => {
                if i == 0{
                    return Err("invalid spot")
                }
                let one_spot_back:usize = (place+(i-1)*7).into();
                board[one_spot_back] = team;
                return Ok(board)
            }
            
        }
    }
    return Err("no spots");
}

fn check_win(&board: &[u8; 42]) -> Option<u8>{
    //println!("OUTCOME");
    //display_board(&board);
    for i in 0..42 {
        let row:u8 = i/7;
        let col:u8 = i%7;
        let index = i as usize;
        //println!("index: {} col: {} row: {}",i,col,row);
        let team: u8 = board[index];
        if team == 0{
            continue;
        }
        if row < 3{
            if team == board[index + 7] && team == board[index + 14] && team == board[index + 21]{
                return Some(team);
            }
            if col <= 3{
                if team == board[index + 8] && team == board[index + 16] && team == board[index + 24]{
                    return Some(team);
                }
            }
            if col >= 3{
                if team == board[index + 6] && team == board[index + 12] && team == board[index + 18]{
                    return Some(team);
                }
            }
        }
        if col < 4{
            if team == board[index + 1] && team == board[index + 2] && team == board[index + 3]{
                return Some(team);
            }
        }
    }

    return None
}
fn display_board(board: &[u8; 42]) {
    let mut col: u8 = 1; 
    println!("Connect4");
    for pos in board {
        print!("|{}|",pos);
        if col == 7{
            println!("");
            col = 1;
        }else{
            col += 1;
        }
    }
    println!("_____________________");
    println!("|0||1||2||3||4||5||6|");
}
fn solve(board: &[u8; 42],is_maximixing_player:bool) -> u8{
    let start_time = Instant::now();

    let mut threads= vec![];
    for i in 0..7{
        let result = place(&board,i,if is_maximixing_player{2}else{1});
        match result{
            Ok(new_board) => {

                let handle = thread::spawn(move || {
                    let thread_start = Instant::now();
                    let value = minimax(&new_board, DEPTH, is_maximixing_player,-f32::INFINITY, f32::INFINITY);
                    println!("Col: {} value {} time: {}",i,value,thread_start.elapsed().as_secs_f32());
                    return (value,i);
                });
                threads.push(handle);
    

            }
            Err(e) => println!("{}",e)
        }
    }
    let mut best_score = if is_maximixing_player{ f32::INFINITY } else {-f32::INFINITY};
    let mut action = 0;
    
    for thread in threads{
        let data = thread.join().unwrap();
        let value = data.0;
        let i = data.1;

        if (is_maximixing_player && best_score > value) || (!is_maximixing_player && best_score < value){
            best_score = value;
            action = i;
        }
    }

    // it prints '2'
    println!("Threads took {} seconds", start_time.elapsed().as_secs_f32());
    return action;
}
fn evaluate_direction(offsets: [usize;3],index:usize,board: &[u8; 42],team:u8) -> Result<u8,u8>{
    let mut counter_down = 0;
    if index + offsets[0] <= 41 && team == board[index + offsets[0]] {
        counter_down+=1;
    }
    if index + offsets[1] <= 41 &&team == board[index + offsets[1]]{
        counter_down+=1;
    }
    if  index + offsets[2] <= 41 && team == board[index + offsets[2]]{
        counter_down+=1;
    }
    if counter_down == 3{
        return Err(team)
    }
    return Ok(counter_down);
}
fn evaluate_board(board: &[u8; 42],is_maximixing_player:bool,depth:u8) -> f32{
    let mut p1_score = 0.0;
    let mut p2_score = 0.0;
    for i in 0..42 {
        let row:u8 = i/7;
        let col:u8 = i%7;
        let index = i as usize;
        //println!("index: {} col: {} row: {}",i,col,row);
        let team: u8 = board[index];
        let mut score = 0.0;
        if team == 0{
            continue;
        }
        let down = evaluate_direction([7,14,21],index,&board,team);
        match down{
            Ok(counter) => {
                score += 2.0_f32.powf(counter as f32);
            },
            Err(won_team) => {
                if won_team == 1 {return 100.0*depth as f32} else {return -100.0*depth as f32}
            }
        }
        let down_right = evaluate_direction([8,16,24],index,&board,team);
        match down_right{
            Ok(counter) => {
                score += 2.0_f32.powf(counter as f32);
            },
            Err(won_team) => {
                if won_team == 1 {return 100.0*depth as f32} else {return -100.0*depth as f32}
            }
        }
        let down_left = evaluate_direction([6,12,18],index,&board,team);
        match down_left{
            Ok(counter) => {
                score += 2.0_f32.powf(counter as f32);
            },
            Err(won_team) => {
                if won_team == 1 {return 100.0*depth as f32} else {return -100.0*depth as f32}
            }
        }
        
        let right = evaluate_direction([1,2,3],index,&board,team);
        match right{
            Ok(counter) => {
                score += 2.0_f32.powf(counter as f32);
            },
            Err(won_team) => {
                if won_team == 1 {return 100.0*depth as f32} else {return -100.0*depth as f32}
            }
        }
        if team == 1{
            p1_score += score ;
        }else{
            p2_score += score ;
        }

    }
    
    
    let final_score = p1_score as f32 - p2_score as f32;

    return final_score;

}
fn hash_board(board: &[u8; 42]) -> String{
    let mut board_buffer = [0u8; 5];
    let mut board_hash = String::new();
    for number in 0..42 {
        board_hash.push_str(board[number].numtoa_str(3, &mut board_buffer));
    }
    return board_hash;

}


fn minimax(&board: &[u8; 42], depth: u8,is_maximixing_player:bool,mut alpha:f32,mut beta:f32) -> f32{

    if depth == 0{
        return evaluate_board(&board,is_maximixing_player,depth);
    }
    let outcome = check_win(&board);
    if outcome.is_some(){
        let winner = outcome.unwrap();
        if winner == 1 {
            //previous_lookups.insert(hashed,f32::INFINITY/(DEPTH-depth) as f32);
            return 100.0*depth as f32;
        }else{
            //previous_lookups.insert(hashed,-f32::INFINITY/(DEPTH-depth) as f32);
            return -100.0*depth as f32;
        }
    }
    if depth == 0{
        return evaluate_board(&board,is_maximixing_player,depth);
    }
    if is_maximixing_player{
        let mut best_score = -f32::INFINITY;
        for col in 0..7{
            let result = place(&board,col,1);
            match result{
                Ok(new_board) => {
                    let value = minimax(&new_board, depth-1, false,alpha,beta);
                    best_score = best_score.max( value);
                    alpha = alpha.max(best_score);
                    if beta <= alpha{
                        break
                    }
                }
                Err(_) => {}
            }
        }
        return best_score

    }else{
        let mut best_score = f32::INFINITY;
        for col in 0..7{
            let result = place(&board,col,2);
            match result{
                Ok(new_board) => {
 //                   display_board(&board);
                    let value = minimax(&new_board, depth-1, true,alpha,beta);
                    best_score = best_score.min( value);
                    beta = beta.min(best_score);
                    if beta <= alpha{
                        break
                    }
                }
                Err(_) => {}
            }
        }
        return best_score

    }

}
