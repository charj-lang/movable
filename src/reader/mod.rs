pub use code_file::CodeFile;
use std::fs::File;
use std::io::Read;
use std::path::Path;
pub use token_element::TokenElement;

pub mod code_file;
pub mod token_element;

pub fn read_scie_data(path: &Path) -> Vec<CodeFile> {
    let mut file = File::open(path).unwrap();
    let mut data = String::new();
    file.read_to_string(&mut data).unwrap();

    return serde_json::from_str(&data).expect("error file");
}

#[cfg(test)]
mod tests {
    use crate::reader::read_scie_data;
    use std::path::PathBuf;

    #[test]
    fn should_build_first_file() {
        let mut path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        path.push("_fixtures/c/hello.c.json");

        let vec = read_scie_data(&*path);
        assert_eq!(1, vec.len());
    }
}
