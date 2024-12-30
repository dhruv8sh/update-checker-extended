use std::fmt::Display;

#[cxx::bridge(namespace = "rust")]
mod ffi {
    extern "Rust" {
        type Backend;
        fn make_backend() -> Box<Backend>;
        fn is_busy(&self) -> bool;
        fn is_error(&self) -> bool;
        fn msg(&self) -> String;
    }
}
#[derive(Default)]
pub struct Backend {
    state: State,
    updates_available: Vec<Package>
}
struct Package {
    name: String,
}
#[derive(Default)]
enum State {
    Busy(BusyType),
    #[default]
    Idle,
    Error(CustomError),
}
enum BusyType {
    FetchingUpdate(PackageType),
    RemovingPackage(PackageType, String),
    InstallingPackage(PackageType, String),
    UpdatingSystem,
    Calculating,
    MultipleProcesses,
}
enum PackageType {
    Flatpak,
    AUR,
    Arch(String),
    All,
    Unknown,
}
enum CustomError {}
impl Display for PackageType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let str = match self {
            PackageType::Flatpak => "Flatpak".to_string(),
            PackageType::AUR => "AUR".to_string(),
            PackageType::Arch(repo) => repo.to_string(),
            PackageType::All => "All".to_string(),
            PackageType::Unknown => "`Unknown`".to_string(),
        };
        write!(f, "{}", str)
    }
}


impl Backend {
    pub fn msg(&self) -> String {
        match &self.state {
            State::Busy(busy_type) => match busy_type {
                BusyType::FetchingUpdate(pkg_type) => format!("checking {} updates", pkg_type),
                BusyType::RemovingPackage(pkg_type, name) => format!("Removing {} Package: {}", pkg_type, name),
                BusyType::InstallingPackage(pkg_type, name) => format!("Installing {} Package: {}", pkg_type, name),
                BusyType::UpdatingSystem => "Updating System".to_string(),
                BusyType::Calculating => "Calculating...".to_string(),
                BusyType::MultipleProcesses => "Multiple Operations in Progress".to_string(),
            },
            State::Idle => if self.updates_available.is_empty() { String::from("Bruh!") }
                else { format!("{} updates available", self.updates_available.len()) }
            State::Error(err) => format!("Error: {}\nCode:{}", err, err.code()),
        }
    }
    pub fn is_busy(&self) -> bool {
        matches!(&self.state, State::Busy(_))
    }
    pub fn is_error(&self) -> bool {
        matches!(&self.state, State::Error(_))
    }
}

impl Display for CustomError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Some error occured!! Error list unimplemented")
    }
}
impl CustomError {
    fn code(&self) -> i32 { -1 }
}

pub fn make_backend() -> Box<Backend> {
    Box::<Backend>::default()// from config
}
pub fn fetch_updates() {
    todo!()
}
