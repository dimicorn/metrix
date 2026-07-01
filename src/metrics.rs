fn calc_precision(true_positives: f32, false_positives: f32) -> f32 {
    if true_positives + false_positives == 0.0 {
        return 0.0;
    }
    true_positives / (true_positives + false_positives)
}

fn calc_recall(true_positives: f32, false_negatives: f32) -> f32 {
    if true_positives + false_negatives == 0.0 {
        return 0.0;
    }
    true_positives / (true_positives + false_negatives)
}

fn calc_f1_score(precision: f32, recall: f32, beta: f32) -> f32 {
    let beta_squared = beta * beta;
    if precision + recall == 0.0 {
        return 0.0;
    }
    (1.0 + beta_squared) * precision * recall / (beta_squared * precision + recall)
}

pub fn calc_metrics(y_true_values: &[i32], y_pred_values: &[i32], beta: f32) -> (f32, f32, f32) {
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
    let precision = calc_precision(true_positives, false_positives);
    let recall = calc_recall(true_positives, false_negatives);
    let f1_score = calc_f1_score(precision, recall, beta);

    (precision, recall, f1_score)
}
