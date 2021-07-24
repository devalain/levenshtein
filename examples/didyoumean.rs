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

    let mut i_min = words.len();
    let mut min_dist = usize::MAX;
    for (i, w) in words.iter().enumerate() {
        let dist = lev(w, &word);
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
