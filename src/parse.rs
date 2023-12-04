pub (in crate) mod fetch;
pub (in crate) mod text;
mod innerhtml;
mod tagnames;

pub trait Args {
    fn extract(self) -> (&'static str, &'static str, &'static str);
}


impl Args for &'static str {
    fn extract(self) -> (&'static str, &'static str, &'static str) {
        (self, "", "")
    }
}

impl Args for (&'static str, &'static str) {
    fn extract(self) -> (&'static str, &'static str, &'static str) {
        (self.0, self.1, "")
    }
}

impl Args for (&'static str, &'static str, &'static str) {
    fn extract(self) -> (&'static str, &'static str, &'static str) {
        (self.0, self.1, self.2)
    }
}