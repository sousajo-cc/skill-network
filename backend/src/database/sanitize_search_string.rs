//There must be a better way to do this, but I don't feel like googling now

pub trait Sanitize {
    fn sanitize_char(&self, c: char) -> Self;
    fn sanitize(&self) -> Self;
}

impl Sanitize for String {

    fn sanitize_char(&self, c: char) -> Self {
        self.replace(c, &format!("\\{}", c))
    }

    fn sanitize(&self) -> Self {
        self
            .sanitize_char('%')
            .sanitize_char('_')
            .sanitize_char('*')
            .sanitize_char('?')
            .sanitize_char('[')
            .sanitize_char(']')
            .sanitize_char('!')
            .sanitize_char('-')
            .sanitize_char('#')
            .sanitize_char('^')
    }
}