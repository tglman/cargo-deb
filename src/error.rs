use std::io;
use std::time;
use compress::{CompressErr, Archive};
use toml;

quick_error! {
    #[derive(Debug)]
    pub enum CargoDebError {
        Io(err: io::Error) {
            from()
            description(err.description())
            display("I/O error: {}", err)
            cause(err)
        }
        Str(msg: &'static str) {
            from()
            description(msg)
        }
        ArFailed {
            description("ar failed")
        }
        BuildFailed {
            description("build failed")
        }
        StripFailed {
            description("strip failed")
        }
        SystemTime(err: time::SystemTimeError) {
            from()
            description("unable to get system time")
            cause(err)
        }
        Parse(err: toml::de::Error) {
            from()
            description(err.description())
            display("TOML error: {}", err)
            cause(err)
        }
        Compress(err: CompressErr) {
            from()
            description(match *err {
                CompressErr::Compression(_) => "error with zopfli compression",
                // The application was unable to create the `target/debian` directory.
                CompressErr::UnableToCreatePath(_) => "unable to create 'target/debian'",
                // The application was unable to write the archive to disk.
                CompressErr::Write(Archive::Control, _) => "unable to write to 'target/debian/control.tar.gz'",
                CompressErr::Write(Archive::Data, _) => "unable to write to 'target/debian/data.tar.xz'",
            })
            display("{}", match *err {
                CompressErr::Compression(ref reason) => reason.clone(),
                CompressErr::UnableToCreatePath(ref reason) => reason.to_string(),
                CompressErr::Write(_, ref reason) => reason.to_string(),
            })
        }
    }
}

pub type CDResult<T> = Result<T, CargoDebError>;
