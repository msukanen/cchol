fn main() {
    println!("Hello, world!");
}

fn unleet(w:&str) -> String {
    let mut w = w.to_lowercase();
    w = w.replace("1", "l");
    w = w.replace("3", "e");
    w = w.replace("5", "s");
    w = w.replace("7", "t");
    w = w.replace("\\/", "v");
    w
}