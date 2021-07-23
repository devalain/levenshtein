use levenshtein::lev;

#[derive(Debug)]
struct ArgsError(&'static str);
impl std::fmt::Display for ArgsError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "ArgsError")
    }
}
impl std::error::Error for ArgsError {}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut args = std::env::args();
    let _ = args.next();
    let word = args.next().ok_or_else(|| ArgsError("Please provide a word"))?;
    let dico_file = args.next().ok_or_else(|| ArgsError("Please provide a dictionnary file name"))?;

    use std::fs::File;
    use std::io::prelude::*;
    let mut file = File::open(dico_file).expect("Failed to open dico file");
    let mut contents = String::new();
    file.read_to_string(&mut contents).expect("Failed to read dico file");

    let contents = contents.replace("\r", "");
    let words = contents.split("\n").collect::<Vec<&str>>();
    let longest = words.iter().map(|s| str::len(*s)).max();
    let lev_dist = match longest {
        None|Some(0..=35) => |a: &str, b: &str| lev::<35>(a,b),
        Some(36..=50) => |a: &str, b: &str| lev::<50>(a,b),
        Some(51..=100) => |a: &str, b: &str| lev::<100>(a,b),
        Some(101..=150) => |a: &str, b: &str| lev::<150>(a,b),
        Some(151..=200) => |a: &str, b: &str| lev::<200>(a,b),
        Some(201..=250) => |a: &str, b: &str| lev::<250>(a,b),
        Some(251..=300) => |a: &str, b: &str| lev::<300>(a,b),
        _ => |a: &str, b: &str| lev::<350>(a,b)
    };

    let mut i_min = words.len();
    let mut min_dist = usize::MAX;
    for (i, w) in words.iter().enumerate() {
        let dist = lev_dist(w, &word);
        if dist < min_dist {
            min_dist = dist;
            i_min = i;
        }
    }
    if min_dist == 0 {
        println!("The word is correctly spelled.");
    } else {
        println!("Did you mean '{}' ?", words[i_min]);
    }

    Ok(())
}
