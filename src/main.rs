extern crate getopts;
extern crate chrono;

use getopts::Options;
use std::env;
use std::fs;
use std::path;
use std::ops::Sub;
use chrono::Datelike;
use std::error::Error;
type LocalDate = chrono::DateTime<chrono::Local>;

struct Opcje {
    katalog: path::PathBuf,
    recursive: bool,
    //todo ogarnianie kategoryzacji względem regexa nazwy, daty ogólnej itp
}

enum AnyError {
    SystemTimeError,
    IoError,
}

impl From<std::io::Error> for AnyError {
    fn from(e: std::io::Error) -> Self {
        println!("{}",e.description());
        AnyError::IoError
    }
}

impl std::convert::From<std::time::SystemTimeError> for AnyError {
    fn from(_: std::time::SystemTimeError) -> Self {
        AnyError::SystemTimeError
    }
}

fn main() {
    let Opcje { katalog: katalog, recursive: rekurencyjnie } = parse_arguments();
    println!("katalog {:?}", katalog.as_path().canonicalize());
    let res: std::io::Result<fs::ReadDir> = fs::read_dir(katalog);
    match do_the_job(res.unwrap(), rekurencyjnie) {
        Ok(()) => println!("SKPOX"),
        _ => println!("NIE UDALS")
    };
}

fn do_the_job(read_dir: fs::ReadDir, reku: bool) -> Result<(), AnyError> {
    println!("rekurencyjny? {}", reku);
    for dir_entry in read_dir {
        let dir_entry: fs::DirEntry = dir_entry?;
        let file_type: fs::FileType = dir_entry.file_type()?;
        if file_type.is_dir() {
            println!("{:?} - folder", dir_entry.file_name())
        } else if file_type.is_symlink() {
            println!("{:?} - symlink", dir_entry.file_name())
        } else if file_type.is_file() {
            println!("{:?} - file", dir_entry.file_name());
            let file: fs::DirEntry = dir_entry;
            let metadata: fs::Metadata = file.metadata()?; //todo: wyrzucić do osobnej funkcji żeby TRY! nie kończyło programu
            let system_time: std::time::SystemTime = metadata.created()
                .or(metadata.modified())
                .or(metadata.accessed())?;
            //u mmnie w systemie nie ma created, trzeba będzie to ogarnąć w kodzie i iść do accessed jak nie ma
            println!("{:?}", system_time.elapsed().unwrap()); //ile do teraz
            let duration = chrono::Duration::from_std(system_time.elapsed()?);
            let now: chrono::DateTime<chrono::Local> = chrono::Local::now();
            let data: chrono::DateTime<chrono::Local> = now.sub(duration.unwrap());
            println!("{}", data); //DZIAŁA :DDDDDDDDDDDDDDDD
            let dirname = date_to_dirname(data);
            println!("{:?}", &dirname);
            //todo sprawdzić czy właśnie nie jesteśmy w takim folderze
            let newf: path::PathBuf = file.path().as_path().parent().unwrap().join(dirname);
//            println!("{:?}", &newf);
             fs::create_dir_all(&newf)?;
            fs::rename(file.path(), newf.as_path().join(file.file_name()))?;
        }
    }
    Result::Ok(())
}

fn date_to_dirname(data: LocalDate) -> path::PathBuf{
    let year= data.year();
    let month = data.month();
    let day = data.day();
    //todo jakiś parametr regeksowy żeby decydować o wyglądzie nazwy
    path::PathBuf::from(format!("{}-{}-{}", year, month, day))
}

fn parse_arguments() -> Opcje {
    let args: Vec<String> = env::args().collect();


    let mut opts = Options::new();
    opts.optopt("k", "katalog", "Katalog na którym będziem pracować(domyślnie \".\")", "~/nieposortowane-zdjęcia");
    opts.optflag("r", "rekureku", "rekurencyjnie zaglębiaj się w katalogi (jeszcze niezrobione)");
    opts.optflag("p", "pomóż(((", "wypisz ten tekst");

    let matches = match opts.parse(&args[1..]) {
        Ok(m) => m,
        Err(f) => panic!(f.to_string()),
    };

    if matches.opt_present("p") {
        let program = args[0].clone();
        print_help_and_exit(&program, opts)
    }

    let recursive = matches.opt_present("r");

    let katalog: String = matches.opt_str("k").unwrap_or(".".to_string());
    let katalog = path::PathBuf::from(katalog);

    Opcje {
        katalog: katalog,
        recursive: recursive
    }
}

fn print_help_and_exit(nazwa_programu: &str, opts: Options) {
    let biref = format!("{} [opcje]", nazwa_programu);
    println!("{}", opts.usage(&biref));
    std::process::exit(0);
}