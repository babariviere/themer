#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Entry<T> {
    pub name: String,
    pub value: T,
}

impl<T> Entry<T> {
    pub fn new(name: String, value: T) -> Self {
        Entry { name, value }
    }
}

/// Unoptimised map, do not use on large map
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Map<T>(Vec<Entry<T>>);

impl<T> Map<T> {
    pub fn new() -> Self {
        Map(Vec::new())
    }

    pub fn insert(&mut self, name: String, value: T) {
        self.0.push(Entry::new(name, value));
    }

    pub fn get(&self, name: &str) -> Option<&T> {
        for ref entry in &self.0 {
            if entry.name == name {
                return Some(&entry.value);
            }
        }
        None
    }
}

impl<T> Default for Map<T> {
    fn default() -> Self {
        Map::new()
    }
}

impl<'a, T> IntoIterator for &'a Map<T> {
    type Item = &'a Entry<T>;

    type IntoIter = ::std::slice::Iter<'a, Entry<T>>;

    fn into_iter(self) -> Self::IntoIter {
        self.0.iter()
    }
}
