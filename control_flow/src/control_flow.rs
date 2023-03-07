// if Expressions
pub fn handling_conditions() {
    let number: u8 = 6;

    if number % 4 == 0 {
        println!("number is divisible by 4");
    } else if number % 3 == 0 {
        println!("number is divisible by 3");
    } else if number % 2 == 0 {
        println!("number is divisible by 2");
    } else {
        println!("number is not divisible by 4, 3, or 2");
    }
}

//if in a let Statement
pub fn handle_if_let_statement() {
    let condition: bool = true;
    let number: u8 = if condition { 5 } else { 6 };

    println!("The value of number is: {number}");
}

// Loops
pub fn handle_loops(max_counter: u8) -> u8 {
    let mut counter: u8 = 0;

    let result: u8 = loop {
        counter += 1;

        if counter == max_counter {
            break counter * 2;
        }
    };
    result
}

// Loop Labels to Disambiguate Between Multiple Loops
pub fn handle_multiple_loops() {
    let mut count = 0;
    'counting_up: loop {
        println!("count = {count}");
        let mut remaining = 10;

        loop {
            println!("remaining = {remaining}");
            if remaining == 9 {
                break;
            }
            if count == 2 {
                break 'counting_up;
            }
            remaining -= 1;
        }

        count += 1;
    }
    println!("End count = {count}");
}

// Loop while
pub fn handle_while() {
    let a: [u8; 5] = [10, 20, 30, 40, 50];
    let mut index = 0;

    while index < 5 {
        println!("the value is: {}", a[index]);

        index += 1;
    }
}

// Loop for
pub fn handle_loop_for() {
    for number in (1..4).rev() {
        println!("{number}!");
    }
    println!("LIFTOFF!!!");
}
