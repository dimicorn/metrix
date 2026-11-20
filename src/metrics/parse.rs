use csv::Reader;

pub fn parse_data(file_path: &str, id_col: &str, y_true_col: &str, y_pred_col: &str) -> (Vec<i32>, Vec<i32>) {
    let mut rdr = Reader::from_path(file_path).unwrap();
    let headers = rdr.headers().unwrap().clone();

    let mut y_true: Vec<i32> = vec![];
    let mut y_pred: Vec<i32> = vec![];

    for result in rdr.records() {
        let record = result.unwrap();
        for (i, col) in headers.iter().enumerate() {
            if col == id_col {
                continue;
            } else if col == y_true_col {
                y_true.push(record[i].parse::<i32>().unwrap())
            } else if col == y_pred_col {
                y_pred.push(record[i].parse::<i32>().unwrap())
            }
        }
    }
    (y_true, y_pred)
}