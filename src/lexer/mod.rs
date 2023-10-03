#![allow(unused)]

#[derive(Debug)]
pub struct TokenStream<'a> {
    tokens: Vec<Token<'a>>,
    idx:    usize,
}

impl<'a> TokenStream<'a> {
    pub fn new(tokens: Vec<Token<'a>>) -> Self {
        TokenStream { tokens, idx: 0 }
    }

    pub fn cursor(&self) -> Cursor {
        Cursor {
            tokens: &self.tokens,
            idx:    0,
        }
    }
}

#[derive(Debug)]
pub enum LexerError {
    UnexpectedToken(Span),
    UnknownToken(Span),
    UnexpectedEof(Span),
    UnmatchedDelimiter {
        start:    Span,
        end:      Span,
        expected: Delim,
    },
    UnknownChar(Span),
}

impl LexerError {
    pub fn span() -> Span {
        todo!()
    }
}

#[derive(Copy, Clone, Debug)]
pub struct Cursor<'a> {
    tokens: &'a [Token<'a>],
    idx:    usize,
}

impl<'a> Cursor<'a> {
    pub fn current(&self) -> Option<&'a Token> {
        self.tokens.get(self.idx)
    }

    pub fn next(&mut self) -> Option<&'a Token> {
        let token = self.tokens.get(self.idx);
        self.idx += 1;
        token
    }

    pub fn peek(&self) -> Option<&'a Token> {
        self.tokens.get(self.idx + 1)
    }

    pub fn peek_n(&self, n: usize) -> Option<&'a Token> {
        self.tokens.get(self.idx + n)
    }

    pub fn is_empty(&self) -> bool {
        self.idx >= self.tokens.len()
    }
}

#[derive(Debug)]
pub enum Token<'a> {
    Lit(Lit<'a>),
    Punct(Punct<'a>),
    Block(Block<'a>),
    Ident(Ident<'a>),
}

impl<'a> Token<'a> {
    pub fn span(&self) -> Span {
        match self {
            Token::Lit(lit) => lit.span(),
            Token::Punct(punct) => punct.span(),
            Token::Block(block) => block.span(),
            Token::Ident(ident) => ident.span(),
        }
    }

    pub fn is_lit(&self) -> bool {
        match self {
            Token::Lit(_) => true,
            _ => false,
        }
    }

    pub fn as_lit(&self) -> Option<&Lit<'a>> {
        match self {
            Token::Lit(lit) => Some(lit),
            _ => None,
        }
    }

    pub fn is_punct(&self) -> bool {
        match self {
            Token::Punct(_) => true,
            _ => false,
        }
    }

    pub fn as_punct(&self) -> Option<&Punct<'a>> {
        match self {
            Token::Punct(punct) => Some(punct),
            _ => None,
        }
    }

    pub fn is_block(&self) -> bool {
        match self {
            Token::Block(_) => true,
            _ => false,
        }
    }

    pub fn as_block(&self) -> Option<&Block<'a>> {
        match self {
            Token::Block(block) => Some(block),
            _ => None,
        }
    }

    pub fn is_ident(&self) -> bool {
        match self {
            Token::Ident(_) => true,
            _ => false,
        }
    }

    pub fn as_ident(&self) -> Option<&Ident<'a>> {
        match self {
            Token::Ident(ident) => Some(ident),
            _ => None,
        }
    }
}

#[derive(Copy, Clone, Debug)]
pub struct Span {
    // the offset of the character
    start: usize,
    // the offset of the last character included in the span
    end:   usize,
}

impl Span {
    pub fn new(start: usize, end: usize) -> Self {
        Span { start, end }
    }

    pub fn start(&self) -> usize {
        self.start
    }

    pub fn end(&self) -> usize {
        self.end
    }

    pub fn len(&self) -> usize {
        self.end - self.start
    }

    pub fn join(&self, other: Span) -> Span {
        Span::new(
            std::cmp::min(self.start, other.start),
            std::cmp::max(self.end, other.end),
        )
    }

    pub fn join_option(&self, other: Option<Span>) -> Span {
        match other {
            Some(other) => self.join(other),
            None => *self,
        }
    }
}

#[derive(Copy, Clone, Debug)]
pub struct Str<'a> {
    span:  Span,
    value: &'a str,
}

impl<'a> Str<'a> {
    pub fn new(span: Span, s: &'a str) -> Self {
        Str {
            span,
            value: &s[span.start..=span.end],
        }
    }

    pub fn value(&self) -> &'a str {
        self.value
    }
}

#[derive(Debug)]
pub struct Lit<'a> {
    span:   Span,
    kind:   LitKind,
    value:  Str<'a>,
    prefix: Option<Str<'a>>,
    suffix: Option<Str<'a>>,
}

impl<'a> Lit<'a> {
    pub fn new(
        span: Span,
        kind: LitKind,
        value: Str<'a>,
        prefix: Option<Str<'a>>,
        suffix: Option<Str<'a>>,
    ) -> Self {
        Lit {
            span,
            kind,
            value,
            prefix,
            suffix,
        }
    }

    pub fn span(&self) -> Span {
        self.span
    }

    pub fn kind(&self) -> LitKind {
        self.kind
    }

    pub fn prefix(&self) -> Option<Str<'a>> {
        self.prefix
    }

    pub fn suffix(&self) -> Option<Str<'a>> {
        self.suffix
    }
}

#[derive(Copy, Clone, Debug)]
pub enum LitKind {
    Str,
    Char,
    Int,
}

#[derive(Debug)]
pub struct Punct<'a> {
    value:   Str<'a>,
    spacing: Spacing,
}

impl<'a> Punct<'a> {
    pub fn new(value: Str<'a>, spacing: Spacing) -> Self {
        Punct { value, spacing }
    }

    pub fn span(&self) -> Span {
        self.value.span
    }

    pub fn value(&self) -> Str<'a> {
        self.value
    }

    pub fn spacing(&self) -> Spacing {
        self.spacing
    }
}

#[derive(Copy, Clone, Debug)]
pub enum Spacing {
    Alone,
    Together,
}

#[derive(Debug)]
pub struct Block<'a> {
    span:   Span,
    delim:  Delim,
    tokens: TokenStream<'a>,
}

impl<'a> Block<'a> {
    pub fn new(span: Span, delim: Delim, tokens: TokenStream<'a>) -> Self {
        Block {
            span,
            delim,
            tokens,
        }
    }

    pub fn span(&self) -> Span {
        self.span
    }

    pub fn delim(&self) -> Delim {
        self.delim
    }

    pub fn tokens(&self) -> &TokenStream {
        &self.tokens
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum Delim {
    Parenthesis,
    Bracket,
    Brace,
    Invis,
}

#[derive(Debug)]
pub struct Ident<'a> {
    span:  Span,
    value: Str<'a>,
}

impl<'a> Ident<'a> {
    pub fn new(span: Span, value: Str<'a>) -> Self {
        Ident { span, value }
    }

    pub fn span(&self) -> Span {
        self.span
    }

    pub fn value(&self) -> Str<'a> {
        self.value
    }
}

impl From<char> for Delim {
    fn from(c: char) -> Self {
        match c {
            '(' | ')' => Delim::Parenthesis,
            '[' | ']' => Delim::Bracket,
            '{' | '}' => Delim::Brace,
            _ => Delim::Invis,
        }
    }
}

impl From<&str> for Delim {
    fn from(s: &str) -> Self {
        match s {
            "(" | ")" => Delim::Parenthesis,
            "[" | "]" => Delim::Bracket,
            "{" | "}" => Delim::Brace,
            _ => Delim::Invis,
        }
    }
}

impl<'a> TokenStream<'a> {
    fn parse(s: &'a str) -> Result<TokenStream<'a>, LexerError> {
        let mut delim_stack = Vec::new();
        delim_stack.push((Delim::Invis, 0, Vec::new()));

        let mut iter = s.chars().enumerate().peekable();
        let skip = 0;
        let mut last = ' ';
        while let Some((i, c)) = iter.next() {
            let token = if Self::is_open_delim(c) {
                delim_stack.push((c.into(), i, Vec::new()));
                None
            } else if Self::is_close_delim(c) {
                let (delim, start, tokens) = delim_stack.pop().unwrap();
                let end_delim = c.into();
                if delim != end_delim {
                    return Err(LexerError::UnmatchedDelimiter {
                        start:    Span::new(start, start),
                        end:      Span::new(i, i),
                        expected: delim,
                    });
                }
                Some(Token::Block(Block::new(
                    Span::new(start, i),
                    delim,
                    TokenStream::new(tokens),
                )))
            } else if Self::is_punct(c) {
                Some(Token::Punct(Punct::new(
                    Str::new(Span::new(i, i), s),
                    if iter.peek().map(|c| Self::is_punct((c.1))).unwrap_or(false) {
                        Spacing::Together
                    } else {
                        Spacing::Alone
                    },
                )))
            } else if Self::is_whitespace(c) {
                None
            } else if Self::is_num(c)
                || Self::is_letter(c)
                || c == '_'
                || ((c == '"' || c == '\'') && last != '\\')
            {
                // starts with _ or letter
                // - parse either an ident or a prefix for anything
                let mut prefix_end = i;
                let prefix = if Self::is_letter(c) || c == '_' {
                    while iter
                        .peek()
                        .map(|c| Self::is_letter(c.1) || Self::is_num(c.1) || c.1 == '_')
                        .unwrap_or(false)
                    {
                        (prefix_end, last) = iter.next().unwrap();
                    }
                    let span = Span::new(i, prefix_end);
                    Some(Str::new(span, s))
                    // starts with a number
                    // - parse either an int or a prefix for anything
                } else if Self::is_num(c) {
                    while iter
                        .peek()
                        .map(|c| Self::is_num(c.1) || c.1 == '_')
                        .unwrap_or(false)
                    {
                        (prefix_end, last) = iter.next().unwrap();
                    }
                    let span = Span::new(i, prefix_end);
                    Some(Str::new(span, s))
                    // otherwise, it's a bare string or char literal
                } else {
                    None
                };
                if let Some(peek) = iter.peek() {
                    if peek.1 == '#' {
                        let (mut end, mut maybe_backslash) = iter.next().unwrap();
                        // raw string or raw ident
                        let mut hashes = 1;
                        while iter.peek().map(|c| c.1 == '#').unwrap_or(false) {
                            maybe_backslash = iter.next().unwrap().1;
                            hashes += 1;
                        }
                        if let Some(peek) = iter.peek() {
                            // raw ident
                            if Self::is_letter(peek.1) {
                                while iter
                                    .peek()
                                    .map(|c| {
                                        Self::is_letter(c.1) || Self::is_num(c.1) || c.1 == '_'
                                    })
                                    .unwrap_or(false)
                                {
                                    end = iter.next().unwrap().0;
                                }
                                let span = Span::new(i, end);
                                Some(Token::Lit(Lit::new(
                                    span,
                                    LitKind::Str,
                                    Str::new(span, s),
                                    None,
                                    None,
                                )))
                            } else if peek.1 == '"' && maybe_backslash != '\\' {
                                let mut last = iter.next().unwrap();
                                end = last.0;
                                let mut last = last.1;
                                // raw string
                                loop {
                                    let mut found_hashes = 0;
                                    if iter
                                        .peek()
                                        .map(|c| c.1 == '"' && last != '\\')
                                        .unwrap_or(false)
                                    {
                                        while iter.peek().map(|c| c.1 == '#').unwrap_or(false) {
                                            (end, last) = iter.next().unwrap();
                                            found_hashes += 1;
                                        }
                                        if found_hashes == hashes {
                                            break;
                                        }
                                        found_hashes = 0;
                                    }
                                    end = iter.next().unwrap().0;
                                }
                                let span = Span::new(i, end);
                                let suffix = if iter
                                    .peek()
                                    .map(|c| {
                                        Self::is_letter(c.1) || Self::is_num(c.1) || c.1 == '_'
                                    })
                                    .unwrap_or(false)
                                {
                                    let start = iter.next().unwrap().0;
                                    let mut end = start;
                                    while iter
                                        .peek()
                                        .map(|c| {
                                            Self::is_letter(c.1) || Self::is_num(c.1) || c.1 == '_'
                                        })
                                        .unwrap_or(false)
                                    {
                                        end = iter.next().unwrap().0;
                                    }
                                    Some(Str::new(Span::new(start, end), s))
                                } else {
                                    None
                                };
                                Some(Token::Lit(Lit::new(
                                    span,
                                    LitKind::Str,
                                    Str::new(span, s),
                                    prefix,
                                    suffix,
                                )))
                            } else {
                                return Err(LexerError::UnknownToken(Span::new(i, peek.0)));
                            }
                        } else {
                            return Err(LexerError::UnexpectedEof(Span::new(s.len(), s.len())));
                        }
                    } else if (peek.1 == '"' || peek.1 == '\'') && last != '\\' {
                        // string or char with a prefix and maybe a suffix
                        let (start, ch) = iter.next().unwrap();
                        let mut end = start;
                        while iter.peek().map(|c| c.1 != ch).unwrap_or(false) {
                            end = iter.next().unwrap().0;
                        }
                        let span = Span::new(start, end);
                        let suffix = if iter
                            .peek()
                            .map(|c| Self::is_letter(c.1) || Self::is_num(c.1) || c.1 == '_')
                            .unwrap_or(false)
                        {
                            let start = iter.next().unwrap().0;
                            let mut end = start;
                            while iter
                                .peek()
                                .map(|c| Self::is_letter(c.1) || Self::is_num(c.1) || c.1 == '_')
                                .unwrap_or(false)
                            {
                                end = iter.next().unwrap().0;
                            }
                            Some(Str::new(Span::new(start, end), s))
                        } else {
                            None
                        };

                        Some(Token::Lit(Lit::new(
                            span,
                            if ch == '"' {
                                LitKind::Str
                            } else {
                                LitKind::Char
                            },
                            Str::new(span, s),
                            prefix,
                            suffix,
                        )))
                    } else if Self::is_letter(peek.1) {
                        // this is now the suffix for an int
                        let start = iter.next().unwrap().0;
                        let mut end = start;
                        while iter
                            .peek()
                            .map(|c| Self::is_letter(c.1) || Self::is_num(c.1) || c.1 == '_')
                            .unwrap_or(false)
                        {
                            end = iter.next().unwrap().0;
                        }
                        let suffix_span = Span::new(start, end);
                        let suffix = Str::new(suffix_span, s);
                        Some(Token::Lit(Lit::new(
                            prefix.unwrap().span,
                            LitKind::Int,
                            prefix.unwrap(),
                            None,
                            Some(suffix),
                        )))
                    } else {
                        // in this case, it's either an ident or an int just with another token
                        // next
                        let span = Span::new(i, prefix_end);
                        if Self::is_num(c) {
                            Some(Token::Lit(Lit::new(
                                span,
                                LitKind::Int,
                                Str::new(span, s),
                                None,
                                None,
                            )))
                        } else {
                            Some(Token::Ident(Ident::new(span, Str::new(span, s))))
                        }
                    }
                } else {
                    // in this case, it's just a bare ident or int at the very end of the input
                    let span = Span::new(i, prefix_end);
                    if Self::is_num(c) {
                        Some(Token::Lit(Lit::new(
                            span,
                            LitKind::Int,
                            Str::new(span, s),
                            None,
                            None,
                        )))
                    } else {
                        Some(Token::Ident(Ident::new(span, Str::new(span, s))))
                    }
                }
            } else {
                return Err(LexerError::UnknownChar(Span::new(i, i)));
            };
            last = c;
            match token {
                Some(t) => delim_stack.last_mut().unwrap().2.push(t),
                None => continue,
            }
        }
        if delim_stack.len() > 1 {
            let (delim, start, tokens) = delim_stack.pop().unwrap();
            return Err(LexerError::UnmatchedDelimiter {
                start:    Span::new(start, start),
                end:      Span::new(s.len(), s.len()),
                expected: delim,
            });
        } else {
            let (_, _, tokens) = delim_stack.pop().unwrap();
            return Ok(TokenStream::new(tokens));
        }
    }

    fn is_punct(c: char) -> bool {
        match c {
            '&' | '=' | '@' | '^' | ':' | ',' | '$' | '.' | '>' | '<' | '-' | '+' | '!' | '|'
            | '%' | '#' | '?' | ';' | '/' | '*' | '~' | '`' | '\\' => true,
            _ => false,
        }
    }

    fn is_whitespace(c: char) -> bool {
        match c {
            ' ' | '\t' | '\r' | '\n' => true,
            _ => false,
        }
    }

    fn is_open_delim(c: char) -> bool {
        match c {
            '(' | '[' | '{' => true,
            _ => false,
        }
    }

    fn is_close_delim(c: char) -> bool {
        match c {
            ')' | ']' | '}' => true,
            _ => false,
        }
    }

    fn is_letter(c: char) -> bool {
        match c {
            'a'..='z' | 'A'..='Z' => true,
            _ => false,
        }
    }

    fn is_num(c: char) -> bool {
        match c {
            '0'..='9' => true,
            _ => false,
        }
    }
}
