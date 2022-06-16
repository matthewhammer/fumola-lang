pub type OurResult<X> = Result<X, OurError>;

#[derive(Debug, Clone)]
pub enum OurError {
    String(String),
}
