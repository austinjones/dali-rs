use clap::{App, Arg, SubCommand};
use dali::resource::{ContainsError, PostError, ResourceError, ResourceStorage};
use dali::DaliContext;
use std::io;
use std::path::{Path, PathBuf};
use uuid::Uuid;

#[derive(Debug)]
pub enum Error {
    ResourceError(ResourceError),
    ImportPathNotFound(PathBuf),
    ContainsFileError(ContainsError),
    ReadImportsError(io::Error),
    CopyFileError(PostError),
}

pub fn main() -> Result<(), Error> {
    let matches = App::new("Dali command-line interface")
        .version("0.0.1")
        .author("Austin Jones <https://github.com/austinjones>")
        .about("Command-line interface for the Dali toolkit")
        .subcommand(
            SubCommand::with_name("import")
                .about("imports image resources into Dali storage.  deduplicates imports, so it can be run multiple times")
                .version("0.0.1")
                .arg(
                    Arg::with_name("resource")
                        .long("resource")
                        .short("u")
                        .default_value("import-512")
                        .help("Specifies the target resource name"),
                )
                .arg(
                    Arg::with_name("paths")
                        .index(1)
                        .multiple(true)
                        .help("Files or directories to scan for input"),
                ),
        )
        .get_matches();

    if let Some(import) = matches.subcommand_matches("import") {
        let mut runtime = DaliContext::new();
        let resource = import.value_of("resource").unwrap();
        let paths = import.values_of("paths");

        let resource_storage = runtime
            .resource(resource)
            .map_err(|e| Error::ResourceError(e))?;
        for path_str in paths.unwrap() {
            let path = Path::new(path_str);

            println!("Found: {}", path.to_str().unwrap(),);

            if !path.exists() {
                return Err(Error::ImportPathNotFound(path.to_path_buf()));
            }

            if path.is_dir() {
                import_dir(&resource_storage, path)?;
            } else if path.is_file() {
                import_file(&resource_storage, path)?;
            }
        }
    }

    Ok(())
}

fn import_dir(resource_storage: &ResourceStorage, path: &Path) -> Result<(), Error> {
    for path in path.read_dir().map_err(|e| Error::ReadImportsError(e))? {
        let path = path.map_err(|e| Error::ReadImportsError(e))?;
        import_file(resource_storage, path.path().as_path())?;
    }

    Ok(())
}

fn import_file(resource_storage: &ResourceStorage, path: &Path) -> Result<(), Error> {
    if resource_storage
        .contains_file(path)
        .map_err(|e| Error::ContainsFileError(e))?
    {
        println!("Skipping (already imported): {}", path.to_str().unwrap());
        return Ok(());
    }

    if !resource_storage.accepts(path) {
        println!("Skipping (invalid format): {}", path.to_str().unwrap());
        return Ok(());
    }

    let id = Uuid::new_v4();
    let target = resource_storage
        .post_file(id, path)
        .map_err(|e| Error::CopyFileError(e))?;

    println!(
        "Copied: {} to {}",
        path.to_str().unwrap(),
        target.to_str().unwrap()
    );

    Ok(())
}
