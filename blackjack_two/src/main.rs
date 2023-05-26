use rand::Rng;
use std::io;

fn generate_shuffled_deck()->Vec<i32>{
let mut cards_deck:Vec<i32>=Vec::new();
let mut shuffled_deck:Vec<i32>=Vec::new();
let mut cards_deck_length=51;  
let mut card=13;
while card>0{
    cards_deck.push(card);
    cards_deck.push(card);
    cards_deck.push(card);
    cards_deck.push(card);
    card-=1;
    }
while cards_deck_length>0{
    let number=rand::thread_rng().gen_range(0..=cards_deck_length);
    let random_card=cards_deck[number];   
    shuffled_deck.push(random_card);
    cards_deck.remove(number);
    cards_deck_length-=1;
    if cards_deck_length==0{  
        let final_card=cards_deck[0];
        shuffled_deck.push(final_card);
        }
    }
shuffled_deck
}

fn print_player_hand(player_hand:&Vec<i32>)->i32{
    println!("Player hand");
    let mut hand_value=0;
    let mut aces=0;
    for i in player_hand{
       println!("{}",*i);
       if *i<2{aces+=1;}
       if *i>10{hand_value+=10;}
       else{hand_value+=i;}         //does ace get added as 1?                 
       }
if aces>0{hand_value+=10}
if (hand_value>21)&(aces>0){hand_value-=10}                            
println!("player hand value {}",&hand_value);
hand_value
}

fn print_dealer_hand(dealer_hand:&Vec<i32>)->i32{
    let mut hand_value=0;
    let mut print_card=0;
    let mut aces=0;
    println!("Dealer hand");
    for i in dealer_hand{
        if print_card==0{
        println!("X");
        }else{
        println!("{}",*i);
        }
        if *i<2{aces+=1;}
        if *i>10{hand_value+=10;}
        else{hand_value+=i;}       
    print_card+=1;
    }
if aces>0{hand_value+=10}
if (hand_value>21)&(aces>0){hand_value-=10}
if dealer_hand[0]>9{println!("dealer hand value greater than {}",&hand_value-10);}
else{println!("dealer hand value greater than {}",&hand_value-dealer_hand[0]);}
hand_value    
}

fn player_move()->i32{
    let mut h_or_s=0;
    println!("player 1= hit or 2=stand");   
    let mut guess = String::new();
    io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read line");
    let guess: u32 = guess.trim().parse().expect("Please type a number!");
    match guess {
        1=> h_or_s+=1,
        2=> h_or_s+=2, 
        _=> println!("1 for hit, 2 for stand"),  
        }
h_or_s
}

fn print_final_dealer_hand(dealer_hand:&Vec<i32>){
    println!("Dealer full hand");
    let mut hand_value=0;
    let mut aces=0;
    for i in dealer_hand{
       println!("{}",*i);
       if *i<2{aces+=1;}
       if *i>10{hand_value+=10;}
       else{hand_value+=i;}                          
       }
if aces>0{hand_value+=10}
if (hand_value>21)&(aces>0){hand_value-=10}                            
println!("final value of dealer hand {}",&hand_value);
}

fn main(){
loop{
let mut deck=generate_shuffled_deck();
let mut player_hand:Vec<i32>=Vec::new();
let mut dealer_hand:Vec<i32>=Vec::new();
let mut cards_to_deal=2;

println!("   ");
println!("------------");
while cards_to_deal>0{      
    player_hand.push(deck[0]);
    deck.remove(0);
    dealer_hand.push(deck[0]);
    deck.remove(0);
    cards_to_deal-=1;      
    }                                          
let dealer_total=print_dealer_hand(&dealer_hand);   
let player_total=print_player_hand(&player_hand);
println!("  ");
while deck.len()>15{
if player_total==21{println!("Blackjack!!! Player wins!");
    break}
if dealer_total==21{println!("Blackjack!!! Dealer wins!");
    break}
println!("  ");
let hit_or_stand=player_move();
match hit_or_stand{ 
1=>{
    player_hand.push(deck[0]);       
    deck.remove(0);
    print_dealer_hand(&dealer_hand);  
    let player_total=print_player_hand(&player_hand);
    if player_total==21{println!("Blackjack!!! Player wins!");
        break}
    if player_total>21{println!("Busted!!! Dealer wins!");
        break}
    println!("  ");
    },
2=>{                                          
    println!("  ");
    loop{
    let player_total=print_player_hand(&player_hand);        
    if dealer_total>player_total{println!("Dealer wins!");
        break}  
    dealer_hand.push(deck[0]);      
    deck.remove(0);
    let dealer_total=print_dealer_hand(&dealer_hand);
    if dealer_total>21{println!("Busted!!! Player wins!");
        break}
    if dealer_total>player_total{println!("Dealer wins!");
        break}
    if dealer_total==21{println!("Blackjack!!! Dealer wins!");
        break}
    }
    break
    },    
_=>println!("enter 1 for hit and 2 for stand"),
}
}
print_final_dealer_hand(&dealer_hand);
println!("------------");
println!("   ");
}
}














