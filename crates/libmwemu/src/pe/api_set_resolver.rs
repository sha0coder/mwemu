use nt_apiset::ApiSetMap;
use pelite::pe64::PeFile;

#[derive(Debug)]
pub enum ApiSetResolveError {
    Io(std::io::Error),
    Parse(String),
}

impl std::fmt::Display for ApiSetResolveError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ApiSetResolveError::Io(e) => write!(f, "IO error: {}", e),
            ApiSetResolveError::Parse(e) => write!(f, "Parse error: {}", e),
        }
    }
}

impl std::error::Error for ApiSetResolveError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            ApiSetResolveError::Io(e) => Some(e),
            ApiSetResolveError::Parse(_) => None,
        }
    }
}

impl From<std::io::Error> for ApiSetResolveError {
    fn from(e: std::io::Error) -> Self {
        ApiSetResolveError::Io(e)
    }
}

pub struct ApiSetResolver {
    dll_bytes: Vec<u8>,
}

impl ApiSetResolver {
    pub fn from_file(path: &str) -> Result<Self, ApiSetResolveError> {
        let dll_bytes = std::fs::read(path)?;
        Ok(Self { dll_bytes })
    }

    pub fn resolve(&self, dll_name: &str) -> Option<String> {
        let name = dll_name
            .strip_suffix(".dll")
            .unwrap_or(dll_name)
            .to_lowercase();

        if !Self::is_api_set(&name) {
            return None;
        }

        let pe_file = PeFile::from_bytes(&self.dll_bytes).ok()?;
        let map = ApiSetMap::try_from_pe64(pe_file).ok()?;

        if let Some(host) = self.resolve_exact(&map, &name) {
            return Some(host);
        }

        self.resolve_fallback(&map, &name)
    }

    fn format_host(host: &str) -> String {
        let host = host.to_lowercase();
        if host.ends_with(".dll") {
            host
        } else {
            format!("{}.dll", host)
        }
    }

    fn resolve_exact(&self, map: &ApiSetMap, name: &str) -> Option<String> {
        let entry = map.find_namespace_entry(name)?;
        let entry = entry.ok()?;
        let value = entry.value_entries().ok()?.next()?;
        let host = value.value().ok()?;
        let host_str = host.to_string().ok()?;
        Some(Self::format_host(&host_str))
    }

    fn resolve_fallback(&self, map: &ApiSetMap, name: &str) -> Option<String> {
        let base_name = match name.rfind('-') {
            Some(pos) => &name[..pos],
            None => return None,
        };

        let entries = map.namespace_entries().ok()?;
        for entry in entries {
            let entry_name = match entry.name() {
                Ok(n) => n,
                Err(_) => continue,
            };
            let entry_str = match entry_name.to_string() {
                Ok(s) => s.to_lowercase(),
                Err(_) => continue,
            };

            if !entry_str.starts_with(base_name) {
                continue;
            }

            let entry_suffix = &entry_str[base_name.len()..];
            if !entry_suffix.is_empty() && entry_suffix.starts_with('-') && entry_suffix[1..].chars().all(|c| c.is_ascii_digit()) {
                let value = match entry.value_entries() {
                    Ok(mut iter) => match iter.next() {
                        Some(v) => v,
                        None => continue,
                    },
                    Err(_) => continue,
                };
                let host = match value.value() {
                    Ok(h) => h,
                    Err(_) => continue,
                };
                let host_str = match host.to_string() {
                    Ok(s) => s,
                    Err(_) => continue,
                };
                return Some(Self::format_host(&host_str));
            }
        }
        None
    }

    pub fn is_api_set(name: &str) -> bool {
        let lower = name.to_lowercase();
        lower.starts_with("api-") || lower.starts_with("ext-")
    }
}
