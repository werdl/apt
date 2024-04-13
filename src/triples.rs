//! triples
//! 
//! contains definitions and function for generating and dealing with host triples

pub enum Os {
    Linux // currently only Linux is supported
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Arch { // everything supported by dpkg (we rip their binaries from .deb files)
    Amd64,
    Arm64,
    Armel,
    Armhf,
    I386,
    Mips,
    Mips64el,
    Mipsel,
    Ppc64el,
    S390x
}

pub enum Toolchain { // this is not super relevant right now, but will be useful when building from source is supported
    Gcc,
    Llvm
}

pub struct Triple {
    pub os: Os,
    pub arch: Arch,
    pub toolchain: Toolchain
}

impl Triple {
    pub fn new(os: Os, arch: Arch, toolchain: Toolchain) -> Triple {
        Triple {
            os: os,
            arch: arch,
            toolchain: toolchain
        }
    }
}

impl core::fmt::Display for Arch {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        match self {
            Arch::Amd64 => write!(f, "amd64"),
            Arch::Arm64 => write!(f, "arm64"),
            Arch::Armel => write!(f, "armel"),
            Arch::Armhf => write!(f, "armhf"),
            Arch::I386 => write!(f, "i386"),
            Arch::Mips => write!(f, "mips"),
            Arch::Mips64el => write!(f, "mips64el"),
            Arch::Mipsel => write!(f, "mipsel"),
            Arch::Ppc64el => write!(f, "ppc64el"),
            Arch::S390x => write!(f, "s390x")
        }
    }
}

impl From<&str> for Arch {
    fn from(s: &str) -> Arch {
        match s {
            "amd64" => Arch::Amd64,
            "arm64" => Arch::Arm64,
            "armel" => Arch::Armel,
            "armhf" => Arch::Armhf,
            "i386" => Arch::I386,
            "mips" => Arch::Mips,
            "mips64el" => Arch::Mips64el,
            "mipsel" => Arch::Mipsel,
            "ppc64el" => Arch::Ppc64el,
            "s390x" => Arch::S390x,
            _ => panic!("Unsupported arch")
        }
    }
}   

impl From<&str> for Triple {
    fn from(s: &str) -> Triple {
        let parts: Vec<String> = s.split('-').collect::<Vec<&str>>().iter().map(
            |x| x.to_lowercase()
        ).collect();

        match parts.len() {
            3 => (),
            _ => panic!("Invalid triple")
        }

        let os = match parts[0].as_str() {
            "linux" => Os::Linux,
            _ => panic!("Unsupported OS")
        };

        let arch = match parts[1].as_str() {
            "amd64" => Arch::Amd64,
            "arm64" => Arch::Arm64,
            "armel" => Arch::Armel,
            "armhf" => Arch::Armhf,
            "i386" => Arch::I386,
            "mips" => Arch::Mips,
            "mips64el" => Arch::Mips64el,
            "mipsel" => Arch::Mipsel,
            "ppc64el" => Arch::Ppc64el,
            "s390x" => Arch::S390x,
            _ => panic!("Unsupported arch")
        };
        let toolchain = match parts[2].as_str() {
            "llvm" | "clang" => Toolchain::Llvm,
            _ => Toolchain::Gcc
        };
        Triple::new(os, arch, toolchain)
    }
}

impl core::fmt::Display for Triple {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        let os = match self.os {
            Os::Linux => "linux"
        };
        let arch = match self.arch {
            Arch::Amd64 => "amd64",
            Arch::Arm64 => "arm64",
            Arch::Armel => "armel",
            Arch::Armhf => "armhf",
            Arch::I386 => "i386",
            Arch::Mips => "mips",
            Arch::Mips64el => "mips64el",
            Arch::Mipsel => "mipsel",
            Arch::Ppc64el => "ppc64el",
            Arch::S390x => "s390x"
        };
        let toolchain = match self.toolchain {
            Toolchain::Gcc => "gcc",
            Toolchain::Llvm => "llvm"
        };
        write!(f, "{}-{}-{}", os, arch, toolchain)
    }
}