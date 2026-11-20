use crate::confusion_matrix;

pub fn precision(y_true: &[i32], y_pred: &[i32]) -> f32 {
    let cm = confusion_matrix(y_true, y_pred);
    if cm.true_positives + cm.false_positives == 0.0 {
        return 0.0;
    }
    cm.true_positives / (cm.true_positives + cm.false_positives)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_precision() {
        // 5 true positives, 5 false positives
        let y_true = [1, 1, 1, 1, 1, 0, 0, 0, 0, 0];
        let y_pred = [1, 1, 1, 1, 1, 1, 1, 1, 1, 1];
        assert_eq!(precision(&y_true, &y_pred), 0.5);
    }
}
