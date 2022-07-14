pub enum Prediction {
    Word(String),
    Partial(String),
}

pub struct Predictor {
    chain: PredictorChain,
    depth: usize,
}

impl Iterator for Predictor {
    type Item = Prediction;

    fn next(&mut self) -> Option<Self::Item> {
        None
    }
}

struct PredictorChain {}

struct ChainNode {}
