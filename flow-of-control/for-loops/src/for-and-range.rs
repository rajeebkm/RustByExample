//---for and range---//
// The for in construct can be used to iterate through an Iterator. One of the easiest ways to create an iterator is to use the range notation a..b. This yields values from a (inclusive) to b (exclusive) in steps of one.

// Let's write FizzBuzz using for instead of while.
fn main() {
    // `n` will take the values: 1, 2, ..., 100 in each iteration
    for n in 1..101 {
        if n % 15 == 0 {
            println!("fizzbuzz");
        } else if n % 3 == 0 {
            println!("fizz");
        } else if n % 5 == 0 {
            println!("buzz");
        } else {
            println!("{}", n);
        }
    }

    println!("//-----------------------------------------------------------------------------------------//");

// Alternatively, a..=b can be used for a range that is inclusive on both ends. The above can be written as:

// `n` will take the values: 1, 2, ..., 100 in each iteration
    for n in 1..=100 {
        if n % 15 == 0 {
            println!("fizzbuzz");
        } else if n % 3 == 0 {
            println!("fizz");
        } else if n % 5 == 0 {
            println!("buzz");
        } else {
            println!("{}", n);
        }
    }
}



