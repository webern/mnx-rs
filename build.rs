use std::fs;
use std::path::PathBuf;
use typify::{TypeSpace, TypeSpaceSettings};

const MNX_SCHEMA_PATH: &str = "./docs/mnx-schema.json";
const OUT_PATH: &str = "./src/mnx.rs";

fn main() {
    let schema_path = canonicalize(MNX_SCHEMA_PATH);
    let content = std::fs::read_to_string(&schema_path).unwrap();
    let schema = serde_json::from_str::<schemars::schema::RootSchema>(&content).unwrap();

    let mut type_space = TypeSpace::new(TypeSpaceSettings::default().with_struct_builder(true));
    type_space.add_root_schema(schema).unwrap();

    let contents =
        prettyplease::unparse(&syn::parse2::<syn::File>(type_space.to_stream()).unwrap());
    let out_file = canonicalize(OUT_PATH);
    let _ = fs::remove_file(OUT_PATH);
    fs::write(out_file, contents).unwrap();
}

fn canonicalize(s: &str) -> PathBuf {
    let raw = PathBuf::from(s);
    if s == MNX_SCHEMA_PATH {
        raw.canonicalize().expect(&format!(
            "File does not exist at this path: {}",
            raw.to_string_lossy()
        ))
    } else {
        raw
    }
}
