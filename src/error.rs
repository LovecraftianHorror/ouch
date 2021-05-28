use std::{fmt, path::PathBuf};

use crate::{oof, utils::colors};

#[derive(Debug, PartialEq)]
pub enum Error {
    UnknownExtensionError(String),
    MissingExtensionError(PathBuf),
    // TODO: get rid of this error variant
    InvalidUnicode,
    InvalidInput,
    IoError { reason: String },
    FileNotFound(PathBuf),
    AlreadyExists,
    InvalidZipArchive(&'static str),
    PermissionDenied,
    UnsupportedZipArchive(&'static str),
    InternalError,
    OofError(oof::OofError),
    CompressingRootFolder,
    MissingArgumentsForCompression,
    CompressionTypo,
    WalkdirError { reason: String },
}

pub type Result<T> = std::result::Result<T, Error>;

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Error::MissingExtensionError(filename) => {
                write!(f, "{}[ERROR]{} ", colors::red(), colors::reset())?;
                // TODO: show MIME type of the unsupported file
                write!(f, "cannot compress to {:?}, likely because it has an unsupported (or missing) extension.", filename)
            },
            Error::WalkdirError { reason } => {
                write!(f, "{}[ERROR]{} {}", colors::red(), colors::reset(), reason)
            },
            Error::FileNotFound(file) => {
                write!(f, "{}[ERROR]{} ", colors::red(), colors::reset())?;
                if file == &PathBuf::from("") {
                    return write!(f, "file not found!");
                }
                write!(f, "file {:?} not found!", file)
            },
            Error::CompressingRootFolder => {
                write!(f, "{}[ERROR]{} ", colors::red(), colors::reset())?;
                let spacing = "        ";
                writeln!(f, "It seems you're trying to compress the root folder.")?;
                writeln!(
                    f,
                    "{}This is unadvisable since ouch does compressions in-memory.",
                    spacing
                )?;
                write!(
                    f,
                    "{}Use a more appropriate tool for this, such as {}rsync{}.",
                    spacing,
                    colors::green(),
                    colors::reset()
                )
            },
            Error::MissingArgumentsForCompression => {
                write!(f, "{}[ERROR]{} ", colors::red(), colors::reset())?;
                let spacing = "        ";
                writeln!(f,"The compress subcommands demands at least 2 arguments, an input file and an output file.")?;
                writeln!(f, "{}Example: `ouch compress img.jpeg img.zip`", spacing)?;
                write!(f, "{}For more information, run `ouch --help`", spacing)
            },
            Error::InternalError => {
                write!(f, "{}[ERROR]{} ", colors::red(), colors::reset())?;
                write!(f, "You've reached an internal error! This really should not have happened.\nPlease file an issue at {}https://github.com/vrmiguel/ouch{}", colors::green(), colors::reset())
            },
            Error::OofError(err) => {
                write!(f, "{}[ERROR]{} {}", colors::red(), colors::reset(), err)
            },
            Error::IoError { reason } => {
                write!(f, "{}[ERROR]{} {}", colors::red(), colors::reset(), reason)
            },
            Error::CompressionTypo => {
                write!(f, "Did you mean {}ouch compress{}?", colors::magenta(), colors::reset())
            },
            _err => {
                todo!();
            },
        }
    }
}

impl From<std::io::Error> for Error {
    fn from(err: std::io::Error) -> Self {
        match err.kind() {
            std::io::ErrorKind::NotFound => panic!("{}", err),
            std::io::ErrorKind::PermissionDenied => Self::PermissionDenied,
            std::io::ErrorKind::AlreadyExists => Self::AlreadyExists,
            _other => Self::IoError { reason: err.to_string() },
        }
    }
}

impl From<zip::result::ZipError> for Error {
    fn from(err: zip::result::ZipError) -> Self {
        use zip::result::ZipError::*;
        match err {
            Io(io_err) => Self::from(io_err),
            InvalidArchive(filename) => Self::InvalidZipArchive(filename),
            FileNotFound => Self::FileNotFound("".into()),
            UnsupportedArchive(filename) => Self::UnsupportedZipArchive(filename),
        }
    }
}

impl From<walkdir::Error> for Error {
    fn from(err: walkdir::Error) -> Self {
        Self::WalkdirError { reason: err.to_string() }
    }
}

impl From<oof::OofError> for Error {
    fn from(err: oof::OofError) -> Self {
        Self::OofError(err)
    }
}
