use regex::Regex;
use std::{
    fs::{self, File},
    io::{Read, Write},
    process::Command,
};
use walkdir::WalkDir;

// fn create_figures_dir() -> Result<(), std::io::Error> {
//     _ = fs::create_dir("figures");

//     for path in WalkDir::new("files/ZZ_attachments") {
//         let dir_entry = path?;
//         let file_name = dir_entry.file_name().to_str().unwrap_or("");
//         if file_name.ends_with(".png") {
//             fs::copy(dir_entry.path(), format!("./dist/figures/{}", file_name))?;
//         }
//     }

//     Ok(())
// }

fn main() -> Result<(), std::io::Error> {
    fs::create_dir("./dist")?;

    // create_figures_dir()?;

    let mut text = String::new();

    for path in WalkDir::new(".") {
        let dir_entry = path?;
        let path = dir_entry.path();
        let file_name = path.file_name().and_then(|s| s.to_str());
        if let Some(file_name) = file_name {
            if file_name.ends_with(".md") {
                let mut current_file = std::fs::OpenOptions::new().read(true).open(path).unwrap();
                let mut s = String::new();
                current_file.read_to_string(&mut s).unwrap();
                text.push_str(&format!("# {}", file_name));
                text.push_str(&s);
            }
        }
    }

    let re = Regex::new("\\!\\[\\[(.*)\\]\\]").unwrap();
    let replaced = re.replace_all(&text, "![Caption](${1})");

    let mut file = File::create("dist/output.md")?;
    file.write_all(replaced.as_bytes()).unwrap();

    let output = Command::new("pandoc")
        .arg("./dist/output.md")
        .arg("-o")
        .arg("./dist/output.pdf")
        .arg("--resource-path")
        .arg("./ZZ_attachments")
        .output()
        .unwrap();

    println!("Done. Created output.md and output.pdf {:?}", output);
    Ok(())
}
