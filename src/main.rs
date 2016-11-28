extern crate getopts;
extern crate chrono;

use getopts::Options;
use std::env;
use std::fs;
use std::path;
use std::ops::Sub;
use chrono::Datelike;

type LocalDate = chrono::DateTime<chrono::Local>;

struct ProgramParameters {
    directory: path::PathBuf,
    recursive: bool,
}

fn main() {
    let ProgramParameters { directory, recursive } = parse_arguments();
    structuralize_directory(fs::read_dir(directory), recursive).unwrap();
}

fn structuralize_directory(read_dir: std::io::Result<fs::ReadDir>, recursive: bool) -> std::result::Result<(), std::io::Error> {
    for dir_entry in read_dir.unwrap() {
        let dir_entry: fs::DirEntry = dir_entry.unwrap();
        let file_type: fs::FileType = dir_entry.file_type().unwrap();
        if file_type.is_dir() && recursive {
            structuralize_directory(fs::read_dir(dir_entry.path()), recursive).unwrap(); //todo jak niżej
        } else if file_type.is_file() {
            handle_single_file(dir_entry).unwrap(); //todo jak się jeden ne uda to nic się nie stanie
        } //w przeciwnym wypadku jest sznurkiem
    }
    Result::Ok(())
}

fn handle_single_file(file: fs::DirEntry) -> Result<(), std::io::Error> {
    let metadata: fs::Metadata = file.metadata().unwrap();
    let creation_time: std::time::SystemTime = metadata.created()
        .or(metadata.modified())
        .or(metadata.accessed()).unwrap();
    let file_age = chrono::Duration::from_std(creation_time.elapsed().unwrap()).unwrap();
    let now: chrono::DateTime<chrono::Local> = chrono::Local::now();
    let creation_time: chrono::DateTime<chrono::Local> = now.sub(file_age);
    let new_dir_name = date_to_dir_name(creation_time);

    let path = file.path();
    let path: &path::Path = path.as_path();
    if !path.parent().unwrap().ends_with(&new_dir_name) {
        let new_dir_path: path::PathBuf = path.parent().unwrap().join(new_dir_name);
        fs::create_dir_all(&new_dir_path)?;
        fs::rename(file.path(), new_dir_path.as_path().join(file.file_name()))?;
    }
    Ok(())
}

//todo jakiś parametr regeksowy żeby decydować o wyglądzie nazwy
fn date_to_dir_name(date: LocalDate) -> path::PathBuf {
    path::PathBuf::from(format!("{}-{}-{}",
                                date.year(),
                                date.month(),
                                date.day()))
}

fn parse_arguments() -> ProgramParameters {
    let args: Vec<String> = env::args().collect();
    let mut opts = Options::new();
    opts.optopt("k", "katalog", "Katalog na którym będziem pracować(domyślnie \".\")", "~/nieposortowane-zdjęcia");
    opts.optflag("r", "rekureku", "rekurencyjnie zaglębiaj się w podkatalogi");
    opts.optflag("p", "pomóż(((", "wypisz ten tekst");

    let matches = opts.parse(&args[1..]).unwrap();
    if matches.opt_present("p") {
        let program_name = args[0].clone();
        print_help_and_exit(&program_name, opts)
    }
    let recursive = matches.opt_present("r");
    let path_to_directory: String = matches.opt_str("k").unwrap_or(".".to_string());
    ProgramParameters {
        directory: path::PathBuf::from(path_to_directory),
        recursive: recursive
    }
}

fn print_help_and_exit(program_name: &str, opts: Options) {
    let brief = format!("{} [opcje]", program_name);
    println!("{}", opts.usage(&brief));
    std::process::exit(0);
}