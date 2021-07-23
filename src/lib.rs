pub trait Lev {
    type Other;
    fn lev(&self, other: &Self::Other) -> usize;
}

impl Lev for String {
    type Other = String;
    fn lev(&self, other: &Self::Other) -> usize {
        0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn compile() {
        let a = String::from("Hello");
        let b = String::from("world");
        println!("{}", a.lev(&b));
    }
}
