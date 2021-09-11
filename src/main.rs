use pyo3::prelude::*;

#[derive(Debug)]
struct Token {
    sentence: String,
    text: String,
    pos: String,
    lemma: String,
}

#[derive(Debug)]
struct Tokenizer {
    nlp: PyObject,
}

impl Tokenizer {
    fn new() -> Self {
        Python::with_gil(|py| -> PyResult<Tokenizer> {
            let spacy = PyModule::import(py, "spacy")?;
            let tokenizer_module = PyModule::from_code(py, r#"

def tokenizer(spacy):
    nlp = spacy.load("ja_core_news_lg")
    return nlp

            "#, "tokenizer.py", "tokenizer")?;

            let nlp: PyObject  = tokenizer_module.getattr("tokenizer")?.call1((spacy,))?.extract()?;

            Ok(Tokenizer {
                nlp,
            })
        }).unwrap()

    }

    fn tokenize(&self, sentence: &str) -> Vec<Token> {
        Python::with_gil(|py| -> PyResult<Vec<Token>> {
            let tokenizer = PyModule::from_code(py, r#"

def tokenize(nlp, sentence):
    doc = nlp(sentence)
    return doc

        "#, "tokenizer.py", "tokenizer")?;

        let tokens: Vec<&PyAny> = tokenizer.getattr("tokenize")?.call1((self.nlp.as_ref(py), sentence))?.extract()?;

        Ok(tokens.iter().map(|x| Token {
                sentence: sentence.to_string(),
                text: x.getattr("text").unwrap().to_string(),
                pos: x.getattr("pos_").unwrap().to_string(),
                lemma: x.getattr("lemma_").unwrap().to_string()
                }).collect())

        }).unwrap()
    }
}

fn main() {
    let tokenizer = Tokenizer::new();

    let sentences = vec!["でも、方針も決まったし、もう私に迷いは無い！", "ベッドの上に立ち上がり騒いでいたら、またお母さんに怒られた。", "そして、私はそのままベッドに潜り込んで眠りにつく。良い夢が……陽信の夢が見られるといいな。"];

    for sentence in sentences.iter() {
        let tokens = tokenizer.tokenize(sentence);

        println!("{:?}", sentence);
        
        for token in tokens.iter() {
            println!("{:?} {:?} {:?}", token.text, token.lemma, token.pos);
        }
    }



}

