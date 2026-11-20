use crate::confusion_matrix;

pub fn recall(y_true: &[i32], y_pred: &[i32]) -> f32 {
    let cm = confusion_matrix(y_true, y_pred);
    if cm.true_positives + cm.false_negatives == 0.0 {
        return 0.0;
    }
    cm.true_positives / (cm.true_positives + cm.false_negatives)
}