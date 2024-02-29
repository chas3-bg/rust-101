fn main() {
    println!("Hello, collection::String!");

    //create new String
    let mut s1 = String::new();
    let data = "initial contents";
    s1 = data.to_string();
    println!("s1 = {}", s1);

    //or
    let s2: String = String::from("initial contents");

    let mut s3 = String::from("foo");
    s3.push_str("bar");

    //indexing of strings is no-no, instead slice.
    //each UTF- char is 2 bytes, so the first slice must be [[0..2]]
    let s4 = String::from("Здравей");
    let s4_s = &s4[0..4];
    println!("s4_s = {}", s4_s);

    for c in s4.chars() {
        println!("{}", c);
    }
}
