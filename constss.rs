const fn num() -> u32 {
    10
}

const RESULT_1: u32 = num();

fn main() {
    println!("constant RESULT_1 --> {RESULT_1}");
    
    let four: u32 = "4".parse().unwrap();

    assert_eq!(4, four);
    assert_eq!(5, four);

}
