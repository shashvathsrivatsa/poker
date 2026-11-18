pub trait Percent {
    fn percent(&self) -> String;
}

impl Percent for f64 {
    fn percent(&self) -> String {
        format!("{:?}%", self * 100.0)
    }
}

