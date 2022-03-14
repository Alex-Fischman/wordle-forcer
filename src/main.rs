// brave flint pudgy shock
// brace moult shiny
// crony slate

fn main() {
    use std::io::BufRead;
    let words: Vec<_> = std::io::BufReader::new(std::fs::File::open("src/words.txt").unwrap())
        .lines()
        .map(Result::unwrap)
        .collect();
    let index = |c| "abcdefghijklmnopqrstuvwxyz".find(c).unwrap();
    let mut counts = [[0; 5]; 26];
    for word in &words {
        for (i, c) in word.chars().enumerate() {
            counts[index(c)][i] += 1;
        }
    }

    let words: Vec<String> = words
        .into_iter()
        .filter(|word| {
            let mut v: Vec<char> = word.chars().collect();
            v.sort_unstable();
            v.dedup();
            v.len() == 5
        })
        .collect();

    let chars: Vec<[char; 5]> = words
        .iter()
        .map(|word| word.chars().collect::<Vec<char>>().try_into().unwrap())
        .collect();

    let mut groups: Vec<usize> = words.iter().enumerate().map(|p| p.0).collect();

    let rank = |&a: &usize| {
        chars[a]
            .iter()
            .enumerate()
            .map(|(i, c)| counts[index(*c)][i])
            .sum::<usize>()
    };
    groups.sort_unstable_by_key(rank);
    use std::io::Write;
    let mut file = std::fs::File::create("out.txt").unwrap();
    for a in groups {
        let _ = writeln!(file, "{} {:?}", words[a], rank(&a));
    }
}
