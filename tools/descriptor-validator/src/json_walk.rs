use std::fmt;

#[derive(Debug, Clone, PartialEq)]
pub enum JsonValue {
    Null,
    Bool(bool),
    Number(String),
    String(String),
    Array(Vec<JsonValue>),
    Object(Vec<(String, JsonValue)>),
}

impl JsonValue {
    pub fn as_str(&self) -> Option<&str> {
        match self {
            Self::String(value) => Some(value),
            _ => None,
        }
    }

    pub fn as_bool(&self) -> Option<bool> {
        match self {
            Self::Bool(value) => Some(*value),
            _ => None,
        }
    }

    pub fn get(&self, key: &str) -> Option<&JsonValue> {
        match self {
            Self::Object(fields) => fields
                .iter()
                .find_map(|(field_key, value)| (field_key == key).then_some(value)),
            _ => None,
        }
    }

    pub fn fields(&self) -> Option<&[(String, JsonValue)]> {
        match self {
            Self::Object(fields) => Some(fields),
            _ => None,
        }
    }

    pub fn is_number_zero(&self) -> bool {
        match self {
            Self::Number(raw) => raw
                .parse::<f64>()
                .map(|value| value == 0.0)
                .unwrap_or(false),
            _ => false,
        }
    }

    pub fn is_string_or_number_zero(&self) -> bool {
        match self {
            Self::Number(_) => self.is_number_zero(),
            Self::String(raw) => raw
                .parse::<f64>()
                .map(|value| value == 0.0)
                .unwrap_or(false),
            _ => false,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct JsonParseError {
    pub message: String,
    pub offset: usize,
}

impl JsonParseError {
    fn new(message: impl Into<String>, offset: usize) -> Self {
        Self {
            message: message.into(),
            offset,
        }
    }
}

impl fmt::Display for JsonParseError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} at byte {}", self.message, self.offset)
    }
}

pub fn parse_json(input: &str) -> Result<JsonValue, JsonParseError> {
    let input = input.strip_prefix('\u{feff}').unwrap_or(input);
    let mut parser = Parser::new(input);
    let value = parser.parse_value()?;
    parser.skip_ws();
    if !parser.is_eof() {
        return Err(JsonParseError::new("unexpected trailing input", parser.pos));
    }
    Ok(value)
}

pub fn is_json_like_string(input: &str) -> bool {
    let trimmed = input.trim_start();
    trimmed.starts_with('{') || trimmed.starts_with('[')
}

struct Parser<'a> {
    input: &'a str,
    pos: usize,
}

impl<'a> Parser<'a> {
    fn new(input: &'a str) -> Self {
        Self { input, pos: 0 }
    }

    fn is_eof(&self) -> bool {
        self.pos >= self.input.len()
    }

    fn peek_char(&self) -> Option<char> {
        self.input[self.pos..].chars().next()
    }

    fn bump_char(&mut self) -> Option<char> {
        let ch = self.peek_char()?;
        self.pos += ch.len_utf8();
        Some(ch)
    }

    fn skip_ws(&mut self) {
        while matches!(self.peek_char(), Some(' ' | '\n' | '\r' | '\t')) {
            self.bump_char();
        }
    }

    fn parse_value(&mut self) -> Result<JsonValue, JsonParseError> {
        self.skip_ws();
        match self.peek_char() {
            Some('"') => self.parse_string().map(JsonValue::String),
            Some('{') => self.parse_object(),
            Some('[') => self.parse_array(),
            Some('t') => self.parse_literal("true", JsonValue::Bool(true)),
            Some('f') => self.parse_literal("false", JsonValue::Bool(false)),
            Some('n') => self.parse_literal("null", JsonValue::Null),
            Some('-' | '0'..='9') => self.parse_number().map(JsonValue::Number),
            Some(_) => Err(JsonParseError::new("unexpected character", self.pos)),
            None => Err(JsonParseError::new("unexpected end of input", self.pos)),
        }
    }

    fn parse_literal(
        &mut self,
        literal: &str,
        value: JsonValue,
    ) -> Result<JsonValue, JsonParseError> {
        if self.input[self.pos..].starts_with(literal) {
            self.pos += literal.len();
            Ok(value)
        } else {
            Err(JsonParseError::new("invalid literal", self.pos))
        }
    }

    fn parse_object(&mut self) -> Result<JsonValue, JsonParseError> {
        self.expect_char('{')?;
        let mut fields = Vec::new();
        loop {
            self.skip_ws();
            if self.peek_char() == Some('}') {
                self.bump_char();
                break;
            }

            let key = self.parse_string()?;
            self.skip_ws();
            self.expect_char(':')?;
            let value = self.parse_value()?;
            fields.push((key, value));

            self.skip_ws();
            match self.peek_char() {
                Some(',') => {
                    self.bump_char();
                }
                Some('}') => {
                    self.bump_char();
                    break;
                }
                _ => return Err(JsonParseError::new("expected object separator", self.pos)),
            }
        }
        Ok(JsonValue::Object(fields))
    }

    fn parse_array(&mut self) -> Result<JsonValue, JsonParseError> {
        self.expect_char('[')?;
        let mut values = Vec::new();
        loop {
            self.skip_ws();
            if self.peek_char() == Some(']') {
                self.bump_char();
                break;
            }

            values.push(self.parse_value()?);
            self.skip_ws();
            match self.peek_char() {
                Some(',') => {
                    self.bump_char();
                }
                Some(']') => {
                    self.bump_char();
                    break;
                }
                _ => return Err(JsonParseError::new("expected array separator", self.pos)),
            }
        }
        Ok(JsonValue::Array(values))
    }

    fn parse_string(&mut self) -> Result<String, JsonParseError> {
        self.expect_char('"')?;
        let mut value = String::new();
        loop {
            match self.bump_char() {
                Some('"') => break,
                Some('\\') => value.push(self.parse_escape()?),
                Some(ch) if ch.is_control() => {
                    return Err(JsonParseError::new("control character in string", self.pos));
                }
                Some(ch) => value.push(ch),
                None => return Err(JsonParseError::new("unterminated string", self.pos)),
            }
        }
        Ok(value)
    }

    fn parse_escape(&mut self) -> Result<char, JsonParseError> {
        match self.bump_char() {
            Some('"') => Ok('"'),
            Some('\\') => Ok('\\'),
            Some('/') => Ok('/'),
            Some('b') => Ok('\u{0008}'),
            Some('f') => Ok('\u{000c}'),
            Some('n') => Ok('\n'),
            Some('r') => Ok('\r'),
            Some('t') => Ok('\t'),
            Some('u') => self.parse_unicode_escape(),
            _ => Err(JsonParseError::new("invalid escape", self.pos)),
        }
    }

    fn parse_unicode_escape(&mut self) -> Result<char, JsonParseError> {
        let start = self.pos;
        let end = start + 4;
        if end > self.input.len() {
            return Err(JsonParseError::new("short unicode escape", self.pos));
        }
        let raw = &self.input[start..end];
        if !raw.chars().all(|ch| ch.is_ascii_hexdigit()) {
            return Err(JsonParseError::new("invalid unicode escape", self.pos));
        }
        self.pos = end;
        let code = u32::from_str_radix(raw, 16)
            .map_err(|_| JsonParseError::new("invalid unicode escape", start))?;
        char::from_u32(code).ok_or_else(|| JsonParseError::new("invalid unicode scalar", start))
    }

    fn parse_number(&mut self) -> Result<String, JsonParseError> {
        let start = self.pos;
        if self.peek_char() == Some('-') {
            self.bump_char();
        }
        self.consume_digits();
        if self.peek_char() == Some('.') {
            self.bump_char();
            self.consume_digits();
        }
        if matches!(self.peek_char(), Some('e' | 'E')) {
            self.bump_char();
            if matches!(self.peek_char(), Some('+' | '-')) {
                self.bump_char();
            }
            self.consume_digits();
        }
        if self.pos == start || self.input[start..self.pos].parse::<f64>().is_err() {
            return Err(JsonParseError::new("invalid number", start));
        }
        Ok(self.input[start..self.pos].to_string())
    }

    fn consume_digits(&mut self) {
        while matches!(self.peek_char(), Some('0'..='9')) {
            self.bump_char();
        }
    }

    fn expect_char(&mut self, expected: char) -> Result<(), JsonParseError> {
        match self.bump_char() {
            Some(ch) if ch == expected => Ok(()),
            _ => Err(JsonParseError::new(
                format!("expected '{}'", expected),
                self.pos,
            )),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_nested_json() {
        let value = parse_json(r#"{"a":[1,true,{"b":"c"}]}"#).unwrap();
        let JsonValue::Object(fields) = value else {
            panic!("expected object");
        };
        assert_eq!(fields[0].0, "a");
    }

    #[test]
    fn rejects_trailing_input() {
        assert!(parse_json(r#"{"a":1} nope"#).is_err());
    }

    #[test]
    fn accepts_utf8_bom() {
        assert!(parse_json("\u{feff}{\"a\":1}").is_ok());
    }
}
