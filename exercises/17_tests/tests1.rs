// Tests are important to ensure that your code does what you think it should
// do.

fn is_even(n: i64) -> bool {
    n % 2 == 0
}

fn main() {
    // You can optionally experiment here.
}

#[cfg(test)]
mod tests {
    // TODO: Import `is_even`. You can use a wildcard to import everything in
    // the outer module.
    
    //const is_even = Wildcard::new(is_even).unwrap();
    use is_even::*;

    #[test]
    fn you_can_assert() {
        // TODO: Test the function `is_even` with some values.
        assert!(200);
        //assert!(is_even(155),"False");
    }
}/* 
use super::*;

#[test]
fn compare_license_information() {
    assert!(compare_license_types(SomeSoftware, OtherSoftware));
}
*/
