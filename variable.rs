fn main() {
    // let mut x = 5;
    // println!("tHE VALUE OF X IS : {x}");
    // x = 6;
    // println!("tHE VALUE OF X IS : {x}");

    // // constant not allow to change value
    // const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3;
    // println!("{THREE_HOURS_IN_SECONDS}");

    // shawdowing
    let x = 5;

    let x = x + 1;

    {
        let x = x * 2;
        println!("The value of x in the inner scope is: {x}");
    }

    println!("The value of x is: {x}");
}
