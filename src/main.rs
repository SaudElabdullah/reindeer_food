use std::io;

fn main() {
    let mut num_of_entries = 5;
    let mut carry = 0;
    let mut highest_carry = 0;

    while num_of_entries != 0 {
        loop {
            let n: String = io::stdin().lines().next().unwrap().unwrap();
            if n == "" {
                num_of_entries -= 1;
                if carry > highest_carry {
                    highest_carry = carry;
                }
                carry = 0;
                break;
            }
            else {
                carry += n.parse::<i32>().unwrap();
            }
        }
    }

    println!("{}", highest_carry)
}
