use crate::confusion_matrix;

pub fn recall(y_true: &[i32], y_pred: &[i32]) -> f32 {
    let cm = confusion_matrix(y_true, y_pred);
    if cm.true_positives + cm.false_negatives == 0.0 {
        return 0.0;
    }
    cm.true_positives / (cm.true_positives + cm.false_negatives)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_recall() {
        // 5 true positives, 5 false negatives
        let y_true = [1, 1, 1, 1, 1, 1, 1, 1, 1, 1];
        let y_pred = [1, 1, 1, 1, 1, 0, 0, 0, 0, 0];
        assert_eq!(recall(&y_true, &y_pred), 0.5);
    }
}
