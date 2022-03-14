// brave flint pudgy shock

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

    let mut set_j = std::collections::HashSet::with_capacity(10);
    let mut set_k = std::collections::HashSet::with_capacity(15);
    let mut set_l = std::collections::HashSet::with_capacity(20);
    let mut groups = vec![];
    for i in 0..words.len() {
        println!("{} ", words[i]);
        for j in (i + 1)..words.len() {
            set_j.clear();
            set_j.extend(chars[i]);
            set_j.extend(chars[j]);
            if set_j.len() == 10 {
                for k in (j + 1)..words.len() {
                    set_k.clear();
                    set_k.extend(chars[i]);
                    set_k.extend(chars[j]);
                    set_k.extend(chars[k]);
                    if set_k.len() == 15 {
                        for l in (k + 1)..words.len() {
                            set_l.clear();
                            set_l.extend(chars[i]);
                            set_l.extend(chars[j]);
                            set_l.extend(chars[k]);
                            set_l.extend(chars[l]);
                            if set_l.len() == 20 {
                                groups.push((i, j, k, l));
                            }
                        }
                    }
                }
            }
        }
    }

    let rank = |&(a, b, c, d): &(usize, usize, usize, usize)| {
        chars[a]
            .iter()
            .enumerate()
            .chain(chars[b].iter().enumerate())
            .chain(chars[c].iter().enumerate())
            .chain(chars[d].iter().enumerate())
            .map(|(i, c)| counts[index(*c)][i])
            .sum::<usize>()
    };
    groups.sort_unstable_by_key(rank);
    use std::io::Write;
    let mut file = std::fs::File::create("out.txt").unwrap();
    for (a, b, c, d) in groups {
        let _ = writeln!(
            file,
            "{} {} {} {} {:?}",
            words[a],
            words[b],
            words[c],
            words[d],
            rank(&(a, b, c, d))
        );
    }
}
