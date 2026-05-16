pub fn calc_metrics(y_true_values: &[i32], y_pred_values: &[i32]) -> (f32, f32, f32) {
    let mut true_positives = 0 as f32;
    let mut false_positives = 0 as f32;
    let mut false_negatives = 0 as f32;

    for (y_true, y_pred) in y_true_values.iter().zip(y_pred_values.iter()) {
        if *y_true == 0 && *y_pred == 0 {
            true_positives += 1.0;
        } else if *y_true == 1 && *y_pred == 0 {
            false_negatives += 1.0;
        } else if *y_true == 0 && *y_pred == 1 {
            false_positives += 1.0;
        }
    }
    let precision = true_positives / (true_positives + false_positives);
    let recall = true_positives / (true_positives + false_negatives);
    let f1_score = 2.0 * precision * recall / (precision + recall);

    (precision, recall, f1_score)
}
