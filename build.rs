use std::env;
use std::collections::HashMap;
use std::fs::{read_dir,read_to_string};
use std::fs::DirEntry;
use std::fs::File;
use std::io::Write;
use std::path::Path;
use std::path::PathBuf;
use serde::Deserialize;
use toml::Value;

#[derive(Deserialize,Debug)]
struct Data {
    kind: String,
    from: Value,
    method: String
}

#[derive(Deserialize,Debug)]
struct Definition {
    data: Data,
    result: HashMap<String,Value>
}

fn main() {
    let root = PathBuf::from("tests").canonicalize().unwrap();
    let data = root.join("definitions");

    println!("cargo:rerun-if-changed={}", root.display());

    for directory in read_dir(data).unwrap().map(Result::unwrap) {
        let path = directory.path();
        let name = path.file_name().unwrap().to_str().unwrap();
        let test = root.join(name).with_extension("rs");
        build_tests(&path,&test);
    }
}

fn build_tests(path: &PathBuf, test: &PathBuf) {

    let mut output = File::create(&test).unwrap();
    write!(&mut output,"use mil_std_1553b::*;").unwrap();

    for file in read_dir(path).unwrap().map(Result::unwrap) {
        let path = file.path();
        let name = path.file_stem().unwrap().to_str().unwrap();
        let data = read_to_string(&path).unwrap();
        
        let definition = toml::from_str::<Definition>(&data).unwrap();

        let mut checks = String::new();

        for (k,v) in definition.result {
            let value = match v {
                Value::String(n) => n,
                k => k.to_string()
            };

            let check = format!("    assert_eq!( item.{}(), {} );\n", k, value);

            checks.push_str(check.as_str());
        }

        write!(output,
            indoc::indoc!{ r#"

            #[test]
            fn test_{}() {{
                let item = {}::{}({});
            {}}}
            "# },
            name,
            definition.data.kind,
            definition.data.method,
            definition.data.from,
            checks
        ).unwrap();
    }
}
