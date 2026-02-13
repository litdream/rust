// Simple Definition
enum TypedPlayerState {
    Idle,      // The player is standing still
    Running,   // The player is moving
    Jumping,   // The player is in the air}
}

impl TypedPlayerState {
    // This is a method. 'self' refers to the current value of the enum.
    fn can_interact(&self) -> bool {
        match self {
            // You can only open a chest if you are standing still (Idle)
            TypedPlayerState::Idle => true,
            // You can't open a chest while moving or in mid-air
            TypedPlayerState::Running | TypedPlayerState::Jumping => false,
        }
    }

    // You can also have methods that return strings
    fn status_message(&self) -> &str {
        match self {
            TypedPlayerState::Idle => "Standing ready.",
            TypedPlayerState::Running => "Moving fast!",
            TypedPlayerState::Jumping => "In the air!",
        }
    }
}

fn check_player_state() {
    let mut current_state = TypedPlayerState::Idle;

    // We use the dot operator (.) to call methods, just like a struct
    if current_state.can_interact() {
        println!("Action: {}", current_state.status_message());
        println!("You opened the chest!");
    } else {
        println!("You are too busy to do that right now.");
    }

    current_state = TypedPlayerState::Jumping;
    if ! current_state.can_interact() {
	println!("I'm busy:  {}", current_state.status_message());
    }
}



// Enum with Data
enum PlayerState {
    Idle,                               // No data
    Running(f32),                       // Tuple-style: contains a speed (f32)
    Teleporting { x: i32, y: i32 },      // Struct-style: contains named coordinates
}

impl PlayerState {
    fn describe(&self) {
        match self {
            // No data to unpack here
            PlayerState::Idle => {
                println!("The player is standing still.");
            }
            // We "catch" the speed value and give it a temporary name 's'
            PlayerState::Running(s) => {
                println!("The player is sprinting at {} m/s!", s);
            }
            // We unpack the named fields 'x' and 'y'
            PlayerState::Teleporting { x, y } => {
                println!("The player is warping to coordinates ({}, {}).", x, y);
            }
        }
    }
}

fn player_activity() {
    
    // Creating variants with data
    let state1 = PlayerState::Idle;
    let state2 = PlayerState::Running(15.5);
    let state3 = PlayerState::Teleporting { x: 100, y: -50 };

    // Calling the method
    state1.describe();
    state2.describe();
    state3.describe();
}


// Option Example
use std::mem::size_of;
use std::num::NonZeroU32;

/*
Rust Option definition:

enum Option<T> {
    None,
    Some(T),
}
 */


fn option_example() {
    let some_ptr: Option<&i32> = Some(&10);
    let none_ptr: Option<&i32> = None;

    // A standard integer has no "niches" (any bit pattern is valid)
    println!("Size of u32: {} bytes", size_of::<u32>());          // 4
    println!("Size of Option<u32>: {} bytes", size_of::<Option<u32>>()); // 8 (4 + 1 tag + 3 padding)
    println!("---");

    // A reference has a niche (it can't be 0)
    println!("Size of &u32: {} bytes", size_of::<&u32>());         // 8
    println!("Size of Option<&u32>: {} bytes", size_of::<Option<&u32>>()); // 8 (MAGIC! No extra tag)
    println!("---");

    // NonZeroU32 has a niche (it can't be 0)
    println!("Size of NonZeroU32: {} bytes", size_of::<NonZeroU32>()); // 4
    println!("Size of Option<NonZeroU32>: {} bytes", size_of::<Option<NonZeroU32>>()); // 4 (MAGIC!)
}    
    




fn main() {
    check_player_state();
    println!("------------");
    player_activity();
    println!("------------");

    option_example();
}


