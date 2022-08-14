fn main() {
    let haystack = String::from("hello");
    let needle = String::from("ll");

    let ind = str_str(haystack, needle);
    println!("{ind}");
    
    let ind = str_str("hello".to_string(), "ll".to_string());
    println!("{ind}");

    assert_eq!(str_str("must".to_string(), "st".to_string()), 2);
    
    assert_eq!(str_str("muts".to_string(), "st".to_string()), -1);

    assert_eq!(str_str("hhiasf".to_string(), "".to_string()), 0);
    
    
}

fn str_str(haystack: String, needle: String) -> i32 {
    if needle.len() == 0 { return 0i32; }
    let len = needle.len();
    
    for subindex in 0..haystack.len() - len + 1 {
        if haystack[subindex..subindex+len] == needle { return subindex as i32; }
    }
    
    -1
}
