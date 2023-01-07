use std::fmt::{Display, Formatter};
use crate::source::span::Span;
use crate::tokens::{Literal, NumLit};

#[derive(Debug)]
pub(crate) struct ParseError {
    et: ParseET,
    loc: Option<Span>,
    context: Vec<String>
}

impl ParseError {
    pub(crate) fn when<T: Into<String>>(mut self, reason: T) -> Self{
        self.context.push(reason.into());
        self
    }
    pub(crate) fn at(mut self, loc: Span) -> Self{
        self.loc = Some(loc);
        self
    }
}

impl From<std::io::Error> for ParseError {
    fn from(error: std::io::Error) -> Self {
        ParseET::IOError(error).error().when("doing IO operation")
    }
}

#[derive(Debug)]
pub(crate) enum ParseET {
    EOF,
    EmptyInput,
    IOError(std::io::Error),
    TokenizationError(String),
    LiteralError(Literal, String),
    ParsingError(String),
    CompilationError(String),
    AlreadyDefinedError(String, String, Span),
    VariableNotFound(String)
}

impl ParseET {
    pub(crate) fn error(self) -> ParseError {
        ParseError {
            et: self,
            loc: None,
            context: vec![]
        }
    }
    pub(crate) fn at(self, loc: Span) -> ParseError {
        ParseError {
            et: self,
            loc: Some(loc),
            context: vec![]
        }
    }
}

impl Display for ParseError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}{}{}{}",
               match &self.et {
                   ParseET::EOF => format!("Input error:\n    reached end of file"),
                   ParseET::EmptyInput => format!("Input error:\n    input was empty"),
                   ParseET::IOError(e) => format!("IO error:\n    {}", e),
                   ParseET::TokenizationError(e) => format!("Tokenization error:\n    {}", e),
                   ParseET::LiteralError(lit, e) => format!("{} literal error:\n    {}", match lit {
                       Literal::String(_) => "String",
                       Literal::Char(_) => "Char",
                       Literal::Number(NumLit::Integer(_), _) => "Integer",
                       Literal::Number(NumLit::Float(_), _) => "Float",
                       Literal::Bool(_) => "Float",
                       Literal::Array(..) => "Array"
                   }, e),
                   ParseET::ParsingError(e) => format!("Parsing error:\n    {}", e),
                   ParseET::CompilationError(e) => format!("Compilation error:\n    {}", e),
                   ParseET::AlreadyDefinedError(what, name, loc) =>
                       format!("Multiple definitions error:\n    {} {} was already defined",
                       what, name),
                   ParseET::VariableNotFound(ident) => format!("Name error:\n    could not find variable {ident}")
               },
               if self.context.len() > 0 {
                   format!("\n    while {}", self.context.join("\n    while "))
               } else {
                   String::new()
               },
               match &self.et {
                   ParseET::AlreadyDefinedError(what, name, loc) =>
                       format!("\n\n{:?}: {:?}\n{}", loc.source, loc, loc.render_span_code(2)),
                    _ => String::new()
               },
               if let Some(loc) = &self.loc {
                   format!("\n\n{:?}: {:?}\n{}",
                       loc.source,
                       loc,
                       loc.render_span_code(2)
                   )
               } else {
                   String::new()
               },
        )
    }
}

pub(crate) trait OnParseErr{
    fn e_when<S: Into<String>>(self, reason: S) -> Self;
    fn e_at(self, loc: Span) -> Self;
}

impl<T> OnParseErr for Result<T, ParseError> {
    fn e_when<S: Into<String>>(self, reason: S) -> Self {
        self.map_err(|err| err.when(reason))
    }

    fn e_at(self, loc: Span) -> Self {
        self.map_err(|err| err.at(loc))
    }
}