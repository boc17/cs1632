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
    rock: u32,
    paper: u32,
    scissors: u32,
}

enum GameStatus {
    PlayerWin,
    PlayerLoss,
    Tie
}


fn main() {
    
    //give default values to the structs
    let mut g: Game = Game {win: 0, loss: 0, tie: 0, total: 0};
    let mut choice: Thrown = Thrown {rock: 0, paper: 0, scissors: 0};
    
    //done and finished will stop the loops
    let mut done = false;
    let mut finished = false;
    let mut s = String::from("Hello");
    
    while !finished{
    while !done {
    
       //return the user input    
       match get_a_string() {
           Ok(n) => s = n,
           Err(n) => {
                 println!("ERROR COULD NOT READ!");
                 s = String::from("ERROR");
                 println!("{}", n);
              }
       }
      
      update_choice(&mut choice, &s);
      let s = s.trim();
      
      done = compare(s.to_string());
      finished = compare_quit(s.to_string());  
    }
    
    
    
    if finished == true   //if user wants to quit
    {   
           println!("Player Stats:");
    }
    else
    {
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
  
 
    
	       match check_for_win(&s, &computer_answer) {
			  GameStatus::PlayerWin => println!("You Win!"),
			  GameStatus::PlayerLoss => println!("You Lose!"),
			  GameStatus::Tie => println!("It's a tie!"),
	    }
    
	    if s.trim().to_string() == computer_answer.trim()
	    {
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
	       change_loss(&mut g);
	     }


     }
    
     done = false;
    }
    
   
    print_results(g, choice);
   
}


fn print_results(input: Game, choice: Thrown){

	if input.total == 0
	{
	   println!("Wins 0.00%");
	   println!("Ties 0.00%");
	   println!("Losses 0.00%");  
	   println!("Rocks 0");
	   println!("Papers 0");
           println!("Scissors 0");
	}
	else
	{
	           let wins: f32 = ((input.win as f32)/(input.total as f32))*100.0;
	           let losses: f32 = ((input.loss as f32)/(input.total as f32))*100.0;
	           let ties: f32 = ((input.tie as f32)/(input.total as f32))*100.0;
	   
		   println!("Wins {:.2}%", wins);
		   println!("Ties {:.2}%", ties);
		   println!("Losses {:.2}%", losses);  
		   println!("Rocks {}", choice.rock);
		   println!("Papers {}", choice.paper);
                   println!("Scissors {}", choice.scissors);
         }


}





//check for input of q
fn compare_quit(string: String) -> bool {
    
    if string == "q".to_string() {
        return true;
    }    
    return false;
}



//check if valid input of (q,r,p,s)
fn compare(string: String) -> bool {

    if string == "q".to_string() {
    
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
             
               input_change.rock = input_change.rock + 1;
               
    }
    if user.trim() == "p"
    {
             
               input_change.paper = input_change.paper + 1;
               
    }
    if user.trim() == "s"
    {
                
                 input_change.scissors = input_change.scissors + 1;
                  
    }

}


//increment win and total
fn change_win(mut input: &mut Game) {
   input.win = input.win + 1;
   input.total = input.total + 1;   
   
}

//increment loss and total
fn change_loss(mut input: &mut Game) {
   input.loss = input.loss + 1;
   input.total = input.total + 1;   
   
}

//increment tie and total
fn change_tie(mut input: &mut Game) {
   input.tie = input.tie + 1;
   input.total = input.total + 1;   
   
}


//get user input
fn get_a_string() -> Result<String, Error> {
    println!("Enter choice (r,p,s) or q to quit >");
    let mut to_return = String::new();
    io::stdin().read_line(&mut to_return).expect("FAIL");
    Ok(to_return)
}

//random number for computer guess
fn random_guess() -> u32{

  let random_number = rand::thread_rng().gen_range(0,3);

  return random_number;
}





//return correct outcome of which player wins
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

