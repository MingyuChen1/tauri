// rb-e2e follow-up closed loop test file
fn main() {
    let x: Option<i32> = None;
    let y = x.unwrap_or(0);
    println!("{y}");
}
