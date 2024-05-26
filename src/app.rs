use std::io;

use anyhow::Context;
use serde::de::DeserializeOwned;

use crate::Object;

pub fn parse_stdin() -> anyhow::Result<Vec<Object>> {
    parse_input_from(std::io::stdin().lock()).context("failed to parse stdin")
}
pub fn parse_input_from(base_reader: impl io::Read) -> Result<Vec<Object>, anyhow::Error> {
    parse_vec_from(base_reader).context("failed to parse all the \"knapsack objects\"")
}
fn parse_vec_from<R: DeserializeOwned>(base_reader: impl io::Read) -> Result<Vec<R>, csv::Error> {
    let mut reader = csv::ReaderBuilder::new()
        .has_headers(false)
        .from_reader(base_reader);
    reader.deserialize().collect()
}

#[cfg(test)]
mod test {
    use ordered_float::NotNan;

    #[test]
    fn parse_input_from() {
        let data = "\
            value,volume\n\
            100.0,10.0\n\
            200.0,20.0\n\
        ";
        let mut reader = std::io::Cursor::new(data);

        let result = super::parse_input_from(&mut reader).expect("Failed to parse input");

        assert_eq!(result.len(), 2);

        let obj1 = crate::Object::easy_new(100.0, 10.0);
        let obj2 = crate::Object::easy_new(200.0, 20.0);

        assert_eq!(result[0], obj1);
        assert_eq!(result[1], obj2);
    }

    #[test]
    fn parse_vec_from() {
        let data = "\
            value,volume\n\
            100.0,10.0\n\
            200.0,20.0\n\
        ";
        let mut reader = std::io::Cursor::new(data);

        let result: Vec<[NotNan<f64>; 2]> =
            super::parse_vec_from(&mut reader).expect("Failed to parse input");

        assert_eq!(result.len(), 2);

        let pair1 = [100.0, 10.0];
        let pair2 = [200.0, 20.0];

        assert_eq!(result[0], pair1);
        assert_eq!(result[1], pair2);
    }
}
