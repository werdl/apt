use crate::triples::Arch;

#[derive(Debug, PartialEq, Clone, Eq)]
pub struct Version {
    pub major: u32,
    pub minor: u32,
    pub patch: u32,

    pub other: String,
}

impl PartialOrd for Version {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        if self.major != other.major {
            return Some(self.major.cmp(&other.major));
        }

        if self.minor != other.minor {
            return Some(self.minor.cmp(&other.minor));
        }

        if self.patch != other.patch {
            return Some(self.patch.cmp(&other.patch));
        }

        Some(self.other.cmp(&other.other))
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Package {
    pub name: String,
    pub version: Version,
    pub depends: Vec<Package>,
    pub arch: Arch,
    pub deb_link: String,
}

impl PartialOrd for Package {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.version.partial_cmp(&other.version)
    }
}

impl Ord for Package {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.version.partial_cmp(&other.version).unwrap()
    }
}

pub trait Filter {
    fn filter_by_arch(self, arch: Arch, name: &str) -> Vec<String>;
    fn to_pkg(self) -> Vec<Package>;
}


impl Filter for Vec<String> {
    fn filter_by_arch(self, arch: Arch, name: &str) -> Vec<String> {
        self.into_iter().filter(|x| {
            // first, strip .deb from the end
            let x = x.trim_end_matches(".deb");

            // now extract from the last underscore
            let mut parts: Vec<&str> = x.split("_").collect();

            let mut slash_parts: Vec<&str> = x.split("/").collect();

            // now extract the arch
            let found_arch = parts.pop();

            // now check if the arch is the same as the one we want
            Some(arch.to_string().as_str()) == Some(found_arch.unwrap()) && slash_parts.pop().unwrap().starts_with(name)
        }).collect()
    }

    fn to_pkg(self) -> Vec<Package> {
        self.into_iter().map(|x_link| {
            // first, strip .deb from the end
            let x = x_link.trim_end_matches(".deb");

            // now extract from the last underscore
            let mut parts: Vec<&str> = x.split("_").collect();

            // now extract the arch
            let found_arch = parts.pop().unwrap();

            // now extract the version
            let found_version = parts.pop().unwrap();


            // the name is the last part of the link
            let mut slash_parts: Vec<&str> = x.split("/").collect();

            let found_name = slash_parts.pop().unwrap();

            // now split at the first underscore and take the first part
            let found_name_list: Vec<&str> = found_name.split("_").collect();

            let found_name = found_name_list.get(0).unwrap_or(&"").to_string();

            // now extract the version
            let version_parts: Vec<&str> = found_version.split(".").collect();


            let major = version_parts.get(0).unwrap_or(&"0").parse::<u32>().unwrap();
            let minor = version_parts.get(1).unwrap_or(&"0").parse::<u32>().unwrap();
            let patch = version_parts.get(2).unwrap_or(&"0").parse::<u32>();

            // parse the patch until we reach a non-numeric character
            let patch = match patch {
                Ok(_) => patch,
                Err(_) => {
                    let mut patch = version_parts.get(2).unwrap_or(&"0").chars();
                    let mut patch_str = String::new();
                    while let Some(c) = patch.next() {
                        if c.is_numeric() {
                            patch_str.push(c);
                        } else {
                            break;
                        }
                    }

                    patch_str.parse::<u32>()
                }
            };

            let other = match patch {
                Ok(_) => version_parts.get(2).unwrap_or(&""),
                Err(_) => version_parts.get(2).unwrap_or(&"0"),
            };

            // now remove the patch from the other string
            let other = other.trim_start_matches(patch.clone().unwrap_or(0).to_string().as_str());

            let patch = patch.unwrap_or(0);

            Package {
                name: found_name.to_string(),
                version: Version {
                    major,
                    minor,
                    patch,
                    other: other.to_string(),
                },
                depends: Vec::new(),
                arch: Arch::from(found_arch),
                deb_link: x_link.clone(),
            }
        }).collect()
    }
}