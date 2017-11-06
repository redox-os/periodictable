#![allow(dead_code)]

#[derive(Copy, Clone)]
pub enum Threshold<T: PartialOrd> {
    LessOrEqual(T),
    GreaterOrEqual(T),
    Between(T, T),
    Equals(T),
}

fn eval_threshold<T: PartialOrd>(threshold: Threshold<T>, value: T) -> bool {
    match threshold {
        Threshold::LessOrEqual(v) => { value <= v },
        Threshold::GreaterOrEqual(v) => { value >= v },
        Threshold::Between(v1, v2) => { value >= v1 && value <= v2 },
        Threshold::Equals(v) => { value == v },
    }
}

fn eval_threshold_option<T: PartialOrd>(threshold: Threshold<T>, value: Option<T>) -> bool {
    if let Some(v) = value {
        eval_threshold(threshold, v)
    } else {
        false
    }
}
