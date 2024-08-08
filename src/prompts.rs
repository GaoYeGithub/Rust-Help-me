use std::io;

// returns a boolean, accepts text input
fn prompt() -> bool {
  let mut input = String::new();
  io::stdin().read_line(&mut input).unwrap();
  return input.to_ascii_lowercase().starts_with("y");
}

// first prompt
pub fn intro() -> () {
  println!("Someone's calling you, do you answer?");

  // if prompt() returns true, that's enough for rust to carry on!
  // a few other expressions that also amount to true are 2 == 2,
  // 1 > 0, or just the word true.
  if prompt() {
    phone_answered();
  } else {
    println!("You hung up the phone. You didn't find out what was all that about. Maybe you will. One day...");
  }
}

// second prompt
pub fn phone_answered() -> () {
  println!("You hear the caller say: 'Hey, I know it's been a long time, you down to meet up?'");
  println!("Her voice seems vaguely familiar, and she sounds a bit distressed. Do you accept?");

  if prompt() {
    challenge_accepted();
  }
  else {
    println!("You hung up the phone. You didn't find out what was all that about. Maybe you will. One day...");
    println!("YOU WON! To be continued...");
  }
}

// third prompt
pub fn challenge_accepted() -> () {
  println!("You meet up with the old friend, and she hands you a bag full of cash.");
  println!("Your friend says: 'Here, don't ask questions, thanks.'");
  println!("Congratulations! You are now rich and have sufficiently covered your material necessities, but does that amount to true happiness? You may have won the game, but you haven't won at life.");
  println!("GAME OVER!");
}