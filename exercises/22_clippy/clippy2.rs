fn main() {
    let mut res = 42;
    let option = Some(12);
    // TODO: Fix the Clippy lint.

    if let x < option{
        res += x;
        x += 1;
    }
    /*for x in option {
        res += x;
    }*/

    println!("{res}");
}
