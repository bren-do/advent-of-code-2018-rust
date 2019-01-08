use std::fs;

fn reactable(a: u8, b: u8) -> bool {
    if a > b {
        a - b == 32
    } else {
        b - a == 32
    }
}

fn react(letters: &mut Vec<u8>) -> (bool, &mut Vec<u8>) {
    let length = letters.len();
    for (i, &item) in letters.iter().enumerate() {
        if i + 1 < length && reactable(item, letters[i + 1]) {
            letters.remove(i);
            letters.remove(i);
            return (true, letters);
        }
    }
    (false, letters)
}

fn reaction_size(letters: &mut Vec<u8>) -> usize {
    let mut output_bytes = letters;
    let mut reacted = true;
    while reacted {
        let (r, o) = react(output_bytes);
        output_bytes = o;
        reacted = r;
    }
    output_bytes.to_vec().len()
}

fn reaction_for_letter(input: &str, reacting_letter: u8) -> usize {
    let mut output_bytes = input
        .trim()
        .as_bytes()
        .iter()
        .map(|&x| x) //y tho? why do we need to deref these?
        .filter(|&x| reacting_letter != x && reacting_letter + 32 != x)
        .collect::<Vec<u8>>();
    reaction_size(&mut output_bytes)
}

fn min_reaction_length(input: &str) -> usize {
    let reacting_letters = "ABCDEFGHIJKLMNOPQRSTUVWXYZ";
    reacting_letters
        .as_bytes()
        .iter()
        .map(|&letter| reaction_for_letter(input, letter))
        .min()
        .unwrap()
}

fn main() {
    // let test_input = "dabAcCaCBAcCcaDA";
    let real_input =
        fs::read_to_string("input.txt").expect("Something went wrong reading the file");

    let mut letters = real_input.trim().as_bytes().to_vec();
    let output_size = reaction_size(&mut letters);

    println!("Part1: {}", output_size);

    let min_output_length = min_reaction_length(&real_input);
    println!("Part2: {}", min_output_length);
}
