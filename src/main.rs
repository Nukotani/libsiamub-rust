use curl::easy::Easy;

const USER_AGENT: &str = "Mozilla/5.0 (X11; Linux x86_64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/120.0.0.0 Safari/537.36";

fn main() {
    let mut easy = Easy::new();

    easy.useragent(USER_AGENT).unwrap();
}
