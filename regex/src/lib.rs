use regex_macros::regex;
regex! {Test => "ab + c*"}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        // TODO: this doesn't work because the DFA only has a single accept state, need
        // multiple accept states to handle multiple possible endpoints
        panic!("{}", Test::matches("ccc"));
    }
}
