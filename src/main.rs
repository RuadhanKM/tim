use std::{cmp::*, io::{stdin, stdout, Read, Write}, thread::{self, sleep}, time::Duration};
use crossterm::{cursor, queue, QueueableCommand};
use rand::prelude::*;

const TIMER_HEIGHT : i32 = 4;

const TIMER_TOP: &str = "
        |‾‾‾‾‾‾‾‾|            |‾‾‾‾‾‾‾‾|
________________________________________________
------------------------------------------------
";

const TIMER_ZERO: &str = "
  ___   
 / _ \\  
( (_) ) 
 \\___/  ";
const TIMER_ONE: &str = "
   __   
  /  )  
   )(   
  (__)  ";
const TIMER_TWO: &str = "
 ___    
(__ \\   
 / _/   
(____)  ";
const TIMER_THREE: &str = "
 ___    
(__ )   
 (_ \\   
(___/   ";
const TIMER_FOUR: &str = "
  __    
 /. |   
(_  _)  
 (_)    ";
const TIMER_FIVE: &str = "
 ___    
| __)   
|__ \\   
(___/   ";
const TIMER_SIX: &str = "
  _     
 / )    
/ _ \\   
\\___/   ";
const TIMER_SEVEN: &str = "
 ___    
(__ )   
 / /    
(_/     ";
const TIMER_EIGHT: &str = "
 ___    
( _ )   
/ _ \\   
\\___/   ";
const TIMER_NINE: &str = "
 ___    
/ _ \\   
\\_  /   
 (_/    ";
const TIMER_COLON: &str = "
   ()   
        
        
   ()   ";

const SLOT_MACHINE: &str = "
⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⢀⡀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀
⠀⠀⣀⣤⣴⣶⣶⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣶⣶⣤⣤⣀⠀⠀⠀⠀⠀
⠀⠀⢿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⡿⠀⠀⠀⠀⠀
⠀⠀⠀⢀⣀⣀⣀⣀⣀⣀⣀⣀⣀⣀⣀⣀⣀⣀⣀⣀⡀⠀⠀⠀⠀⠀⠀
⠀⠀⠀⢸⣿⠛⠛⠛⠛⠛⠛⠛⠛⠛⠛⠛⠛⠛⠛⣿⡇⠀⠀⣤⣄⠀⠀
⠀⠀⠀⢸⣿⠀⢸⣿⣿⡇⢸⣿⣿⡇⢸⣿⣿⡇⠀⣿⡇⠀⠀⠛⠛⠀⠀                  
⠀⠀⠀⢸⣿⠀⢸⣿⣿⡇⢸⣿⣿⡇⢸⣿⣿⡇⠀⣿⡇⠀⠀⣷⠀⠀⠀                  
⠀⠀⠀⢸⣿⠀⢸⣿⣿⡇⢸⣿⣿⡇⢸⣿⣿⡇⠀⣿⡇⠀⣾⡇⠀⠀⠀                  
⠀⠀⠀⢸⣿⣤⣤⣤⣤⣤⣤⣤⣤⣤⣤⣤⣤⣤⣤⣿⡇⠀⣿⡿⠀⠀                  
⠀⠀⠀⠈⠉⠉⠉⠉⠉⠉⠉⠉⠉⠉⠉⠉⠉⠉⠉⠉⠁⠀⠙⠃                     
⠀⢀⣴⣿⠟⠛⠛⢻⡿⠛⠛⠛⢻⣿⣿⡟⠋⠉⠉⠛⢿⣦⡀⠀⠀⠀⠀                  
⢰⣿⣿⣥⣤⣤⣤⣾⣧⣤⣤⣤⣿⣿⣿⣷⣦⣤⣤⣶⣿⣿⣿⡆⠀⠀⠀                
⠈⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⠀⠀⠀⠀                
⠀⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⠀⠀⠀⠀
⠀⠉⠉⠉⠉⠉⠉⠉⠉⠉⠉⠉⠉⠉⠉⠉⠉⠉⠉⠉⠉⠉⠉";

const SLOT_LEVER_PULL: &str = "
⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⢀⡀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀
⠀⠀⣀⣤⣴⣶⣶⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣶⣶⣤⣤⣀⠀⠀⠀⠀⠀
⠀⠀⢿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⡿⠀⠀⠀⠀⠀  
⠀⠀⠀⢀⣀⣀⣀⣀⣀⣀⣀⣀⣀⣀⣀⣀⣀⣀⣀⣀⡀⠀⠀⠀⠀⠀⠀  
⠀⠀⠀⢸⣿⠛⠛⠛⠛⠛⠛⠛⠛⠛⠛⠛⠛⠛⠛⣿⡇⠀⠀      
⠀⠀⠀⢸⣿⠀⢸⣿⣿⡇⢸⣿⣿⡇⢸⣿⣿⡇⠀⣿⡇⠀⠀⠀⠀       
⠀⠀⠀⢸⣿⠀⢸⣿⣿⡇⢸⣿⣿⡇⢸⣿⣿⡇⠀⣿⡇⠀⠀⠀⠀       
⠀⠀⠀⢸⣿⠀⢸⣿⣿⡇⢸⣿⣿⡇⢸⣿⣿⡇⠀⣿⡇⠀⠀⠀⠀      
⠀⠀⠀⢸⣿⣤⣤⣤⣤⣤⣤⣤⣤⣤⣤⣤⣤⣤⣤⣿⡇⠀⠀⠀⠀      ⣤⣄
⠀⠀⠀⠈⠉⠉⠉⠉⠉⠉⠉⠉⠉⠉⠉⠉⠉⠉⠉⠉⠁⠀⠙⣿⡿⣾⡇⣷⠀⠛⠛
⠀⢀⣴⣿⠟⠛⠛⢻⡿⠛⠛⠛⢻⣿⣿⡟⠋⠉⠉⠛⢿⣦⡀⠀⠀⠀⠀
⢰⣿⣿⣥⣤⣤⣤⣾⣧⣤⣤⣤⣿⣿⣿⣷⣦⣤⣤⣶⣿⣿⣿⡆⠀⠀⠀
⠈⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⠀⠀⠀⠀
⠀⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⠀⠀⠀⠀
⠀⠉⠉⠉⠉⠉⠉⠉⠉⠉⠉⠉⠉⠉⠉⠉⠉⠉⠉⠉⠉⠉⠉";

const SLOTS: &'static [&'static str] = &["💰", "💀", "❌", "❤️", "💎"];

const DIGITS: &'static [&'static str] = &[TIMER_ZERO, TIMER_ONE, TIMER_TWO, TIMER_THREE, TIMER_FOUR, TIMER_FIVE, TIMER_SIX, TIMER_SEVEN, TIMER_EIGHT, TIMER_NINE];

const TIMER_BOTTOM: &str = 
"________________________________________________
------------------------------------------------";

fn construct_timer_art(mut seconds_left: i32) -> String {
	seconds_left = min(seconds_left, 99*60 + 59);

	let dm1: i32 = (seconds_left - seconds_left%60)/60/10;
	let dm2: i32 = (seconds_left - seconds_left%60)/60%10;

	let ds1: i32 = (seconds_left%60)/10;
	let ds2: i32 = (seconds_left%60)%10;

	let mut fin_art = String::from(TIMER_TOP);
	for i in 0..TIMER_HEIGHT {
		fin_art.push_str("||  ");
		fin_art.push_str(&DIGITS[dm1 as usize].split("\n").collect::<Vec<&str>>()[i as usize+1]);
		fin_art.push_str(&DIGITS[dm2 as usize].split("\n").collect::<Vec<&str>>()[i as usize+1]);

		fin_art.push_str(TIMER_COLON.split("\n").collect::<Vec<&str>>()[i as usize+1]);

		fin_art.push_str(&DIGITS[ds1 as usize].split("\n").collect::<Vec<&str>>()[i as usize+1]);
		fin_art.push_str(&DIGITS[ds2 as usize].split("\n").collect::<Vec<&str>>()[i as usize+1]);
		fin_art.push_str("  ||\n");

	}
	fin_art.push_str(TIMER_BOTTOM);

	return fin_art;
}

fn main() {
	let args = std::env::args().collect::<Vec<String>>();

	if args.len() != 2 {
		println!("Usage: tim [time in seconds]");
		return ();
	}

	match args[1].parse::<i32>() {
		Ok(time) => {
			println!("{}", construct_timer_art(time));
			for i in 1..time+1 {
				sleep(Duration::from_secs(1));
				println!("\x1b[48D\x1b[11A");
				println!("{}", construct_timer_art(time-i));
			}

			let mut timer_is_beeping = true;
			let mut rng = rand::thread_rng();

			thread::spawn(move || {
				let mut music_rng = rand::thread_rng();
				while timer_is_beeping {
					actually_beep::beep_with_hz_and_millis(music_rng.gen_range(500..1200) as u32, 100).unwrap();
					sleep(Duration::from_millis(10));
				}
			});

			while timer_is_beeping {
				println!("Press Enter to Spin!{}", SLOT_MACHINE);
				let mut _s: String = String::from("");
				stdin().read_line(&mut _s).unwrap();

				queue!(stdout(), cursor::MoveUp(17)).unwrap();
				stdout().flush().unwrap();
				println!("Press Enter to Spin!{}", SLOT_LEVER_PULL);
				sleep(Duration::from_millis(300));
				queue!(stdout(), cursor::MoveUp(16)).unwrap();
				stdout().flush().unwrap();
				println!("Press Enter to Spin!{}", SLOT_MACHINE);


				let s1 = rng.gen_range(0..SLOTS.len());
				let s2 = rng.gen_range(0..SLOTS.len());
				let s3 = rng.gen_range(0..SLOTS.len());

				queue!(stdout(), cursor::SavePosition).unwrap();
				stdout().flush().unwrap();

				for _i in 0..rng.gen_range(10..15) {
					queue!(stdout(), cursor::MoveTo(6, 12)).unwrap();
					print!("{}", SLOTS[rng.gen_range(0..SLOTS.len())]);

					queue!(stdout(), cursor::RestorePosition).unwrap();
					stdout().flush().unwrap();
					sleep(Duration::from_millis(100));
				}

				queue!(stdout(), cursor::MoveTo(6, 12)).unwrap();
				print!("{}", SLOTS[s1]);
				queue!(stdout(), cursor::RestorePosition).unwrap();
				stdout().flush().unwrap();

				for _i in 0..rng.gen_range(25..30) {
					queue!(stdout(), cursor::MoveTo(10, 12)).unwrap();
					print!("{}", SLOTS[rng.gen_range(0..SLOTS.len())]);

					queue!(stdout(), cursor::RestorePosition).unwrap();
					stdout().flush().unwrap();
					sleep(Duration::from_millis(100));
				}

				queue!(stdout(), cursor::MoveTo(10, 12)).unwrap();
				print!("{}", SLOTS[s2]);
				queue!(stdout(), cursor::RestorePosition).unwrap();
				stdout().flush().unwrap();

				let mut speed = 8..12;
				if s1 == s2 { speed = 40..60 }
				for _i in 0..rng.gen_range(speed) {
					queue!(stdout(), cursor::MoveTo(14, 12)).unwrap();
					print!("{}", SLOTS[rng.gen_range(0..SLOTS.len())]);

					queue!(stdout(), cursor::RestorePosition).unwrap();
					stdout().flush().unwrap();
					sleep(Duration::from_millis(100));
				}

				queue!(stdout(), cursor::MoveTo(14, 12)).unwrap();
				print!("{}", SLOTS[s3]);
				queue!(stdout(), cursor::RestorePosition).unwrap();
				stdout().flush().unwrap();

				if (s1 == s2) && (s2 == s3) && (s1 == s3) {
					println!("\nYou Won! :)");
					timer_is_beeping = false;
				} else {
					sleep(Duration::from_secs(1));
					queue!(stdout(), cursor::MoveUp(16)).unwrap();
					stdout().flush().unwrap();
				}
			}
		}
		Err(_e) => {
			println!("Usage: tim [time in seconds]");
		}
	}
}