use std::fmt::Display;

pub enum GBCSupport {
    None,
    Enhanced,
    Required,
}

impl Display for GBCSupport {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            GBCSupport::None => write!(f, "None"),
            GBCSupport::Enhanced => write!(f, "Enhanced"),
            GBCSupport::Required => write!(f, "Required"),
        }
    }
}
