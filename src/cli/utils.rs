use rand::Rng;

pub fn generate_random_number() -> i32 {
  let mut rng = rand::thread_rng();
  rng.gen_range(1..=1000000)
}

pub fn select_random_stock<F>(mut candidates: Vec<String>, mut rand_fn: F) -> Option<String>
where
  F: FnMut() -> i32,
{
  if candidates.is_empty() {
    return None;
  }

  while candidates.len() > 2 {
    let mid = candidates.len() / 2;
    if rand_fn() % 2 == 0 {
      candidates = candidates[..mid].to_vec();
    } else {
      candidates = candidates[mid..].to_vec();
    }
  }

  if candidates.len() == 2 {
    if rand_fn() % 2 == 0 {
      Some(candidates[0].clone())
    } else {
      Some(candidates[1].clone())
    }
  } else {
    Some(candidates[0].clone())
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_select_from_empty_list() {
    let result = select_random_stock(vec![], || 1);
    assert_eq!(result, None);
  }

  #[test]
  fn test_select_from_single_item() {
    let stocks = vec!["AAPL".to_string()];
    let result = select_random_stock(stocks, || 999);
    assert_eq!(result, Some("AAPL".to_string()));
  }

  #[test]
  fn test_select_always_left() {
    let stocks = vec![
      "A".to_string(),
      "B".to_string(),
      "C".to_string(),
      "D".to_string(),
    ];

    let result = select_random_stock(stocks, || 2);
    assert_eq!(result, Some("A".to_string()));
  }

  #[test]
  fn test_select_always_right() {
    let stocks = vec![
      "A".to_string(),
      "B".to_string(),
      "C".to_string(),
      "D".to_string(),
    ];

    let result = select_random_stock(stocks, || 1);
    assert_eq!(result, Some("D".to_string()));
  }

  #[test]
  fn test_select_from_two_items_picks_first() {
    let stocks = vec!["A".to_string(), "B".to_string()];
    let result = select_random_stock(stocks, || 2);
    assert_eq!(result, Some("A".to_string()));
  }

  #[test]
  fn test_select_from_two_items_picks_second() {
    let stocks = vec!["A".to_string(), "B".to_string()];
    let result = select_random_stock(stocks, || 1);
    assert_eq!(result, Some("B".to_string()));
  }

  #[test]
  fn test_select_alternating() {
    let stocks: Vec<String> = ["A", "B", "C", "D", "E", "F", "G", "H"]
      .iter()
      .map(|s| s.to_string())
      .collect();

    let mut sequence = [2, 1, 2].iter().cycle();
    let result = select_random_stock(stocks, || *sequence.next().unwrap());

    assert_eq!(result, Some("C".to_string()));
  }
}
