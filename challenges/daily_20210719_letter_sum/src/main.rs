static n: u8 = ('a' as u8) - 1;

fn main() {
    println!("{}", letter_sum(""));
    println!("{}", letter_sum("a"));
    println!("{}", letter_sum("z"));
    println!("{}", letter_sum("cab"));
    println!("{}", letter_sum("excellent"));
    println!("{}", letter_sum("microspectrophotometries"));
}

fn letter_sum(input: &str) -> u32{
    input.as_bytes().iter().map(|u| (u - n) as u32).sum::<u32>()
}
