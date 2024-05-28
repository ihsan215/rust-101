
fn main() {

    // by default, variables are immutable so we define a mut for muttable variables
    let mut x = 5;
    println!("The value of x is: {x}");

    x = 6;
    println!("The values of x is: {x}");

    // Const 
    const THREE_HOURS_IN_SECODNS:u32 = 60*60*3;

    println!("The values of THREE_HOURS_IN_SECODNS is: {THREE_HOURS_IN_SECODNS}");

    // Shadowing : 
    // you can declare a new variable with the same name as a previous variable
    // Rustaceans say that the first variable is shadowed by the second

    let x = 5;

    let x = x + 1;

    {
        let x = x * 2;
        println!("The value of x in the inner scope is: {x}");
    }

    println!("The value of x is: {x}");

    /*

    Shadowing is different from marking a variable as mut because we’ll get a compile-time error if we accidentally try to reassign to this variable without using the let keyword

    The other difference between mut and shadowing is that because we’re effectively creating a new variable when we use the let keyword again, we can change the type of the value but reuse the same name.
     */

    // works
    let spaces = "     ";
    let spaces = spaces.len();
    println!("The space value is {spaces}");

        // not works
        // let mut spaces = "     ";
        // let spaces = spaces.len();
        // println!("The space value is {spaces}");

}
