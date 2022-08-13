fn main() {

    // scalar types: integers, floating-point numbers, boolean and chars
    // signed can store numbers from -(2^n-1) to 2^(n-1)-1 inclusive
    // unsigned can sotre from 0 to 2^(n-1)
    
    let decimal = 98_222u32;
    let hex: u16 = 0xABDC;
    let octal: u16 = 0o77;
    let binary_number: u16 = 0b1111_1101;
    let byte2 = b'A';

    println!("{byte2}");    

    let tup = (500, 4.3, 1.0);
    let (x, y, z) = tup;

    println!("{x}, {y}, {z}");
    
}

