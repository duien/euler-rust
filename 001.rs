// If we list all the natural numbers below 10 that are multiples of 3 or 5,
// we get 3, 5, 6 and 9. The sum of these multiples is 23.

// Find the sum of all the multiples of 3 or 5 below 1000.

fn sum_multiples_up_to(max: int) -> int {
  let mut sum = 0;
  let mut num = 0;

  while num < max {
    if num % 3 == 0 || num % 5 == 0 {
      sum = sum + num;
    }
    num += 1;
  }
  sum
}

fn print_sum(max: int) {
  println!("sum to {} is {}", max, sum_multiples_up_to(max));
}

fn main() {
  print_sum(10);
  print_sum(1000);
}
