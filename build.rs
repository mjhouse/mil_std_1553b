use std::collections::HashMap;
use std::fs::{read_dir,read_to_string};
use std::fs::File;
use std::io::Write;
use std::path::PathBuf;
use serde::Deserialize;
use toml::Value;

#[derive(Deserialize,Debug)]
struct WordDefinition {
    kind: String,
    value: u16,
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

        // TODO: implement other tomls and remove
        if name != "words" { continue; }

        build_tests(name,&path,&test);
    }
}

fn build_tests(kind: &str, path: &PathBuf, test: &PathBuf) {

    let mut output = File::create(&test).unwrap();
    write!(&mut output,"use mil_std_1553b::*;\n").unwrap();

    for file in read_dir(path).unwrap().map(Result::unwrap) {
        let path = file.path();
        let name = path.file_stem().unwrap().to_str().unwrap();
        let data = read_to_string(&path).unwrap();
        
        match kind {
            "messages" => build_message_test(&mut output, name, data),
            "packets" => build_packet_test(&mut output, name, data),
            "words" => build_word_test(&mut output, name, data),
            _ => unimplemented!()
        }
    }
}

fn build_message_test(_: &mut File, _: &str, _: String) {
    unimplemented!()
}

fn build_packet_test(_: &mut File, _: &str, _: String) {
    unimplemented!()
}

fn build_word_test(output: &mut File, name: &str, data: String) {
    let definition = toml::from_str::<WordDefinition>(&data).unwrap();
    let mut checks = String::new();

    let kind = definition.kind;
    let value = format!("{:#018b}",definition.value);

    for (k,v) in definition.result {
        let check = word_check(&k,&v);
        checks.push_str(check.as_str());
    }

    write!(output,
        indoc::indoc!{ r#"

        #[test]
        fn test_{}() {{
            let word = {}::from({});
        {}}}
        "# },
        name,
        kind,
        value,
        checks
    ).unwrap();
}

fn word_check(key: &String, initial: &Value) -> String {
    let value = match initial {
        Value::String(n) => n.clone(),
        k => k.to_string()
    };

    let method = match initial {
        Value::Integer(_) => "u8.into()".to_string(),
        _ => String::new(),
    };

    let message = format!("{} != {}",key, value);
    format!("    assert!( word.{}() == {}{}, \"{}\");\n", key, value, method, message)
}