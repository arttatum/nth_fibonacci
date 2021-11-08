fn main() {
    let nth_number = get_nth_fibonacci_number(186);
    println!("{}", nth_number);
}

// This method supports {0 < n <= 186} 
fn get_nth_fibonacci_number(n: u16) -> u128 {
    if n == 1 || n == 2 {
        return 1
    }

    // Why (n-2)? 
    // If we are calculating the 3rd fib number, given the first two elements, 
    //  we only need to do 1 addition (1 + 1 -> 2)... 
    // If we are calculating the 4th, given the first two elements, 
    //  we only need to do 2 additions (1 + 1 -> 2, then 2 + 1 -> 3)
    // In other words, we're beginning the algorithm from the 2nd fibonacci number,
    //  we continue to add the current and previous elements n - 2 times to find the nth fibonacci number.
    get_nth_fibonacci_number_recursive(n - 2, 1, 1)
}

fn get_nth_fibonacci_number_recursive(  additions_remaining: u16, 
                                        previous_value: u128, 
                                        current_value: u128) -> u128 {
    
    if additions_remaining == 1 {
        return current_value + previous_value;
    }

    get_nth_fibonacci_number_recursive( additions_remaining - 1, 
                                        current_value, 
                                        previous_value + current_value)
}