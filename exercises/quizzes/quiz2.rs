// This is a quiz for the following sections:
// - Strings
// - Vecs
// - Move semantics
// - Modules
// - Enums
//
// Let's build a little machine in the form of a function. As input, we're going
// to give a list of strings and commands. These commands determine what action
// is going to be applied to the string. It can either be:
// - Uppercase the string
// - Trim the string
// - Append "bar" to the string a specified amount of times
//
// The exact form of this will be:
// - The input is going to be a Vector of 2-length tuples,
//   the first element is the string, the second one is the command.
// - The output element is going to be a vector of strings.

enum Command {
    Uppercase,
    Trim,
    Append(usize),
}

// fn vec_map(input: &[i32]) -> Vec<i32> {
/*
fn count_for(map: &HashMap<String, Progress>, value: Progress) -> usize {
    let mut count = 0;
    for val in map.values() {
        if *val == value {
            count += 1;
        }
    }
    count
} */

mod my_module {
    use super::Command::Uppercase;
    use super::Command::Trim;
    use super::Command::Append;
    use super::Command;
    
    pub fn transformer(input: Vec<String>, com: Command) -> Vec<String>{
        let bara = String::from("Bar").to_string();
        let barass = "Bar";
        let baras = input.push(bara);
        match com{
            Uppercase => input.to_uppercase(),
            Trim => input.trim_start(),
            Append(usize) => baras,
        }
        /*let bara = String::from("Bar");
        //let bara = "Bar";
        //let mut v = vec![String::from("Hi")];
        //self.push(bara.to_string())
        self.push(bara); */
        /* 
        if com == Uppercase{
            let up = input.to_uppercase();
            up
        }
        else if com == "Trim"{

        }
        else if com == "Append"{

        }*/
    }
    // TODO: Complete the function as described above.
    // pub fn transformer(input: ???) -> ??? { ??? }
}

fn main() {
    // You can optionally experiment here.
}

#[cfg(test)]
mod tests {
    // TODO: What do we need to import to have `transformer` in scope?
    // use ???;
    use super::Command;
    use super::*;
    //use mod::my_module;
    

    #[test]
    fn it_works() {
        let input = vec![
            ("hello".to_string(), Command::Uppercase),
            (" all roads lead to rome! ".to_string(), Command::Trim),
            ("foo".to_string(), Command::Append(1)),
            ("bar".to_string(), Command::Append(5)),
        ];
        let output = transformer(input);

        assert_eq!(
            output,
            [
                "HELLO",
                "all roads lead to rome!",
                "foobar",
                "barbarbarbarbarbar",
            ]
        );
    }
}
