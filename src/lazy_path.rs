use camino::Utf8Path;

pub struct LazyPath<'a> {
    pub path: &'a Utf8Path,
    metadata: Option<std::io::Result<std::fs::Metadata>>,
}

impl<'a> LazyPath<'a> {
    pub fn new(path: &'a (impl AsRef<str> + ?Sized)) -> Self {
        LazyPath {
            path: Utf8Path::new(path),
            metadata: None,
        }
    }

    pub fn metadata(&mut self) -> &mut std::io::Result<std::fs::Metadata> {
        self.metadata
            .get_or_insert_with(|| self.path.symlink_metadata())
    }

    pub fn into_metadata(mut self) -> std::io::Result<std::fs::Metadata> {
        self.metadata();
        self.metadata.unwrap()
    }
}

impl std::fmt::Display for LazyPath<'_> {
    fn fmt(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.path.fmt(formatter)
    }
}
