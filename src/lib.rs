mod mnx;

pub use mnx::*;

#[cfg(test)]
mod test {
    use super::*;
    use std::fs;
    use std::path::PathBuf;

    fn test_data_dir() -> PathBuf {
        let mut path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        path.push("docs");
        path.push("test-data");
        path.canonicalize().expect(&format!(
            "failed to canonicalize {}",
            path.to_string_lossy()
        ))
    }

    fn test_path(s: &str) -> PathBuf {
        let mut path = test_data_dir();
        path.push(s);
        path.canonicalize().expect(&format!(
            "failed to canonicalize {}",
            path.to_string_lossy()
        ))
    }

    #[test]
    fn test() {
        let p = test_path("hello-world.json");
        let json = fs::read_to_string(&p).unwrap();
        let mnx = serde_json::from_str::<MnxDocument>(&json).unwrap();
        let expected_barline_type = MnxDocumentGlobalMeasuresItemBarlineType::Regular;
        let actual_barline_type = mnx
            .global
            .measures
            .first()
            .unwrap()
            .barline
            .clone()
            .unwrap()
            .type_;

        // I tried using assert!(matches!()) but can't figure it out.
        match expected_barline_type {
            MnxDocumentGlobalMeasuresItemBarlineType::Regular => {}
            _ => panic!(
                "unexpected barline type, expected {:?}, got {:?}",
                expected_barline_type, actual_barline_type
            ),
        }

        // TODO: more assertions of what is in this file
    }

    // TODO: write tests for the other example files
}
