fn main() {
    let x = 5;

    let x = x + 2;

    let mut y = 8;

    y = 9;

    let spaces = " ";

    let spaces = spaces.len();

    {
        let x = x * 2;
            println!("The value of x in the inner scope is: {x}");
    }

    println!("The value of x is: {x}");
    println!("The value of y is {y}");
    println!("The amount of spaces is: {spaces}");
    println!("Nothing more to see here.");
}