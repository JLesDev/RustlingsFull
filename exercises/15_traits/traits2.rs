trait AppendBar {
    fn append_bar(self) -> Self;
}

// TODO: Implement the trait `AppendBar` for a vector of strings.
// `append_bar` should push the string "Bar" into the vector.

impl AppendBar for Vec<String> {
    
    fn append_bar(self)-> Vec<String>{
        let bara = String::from("Bar");
        //let append_bara = new Vec::<String>;
        let v = vec![""];
        v.push(&bara.to_string())
        
        
    }


}

fn main() {
    // You can optionally experiment here.
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn is_vec_pop_eq_bar() {
        let mut foo = vec![String::from("Foo")].append_bar();
        assert_eq!(foo.pop().unwrap(), "Bar");
        assert_eq!(foo.pop().unwrap(), "Foo");
    }
}
