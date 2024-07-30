// Keep in mind that Rust is a statically typed language, which means that it must know the types of all variables at compile time.

fn main() {
    
    /*

8-bit	i8	u8
16-bit	i16	u16
32-bit	i32	u32
64-bit	i64	u64
128-bit	i128	u128
arch	isize	usize


Number literals	Example
Decimal	98_222
Hex	0xff
Octal	0o77
Binary	0b1111_0000
Byte (u8 only)	b'A'

*/
      // addition
      let sum = 5 + 10;

      // subtraction
      let difference = 95.5 - 4.3;
  
      // multiplication
      let product = 4 * 30;
  
      // division
      let quotient = 56.7 / 32.2;
      let truncated = -5 / 3; // Results in -1
  
      // remainder
      let remainder = 43 % 5;

      let t = true;

      let f: bool = false; // with explicit type annotation



      // Compound Types 

      // Tuple : they cannot grow or shrink in size.
      let tup: (i32, f64, u8) = (500, 6.4, 1);
      let (x, y, z) = tup;
  
      println!("The value of y is: {y}");

      let x: (i32, f64, u8) = (500, 6.4, 1);

      let five_hundred = x.0;
  
      let six_point_four = x.1;
  
      let one = x.2;


      // Array
      let a: [i32; 5] = [1, 2, 3, 4, 5];
      let first = a[0];
      let second = a[1];

}

