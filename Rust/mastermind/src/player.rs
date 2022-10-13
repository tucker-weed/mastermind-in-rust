use rand::Rng;

pub struct MastermindPlayer {
  pub colors: Vec<u32>
}

impl MastermindPlayer {
  const NUMBER_OF_COLORS: u32 = 6;
  const LIST_SIZE: u32 = 4;

  pub fn pick_colors(&mut self) {
    for _ in 0..MastermindPlayer::LIST_SIZE {
      self.colors.push(rand::thread_rng().gen_range(0..MastermindPlayer::NUMBER_OF_COLORS))
    }
    println!("Server picked {:?}", self.colors); 
  }

  pub fn grade(&self, client_string: &str) -> (bool, String) {
    let client_colors: Vec<&str> = client_string.split_whitespace().collect();
    if client_colors.len() != MastermindPlayer::LIST_SIZE.try_into().unwrap() {
      println!("Wrong number of colors from client: {:?}", client_colors);
      let default_answer_string = String::new();
      let default_answer_string = default_answer_string + "0, 0\n";
      return (false, default_answer_string);
    }
    let mut correct_colors = 0;
    let mut correct_location = 0;
    for i in 0..MastermindPlayer::LIST_SIZE {
      let i = i as usize;
      let int_color: u32 = client_colors[i].parse::<u32>().expect("Client passed invalid non-numeric input");
      if self.colors.contains(&int_color) {
        correct_colors += 1;
      }
      if int_color == self.colors[i] {
        correct_location += 1;
      }
    }
    let done = (correct_colors == 4) && (correct_location == 4);
    return (done, format!("{0} {1}\n", correct_colors, correct_location));
  }
}
