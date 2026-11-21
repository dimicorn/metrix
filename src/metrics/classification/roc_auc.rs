use crate::{DEFAULT_THRESHOLDS, confusion_matrix, recall_curve};

pub fn false_positive_rate(y_true: &[i32], y_pred: &[i32]) -> f32 {
    let cm = confusion_matrix(y_true, y_pred);
    if cm.true_negatives + cm.false_positives == 0.0 {
        return 0.0;
    }
    cm.false_positives / (cm.false_positives + cm.true_negatives)
}

pub fn false_positive_rate_curve(
    y_true: &[i32],
    y_scores: &[f32],
    thresholds: Option<&[f32]>,
) -> Vec<f32> {
    let thresholds: &[f32] = thresholds.unwrap_or(&DEFAULT_THRESHOLDS);
    let mut fprs = Vec::with_capacity(thresholds.len());

    for (i, &threshold) in thresholds.iter().enumerate() {
        let mut y_pred = Vec::with_capacity(y_scores.len());

        for (j, &score) in y_scores.iter().enumerate() {
            y_pred.push((score >= threshold) as i32);
        }

        fprs.push(false_positive_rate(y_true, &y_pred));
    }
    fprs
}

pub fn roc_auc(y_true: &[i32], y_scores: &[f32], thresholds: Option<&[f32]>) -> f32 {
    let recalls = recall_curve(y_true, y_scores, thresholds);
    let fprs = false_positive_rate_curve(y_true, y_scores, thresholds);

    let mut auc = 0.0;
    for i in 1..fprs.len() {
        auc += (fprs[i] - fprs[i - 1]) * (recalls[i] + recalls[i - 1]) / 2.0;
    }
    auc
}
