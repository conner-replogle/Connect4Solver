use std::{collections::HashMap, thread::{self, JoinHandle}, time::Instant};
use numtoa::NumToA;
use rayon::prelude::*;

const DEPTH: u8 = 12;
fn main() {
    println!("DEPTH:{}",DEPTH);
    //asdasdasdasdasd asfjaf ladjdslflj
    let mut master_previous_lookups:HashMap<String,f32> = HashMap::new();
    //7 * 6 = 42
    let mut board: [u8; 42] = [0; 42];
    display_board(&board);
    let mut team = 1;
    let done = false;
    while !done {
        let mut action = 0;
        if team == 1{
            print!("INPUT ACTION:");
            let mut input_text = String::new();
            std::io::stdin()
                .read_line(&mut input_text)
                .expect("failed to read from stdin");
        
            let trimmed = input_text.trim();
            match trimmed.parse(){
                Ok(i) => action = i,
                Err(..) => {println!("this was not an integer: {}", trimmed);continue;},
            };
            
        }else{
            println!("Master HashMap Length{}",master_previous_lookups.len());
            let start_time = Instant::now();

            let mut threads= vec![];
            for i in 0..7{
                let result = place(&board,i,2);
                match result{
                    Ok(new_board) => {
                        let mut previous_lookups:HashMap<String,f32> = master_previous_lookups.clone();

                        let handle = thread::spawn(move || {
                            let thread_start = Instant::now();
                            let value = minimax(&new_board, DEPTH, false,-f32::INFINITY, f32::INFINITY,&mut previous_lookups);
                            println!("Col: {} value {} time: {}",i,value,thread_start.elapsed().as_secs_f32());
                            return (value,previous_lookups,i);
                        });
                        threads.push(handle);
            

                    }
                    Err(e) => println!("{}",e)
                }
            }
            let mut best_score = -f32::INFINITY;
            let mut total_lookup_time = 0.0;
            let mut count = 0;            for thread in threads{
                let data = thread.join().unwrap();
                let value = data.0;
                let i = data.2;
                let hashmap = data.1;
                if best_score < value{
                    best_score = value;
                    action = i;
                }
                let lookup_time = Instant::now();
                master_previous_lookups.extend(hashmap);
                let lookup_time_elapsed = lookup_time.elapsed().as_secs_f32();
                total_lookup_time+=lookup_time_elapsed;
                count+=1;
            }
            println!("Lookup took average of {} seconds with a total of {}",total_lookup_time/count as f32,total_lookup_time);
        
            // it prints '2'
            println!("Threads took {} seconds", start_time.elapsed().as_secs_f32());
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
    return Err("No Spots");
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
fn evaluate_board(board: &[u8; 42],previous_lookups:&mut HashMap<String,f32>,is_maximixing_player:bool) -> f32{
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
        if row < 3{
            if team == board[index + 7] {
                score += 2.0;
            }
            if team == board[index + 14]{
                score += 2.0;
            }
            if team == board[index + 21]{
                score += 2.0;
            }
            if col <= 3{
                if team == board[index + 8] {
                    score += 2.0;
                }
                
                if team == board[index + 16]{
                    score += 2.0;
                
                }
                if team == board[index + 24]{
                    score += 2.0;
                }
            }
            if col >= 3{
                if team == board[index + 6] {
                    score += 2.0;
                    
                }
                if team == board[index + 12]{
                    score += 2.0;
                    
                }
                if team == board[index + 18]{
                    score += 2.0;
                }
            }
        }
        if col < 4{
            if team == board[index + 1] {
                score += 2.0;
            }
            if team == board[index + 2]{
                score += 2.0;
            }
            if team == board[index + 3]{
                score += 2.0;
            }
        }
        if team == 1{
            p1_score += score;
        }else{
            p2_score += score;
        }

    }
    if is_maximixing_player{
        p2_score *= p2_score;
    }else{
        p1_score *= p1_score;
    }
    
    
    let final_score = p2_score as f32-p1_score as f32;
    previous_lookups.insert(hash_board(&board),final_score );

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


fn minimax(&board: &[u8; 42], depth: u8,is_maximixing_player:bool,mut alpha:f32,mut beta:f32,previous_lookups:&mut HashMap<String,f32>) -> f32{
    //display_board(&board);
    let hashed = hash_board(&board);
    let already_searched = previous_lookups.get(&hashed);
    match already_searched{
        Some(value) => {return *value},
        _ => {}
    }
    let outcome = check_win(&board);
    if outcome.is_some(){
        let winner = outcome.unwrap();
        if winner == 1 {
            previous_lookups.insert(hashed,-f32::INFINITY);
            return -f32::INFINITY;
        }else{
            previous_lookups.insert(hashed,f32::INFINITY);
            return f32::INFINITY;
        }
    }
    if depth == 0{
        return evaluate_board(&board,previous_lookups,is_maximixing_player);
    }
    if is_maximixing_player{
        let mut best_score = -f32::INFINITY;
        for col in 0..7{
            let result = place(&board,col,2);
            match result{
                Ok(new_board) => {
                    let value = minimax(&new_board, depth-1, false,alpha,beta,previous_lookups);
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
            let result = place(&board,col,1);
            match result{
                Ok(new_board) => {
 //                   display_board(&board);
                    let value = minimax(&new_board, depth-1, true,alpha,beta,previous_lookups);
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
