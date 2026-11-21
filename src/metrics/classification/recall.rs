use crate::{DEFAULT_THRESHOLDS, confusion_matrix};

pub fn recall(y_true: &[i32], y_pred: &[i32]) -> f32 {
    let cm = confusion_matrix(y_true, y_pred);
    if cm.true_positives + cm.false_negatives == 0.0 {
        return 0.0;
    }
    cm.true_positives / (cm.true_positives + cm.false_negatives)
}

pub fn recall_curve(y_true: &[i32], y_scores: &[f32], thresholds: Option<&[f32]>) -> Vec<f32> {
    let thresholds: &[f32] = thresholds.unwrap_or(&DEFAULT_THRESHOLDS);
    let mut recalls = Vec::with_capacity(thresholds.len());

    for (i, &threshold) in thresholds.iter().enumerate() {
        let mut y_pred = Vec::with_capacity(y_scores.len());

        for (j, &score) in y_scores.iter().enumerate() {
            y_pred.push((score >= threshold) as i32);
        }

        recalls.push(recall(y_true, &y_pred));
    }
    recalls
}
