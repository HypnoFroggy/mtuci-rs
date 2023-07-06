

const SEARCH_TERM: &str = "picture";
const QUOTE: &str = "Every face, every shop, bedroom window, public-house, and
dark square is a picture feverishly turned--in search of what?
It is the same with books. What do we seek through millions of pages?";


fn main() {
    find_term(SEARCH_TERM, QUOTE);
}

fn find_term(search_term: &str, quote: &str) -> String {
    let mut result = String::new();
    let bytes = quote.as_bytes();
    let mut words: Vec<usize> = vec![0];
    let mut lines: Vec<usize> = vec![0];
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' || item == b'.' || item == b',' || item == b'?' || item == b'!' || item == b':' || item == b';' || item == b'"'{
            words.push(i);
        }
        if item == b'\n' {
            lines.push(i);
        }
    }
    lines.push(quote.len());
    let mut x: usize = 0;
    let mut y: usize = 0;
    let mut a: usize = 0;
    let mut b: usize = 0;
    let mut breakpoint: usize = 0;
    let mut line = 0;
    let mut i: usize = 0;
    while i < (words.len() - 1){
        a = if i == 0 {words[i]} else {words[i]+1};
        b = words[i+1];
        if &quote[a..b] == search_term {
            breakpoint = a;
        }
        i += 1;
    }
    i = 0;
    while i < (lines.len() - 1){
        a = if i == 0 {lines[i]} else {lines[i]+1};
        b = lines[i+1];
        if breakpoint < lines[i+1] && breakpoint >= lines[i]{
            line = i + 1;
            x = a;
            y = b;
        }
        i += 1;
    }
    if i == lines.len() - 1 && line == 0{
        breakpoint = lines.len();
        line = i;
        x = a;
        y = b;
    }
    result = line.to_string() + ": " + &quote[x..y];
    result
}


// ----> TESTS
#[cfg(test)]
mod tests {
    use crate::find_term;
    use crate::{SEARCH_TERM, QUOTE};

    #[test]
    fn correct_line() {
        let answer = find_term(SEARCH_TERM, QUOTE);

        assert_eq!("2: dark square is a picture feverishly turned--in search of what?", answer)
    }
}
