// The number, 1406357289, is a 0 to 9 pandigital number because it is made
// up of each of the digits 0 to 9 in some order, but it also has a rather
// interesting sub-string divisibility property.

// Let d1 be the 1st digit, d2 be the 2nd digit, and so on. In this way, we
// note the following:

// d2 * d3 * d4 = 406 is divisible by 2
// d3 * d4 * d5 = 063 is divisible by 3
// d4 * d5 * d6 = 635 is divisible by 5
// d5 * d6 * d7 = 357 is divisible by 7
// d6 * d7 * d8 = 572 is divisible by 11
// d7 * d8 * d9 = 728 is divisible by 13
// d8 * d9 * d10 = 289 is divisible by 17

// Find the sum of all 0 to 9 pandigital numbers with this property.

// extern mod std;
use std::vec;
use std::vec::Permutations;
use std::vec::ElementSwaps;

// fn permutations_iter(self) -> Permutations<T> {
//      Permutations{
//          swaps: ElementSwaps::new(self.len()),
//          v: self.to_owned(),
//      }
//  }

fn main() {
  // let mut base = do vec::from_fn(10) |i| {i as u8};
  let nums = range(0,9).to_owned_vec();
  let perms = Permutations {
    values: nums
  };
  println!("sigh {:?}", perms.next())

  // do 5.times() {
  //   let wat = Permutations::next(base);
  //   println!("something {}", wat)
  // }
  // let nums = vec::Permutations( range(0,9).to_owned_vec() );
  
  // impl Permutations for vec {
  // };

  // let p = Permutations { range(0,9).to_owned_vec() };

  // let permutations = nums.to_owned_vec();
  // println!("We have 2 3 4 as {} {} {}", nums[1], nums[2], nums[3]);


  // for num in range(0, 100) {
  //   do spawn {
  //     // let greeting_message = "Hello?";
  //     println!("Hello from {}", num);
  //   }
  // }

  // let (port, chan): (Port<int>, Chan<int>) = Chan::new();

  // do spawn {
  //   chan.send(10);
  // }

  // println(port.recv().to_str());
}
