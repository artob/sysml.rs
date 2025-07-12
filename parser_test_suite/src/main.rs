use std::path::Path;

use anyhow::bail;
use comfy_table::{Cell, Color, Row, Table};
use glob::glob;

fn main() -> anyhow::Result<()> {
    let sysml_submodule_path = Path::new(env!("CARGO_MANIFEST_DIR")).join("SysML-v2-Release");
    let kerml_path = sysml_submodule_path.join("**/*.kerml");
    let sysml_path = sysml_submodule_path.join("**/*.sysml");
    let mut input_files = glob(&kerml_path.display().to_string())?
        .chain(glob(&sysml_path.display().to_string())?)
        .peekable();
    let mut oks = 0;
    let mut errs = 0;
    let mut res_table = Table::new();
    if input_files.peek().is_none() {
        bail!("No test files found, did you init the git submodule?");
    }
    res_table.set_header(vec!["File", "Result"]);
    for f in input_files {
        let mut row = Vec::new();
        let f = f?;
        let res = sysml_parser::parse_from_file(&f);
        row.push(Cell::from(format_args!(
            "{}",
            f.strip_prefix(&sysml_submodule_path).unwrap().display()
        )));
        match res {
            Ok(_) => {
                row.push(Cell::from("Ok"));
                oks += 1;
            }
            Err(_) => {
                row.push(Cell::from("Error").fg(Color::DarkRed));
                errs += 1
            }
        }
        res_table.add_row(row);
    }
    println!("{res_table}");
    let total = oks + errs;
    println!("{oks} OKs, {errs} errors, {total} total");
    Ok(())
}
