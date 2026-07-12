use crate::packages::package::Package;
use std::io;

enum PackageManager {
    Pacman,
    Apt,
    AptGet,
    Rpm,
    Dnf,
    Zypper,
    Dpkg,
}

enum PackageManagerType {
    OS,
    Language,
    Container,
}
