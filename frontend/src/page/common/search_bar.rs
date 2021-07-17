pub struct SearchBar<T> {
    pub search_query: String,
    pub matched_skills: Vec<T>,
}

impl<T> SearchBar<T> {
    pub fn new() -> Self {
        Self {
            search_query: String::new(),
            matched_skills: Vec::new(),
        }
    }
}