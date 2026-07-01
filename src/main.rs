use clap::{Command, arg};

mod parse;
use parse::parse_data;
mod metrics;
use metrics::calc_metrics;

mod constants {
    pub const ID: &str = "id";
    pub const Y_TRUE: &str = "y_true";
    pub const Y_PRED: &str = "y_pred";
}

fn get_args() -> (String, String, String, String, f32) {
    let matches = Command::new("f1-calc")
        .arg(arg!(-i --input <FILE> "Provides an input file to the program").required(true))
        .arg(arg!(--id <COLUMN> "Name of the ID column").default_value(constants::ID))
        .arg(arg!(--"y-true" <COLUMN> "Name of the y true column").default_value(constants::Y_TRUE))
        .arg(arg!(--"y-pred" <COLUMN> "Name of the y pred column").default_value(constants::Y_PRED))
        .arg(arg!(--"beta" <BETA> "Beta value for F-beta score").default_value("1.0"))
        .get_matches();


    let file_path = matches
        .get_one::<String>("input")
        .expect("input file is required");

    let id_col = matches
        .get_one::<String>("id")
        .expect("id column must be a number");

    let y_true_col = matches
        .get_one::<String>("y-true")
        .expect("y true column must be a number");

    let y_pred_col = matches
        .get_one::<String>("y-pred")
        .expect("y pred column must be a number");

    let beta = matches
        .get_one::<String>("beta")
        .expect("beta value must be a number")
        .parse::<f32>()
        .expect("beta value must be a valid number");

    (
        file_path.to_string(),
        id_col.to_string(),
        y_true_col.to_string(),
        y_pred_col.to_string(),
        beta
    )
}

fn main() {
    let (file_path, id_col, y_true_col, y_pred_col, beta) = get_args();
    let (y_true_values, y_pred_values) = parse_data(&file_path, &id_col, &y_true_col, &y_pred_col);
    let (precision, recall, f1_score) = calc_metrics(&y_true_values, &y_pred_values, beta);

    println!(
        "Precision: {:.3}, Recall: {:.3}, F1 score: {:.3}",
        precision, recall, f1_score
    );
}
