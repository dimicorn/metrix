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

#[cfg(test)]
mod tests {
    use super::*;

    // 1 true positive, 1 false positive, 1 false negative -> precision = recall = 0.5
    const Y_TRUE: [i32; 3] = [1, 0, 1];
    const Y_PRED: [i32; 3] = [1, 1, 0];

    #[test]
    fn test_f1_score() {
        assert_eq!(f1_score(&Y_TRUE, &Y_PRED, Some(1.0)), 0.5);
    }

    #[test]
    fn test_f1_score_default_beta() {
        assert_eq!(f1_score(&Y_TRUE, &Y_PRED, None), 0.5);
    }

    #[test]
    fn test_f1_score_beta_zero() {
        assert_eq!(f1_score(&Y_TRUE, &Y_PRED, Some(0.0)), 0.5);
    }
}
