pub(crate) struct ConfusionMatrix {
    pub true_positives: f32,
    pub true_negatives: f32,
    pub false_positives: f32,
    pub false_negatives: f32,
}

pub(crate) fn confusion_matrix(y_true: &[i32], y_pred: &[i32]) -> ConfusionMatrix {
    let mut true_positives = 0 as f32;
    let mut true_negatives = 0 as f32;
    let mut false_positives = 0 as f32;
    let mut false_negatives = 0 as f32;

    for (gt, pred) in y_true.iter().zip(y_pred) {
        match (gt, pred) {
            (0, 0) => true_negatives += 1.0,
            (1, 1) => true_positives += 1.0,
            (0, 1) => false_positives += 1.0,
            (1, 0) => false_negatives += 1.0,
            _ => {}
        }
    }

    ConfusionMatrix {
        true_positives,
        true_negatives,
        false_positives,
        false_negatives,
    }
}
