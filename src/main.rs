use mylib;

fn main() {
    println!("Hello ");

    let thing = mylib::Thing {
        obj: String::from("world!"),
    };

    thing.do_the_thing();
}
