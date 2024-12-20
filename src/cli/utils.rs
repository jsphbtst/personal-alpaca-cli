use reqwest::blocking::Client;
use rand::Rng;

fn generate_random_number() -> i32 {
    let mut rng = rand::thread_rng();
    rng.gen_range(1..=1000000)
}

pub fn generate_random_number_api() -> i32 {
  let client = Client::new();
  let url = "https://www.random.org/integers/?num=1&min=1&max=1000000&col=1&base=10&format=plain&rnd=new";
  match client.get(url).send() {
    Ok(response) => match response.text() {
      Ok(number_str) => number_str.trim().parse::<i32>().unwrap_or(generate_random_number()),
      Err(_) => generate_random_number()
    },
    Err(_) => generate_random_number()
  }
}
