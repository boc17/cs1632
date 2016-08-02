extern crate rand;
use std::io;
use std::io::Error;
use rand::Rng;


struct Game {
   win: u32,
   loss: u32,
   tie: u32,
   total: u32,
}


struct Thrown{
    Rock: u32,
    Paper: u32,
    Scissors: u32,
}

enum GameStatus {
    PlayerWin,
    PlayerLoss,
    Tie
}


fn main() {
    
    let mut g: Game = Game {win: 0, loss: 0, tie: 0, total: 0};
    let mut choice: Thrown = Thrown {Rock: 0, Paper: 0, Scissors: 0};
    let mut done = false;
    let mut finished = false;
    let mut s = String::from("Hello");
    
    while !finished{
    while !done {
    
       match get_a_string() {
           Ok(n) => s = n,
           Err(n) => {
                 println!("ERROR COULD NOT READ!");
                 s = String::from("ERROR");
              }
       }
      
      update_choice(&mut choice, &s);
      let s = s.trim();
      done = compare(s.to_string());
      finished = compare_quit(s.to_string());  
    }
    
    
    
    let computer = random_guess();
    
    let mut computer_answer = String::from("Hello");
    
    //give value
       if computer == 0
       {
         computer_answer = String::from("r");
         println!("Opponent chose: Rock");
       }
       else if computer == 1
       {
       
         computer_answer = String::from("p");
         println!("Opponent chose: Paper");
       }
       if computer == 2
       {
        
         computer_answer = String::from("s");
         println!("Opponent chose: Scissors");
       }
    
    //
  
    if finished == true   //if user wants to quit
    {   
       println!("Game Over, User Quits");
    }
    else
    {
	       match check_for_win(&s, &computer_answer) {
			  GameStatus::PlayerWin => println!("You Win!"),
			  GameStatus::PlayerLoss => println!("You Lose!"),
			  GameStatus::Tie => println!("It's a tie!"),
	    }
    
	    if s.trim().to_string() == computer_answer.trim()
	    {
		println!("ITS A TIE");
		change_tie(&mut g);

	    }
	    else if (s.trim() == "r" )& (computer_answer.trim() == "s")
	    {  
	       change_win(&mut g);
	    }
	    else if( s.trim() == "s" )& (computer_answer.trim() == "p")
	    {
	       change_win(&mut g);

	     }
	     else if (s.trim().to_string() == "p" )& (computer_answer.trim() == "r")
	     {
		change_win(&mut g);
	     }
	     else
	     {
	       println!("LOSS user picks {}   computer picks {}", s, computer_answer);
	       change_loss(&mut g);
	     }


     }
    
     done = false;
    }
    
   
    
   println!("Wins {}", g.win);
   println!("Ties {}", g.tie);
   println!("Losses {}", g.loss);  
   println!("total games {}", g.total);
   println!("Rocks {}", choice.Rock);
   println!("Papers {}", choice.Paper);
   println!("Scissors {}", choice.Scissors);
}


fn compare_quit(string: String) -> bool {
    
    if string == "q".to_string() {
        return true;
    }    
    return false;
}




fn compare(string: String) -> bool {

    if string == "q".to_string() {
        println!("quit");
        return true;
    }
    else if string == "r".to_string(){
    
    	println!("Player chose: Rock");
    	return true;
    }
        else if string == "p".to_string(){
        
        	println!("Player chose: Paper");
        	return true;
    }
        else if string == "s".to_string(){
        
        	println!("Player chose: Scissors");
        	return true;
    }
    
    else
    {
       println!("Please enter r, p, s, or q!!!!!");
       return false;
    }
}

fn update_choice(mut input_change: &mut Thrown, user: &String){

    if user.trim() == "r"
    {
             
               input_change.Rock = input_change.Rock + 1;
               
    }
    if user.trim() == "p"
           {
             
               input_change.Paper = input_change.Paper + 1;
               
       }
       if user.trim() == "s"
              {
                
                 input_change.Scissors = input_change.Scissors + 1;
                  
       }

}


fn change_win(mut input: &mut Game) {
   input.win = input.win + 1;
   input.total = input.total + 1;   
   
}

fn change_loss(mut input: &mut Game) {
   input.loss = input.loss + 1;
   input.total = input.total + 1;   
   
}

fn change_tie(mut input: &mut Game) {
   input.tie = input.tie + 1;
   input.total = input.total + 1;   
   
}



fn get_a_string() -> Result<String, Error> {
    println!("Enter choice (r,p,s) or q to quit >");
    let mut to_return = String::new();
    io::stdin().read_line(&mut to_return).expect("FAIL");
    Ok(to_return)
}


fn random_guess() -> u32{

  let random_number = rand::thread_rng().gen_range(0,3);

  return random_number;
}






fn check_for_win(user: &String, computer: &String) -> GameStatus {

   
       if user.trim().to_string() == computer.trim()
       {
         
           return GameStatus::Tie;
           
       }
       else if (user.trim() == "r" )& (computer.trim() == "s")
       { 
          return GameStatus::PlayerWin;
       }
       else if( user.trim() == "s" )& (computer.trim() == "p")
       {
          return GameStatus::PlayerWin;
          
        }
        else if (user.trim().to_string() == "p" )& (computer.trim() == "r")
        {
           return GameStatus::PlayerWin;
        }
        else
        {
          return GameStatus::PlayerLoss;
     }
}
