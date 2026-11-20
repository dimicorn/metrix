use crate::{recall, precision};

pub fn f1_score(y_true: &[i32], y_pred: &[i32], beta: Option<f32>) -> f32 {
    let beta = beta.unwrap_or(1.0);
    let prec = precision(y_true, y_pred);
    let rec = recall(y_true, y_pred);

    let beta_squared = beta * beta;
    if prec + rec == 0.0 {
        return 0.0;
    }
    (1.0 + beta_squared) * prec * rec / (beta_squared * prec + rec)
}
