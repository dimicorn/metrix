use crate::{precision_curve, recall_curve};

pub fn pr_auc(y_true: &[i32], y_scores: &[f32], thresholds: Option<&[f32]>) -> f32 {
    let recalls = recall_curve(y_true, y_scores, thresholds);
    let precisions = precision_curve(y_true, y_scores, thresholds);

    let mut pr = 0.0;
    for i in 1..precisions.len() {
        pr += (recalls[i] - recalls[i - 1]) * (precisions[i] + precisions[i - 1]) / 2.0;
    }
    pr
}
