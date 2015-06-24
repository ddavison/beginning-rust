fn add(a: u32, b: u32) -> u32 {return a+b;}
fn subtract(a: u32, b: u32) -> u32 {return a-b;}
fn multiply(a: u32, b: u32) -> u32 {return a*b;}
fn divide(a: u32, b: u32) -> u32 {return a/b;}

#[test]
fn test_adds() {
    assert_eq!(add(1, 2), 3);
}

#[test]
fn test_subtracts() {
    assert_eq!(subtract(3, 1), 2);
}
