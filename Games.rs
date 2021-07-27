/*
 File of various games. Monty Hall, Fares problem

*/


//The Monty Hall problem simulation, returns [0,1] as u64 due to ease of summation in large scale simulations. 0 == false or a loss, 1 = victory 
fn monty_hall()-> u64{

    const doors : [&str;3] = ["goat","car","goat"]; 
    let mut openess : [bool;3] = [false, false, false]; //also used for selection

    let mut player_select = (rand()%3) as usize ; //for greater randomness use proper intervals %3 returns too many 0s
    openess[player_select]=true;
    let mut host_select = (rand()%2 + 1) as usize;
    let mut host_open = (player_select + host_select) %3;
    
  if host_open == 1{
             if player_select==2{
                 host_open=0;
             }
             else{
                 host_open=2;
             }
             
  }
  openess[host_open]=true;

    player_select=openess.iter().position(|x| *x==false).unwrap();
    
    if doors[player_select] == "goat"{
    return 0u64
    }
    else{
    return 1u64
    }
 }
 
 
 // The Fares problem. So named after Reddit user Fares26597 who posed the question of whether or not it is more effective to move
 // to avoid capture from a randomly moving monster on a grid. The program shows that moving is infact better. Originally written by the author in C++
 
 fn fares_problem()->u64{
                             //The size of the grid to use
    const GRID : u64 = 500_000;
    
    let  (mut demon_pos, mut victim_pos, mut movement) = (5u64,6u64,0u64);
    
    while demon_pos !=victim_pos
    {
        demon_pos = rand() % GRID;
        victim_pos = rand() % GRID; //comment to keep victim in place
        movement+=1;
    }
   movement
  }
 
