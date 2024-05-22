use core::fmt::{self, Debug, Display};

#[derive(Clone, PartialEq, Eq)]
pub struct DisplayError<T>(pub T);

impl<T> Debug for DisplayError<T>
where
    T: Debug,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.0.fmt(f)
    }
}

impl<T> Display for DisplayError<T>
where
    T: Display,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.0.fmt(f)
    }
}

impl<T> snafu::Error for DisplayError<T> where T: Display + Debug {}
