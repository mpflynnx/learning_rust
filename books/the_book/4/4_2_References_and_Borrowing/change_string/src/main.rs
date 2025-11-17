fn main() {
    let mut s = String::from("Hello");

    change(&mut s);

    println!("{s}");
}

fn change(a_string: &mut String){
    a_string.push_str(", World!");
}
