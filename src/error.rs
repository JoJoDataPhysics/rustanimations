use std::fmt;

#[derive(Debug)]
pub enum AnimationError {
    InvalidRadius(f32),
    InvalidDistance(f32),
    ChainTooShort(usize),
    MathError(String),
}

impl fmt::Display for AnimationError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            AnimationError::InvalidRadius(r) => write!(f, "Invalid radius: {}", r),
            AnimationError::InvalidDistance(d) => write!(f, "Invalid distance: {}", d),
            AnimationError::ChainTooShort(len) => write!(f, "Chain too short: {} nodes", len),
            AnimationError::MathError(msg) => write!(f, "Math error: {}", msg),
        }
    }
}

impl std::error::Error for AnimationError {}

pub type AnimationResult<T> = Result<T, AnimationError>;
