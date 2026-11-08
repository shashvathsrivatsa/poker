pub trait Percent {
    fn percent(&self) -> String;
}

impl Percent for f64 {
    fn percent(&self) -> String {
        format!("{:.2?}%", self * 100.0)
    }
}

