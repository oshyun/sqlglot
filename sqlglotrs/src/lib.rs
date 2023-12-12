use pyo3::prelude::*;
use std::hash::{Hash, Hasher};

mod settings;
mod token_type;
mod tokenizer;
mod trie;

pub use self::settings::TokenizerSettings;
pub use self::token_type::TokenType;
pub use self::tokenizer::Tokenizer;

#[derive(Debug)]
#[pyclass]
pub struct Token {
    #[pyo3(get)]
    pub token_type: TokenType,
    #[pyo3(get)]
    pub text: String,
    #[pyo3(get)]
    pub line: usize,
    #[pyo3(get)]
    pub col: usize,
    #[pyo3(get)]
    pub start: usize,
    #[pyo3(get)]
    pub end: usize,
    #[pyo3(get)]
    pub comments: Vec<String>,
}

impl Token {
    pub fn new(
        token_type: TokenType,
        text: String,
        line: usize,
        col: usize,
        start: usize,
        end: usize,
        comments: Vec<String>,
    ) -> Token {
        Token {
            token_type,
            text,
            line,
            col,
            start,
            end,
            comments,
        }
    }

    pub fn append_comments(&mut self, comments: &mut Vec<String>) {
        self.comments.append(comments);
    }
}

#[pymethods]
impl Token {
    #[pyo3(name = "__repr__")]
    fn python_repr(&self) -> PyResult<String> {
        Ok(format!("{:?}", self))
    }
}

#[pymethods]
impl TokenType {
    #[pyo3(name = "__repr__")]
    fn python_repr(&self) -> String {
        format!("{:?}", self)
    }

    #[pyo3(name = "__hash__")]
    fn python_hash(&self) -> u64 {
        let mut hasher = std::collections::hash_map::DefaultHasher::new();
        self.hash(&mut hasher);
        hasher.finish()
    }

    #[getter]
    fn name(&self) -> String {
        self.python_repr()
    }

    #[getter]
    fn value(&self) -> String {
        self.name()
    }
}

#[pymodule]
fn sqlglotrs(_py: Python<'_>, m: &PyModule) -> PyResult<()> {
    m.add_class::<Token>()?;
    m.add_class::<TokenType>()?;
    m.add_class::<TokenizerSettings>()?;
    m.add_class::<Tokenizer>()?;
    Ok(())
}