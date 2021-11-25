mod req;
mod resp;

pub trait TryFromHttp<T>: Sized {
    fn try_from_http(_: T) -> anyhow::Result<Self>;
}

pub trait FromHttp<T>: Sized {
    fn from_http(_: T) -> Self;
}

impl<T, U> FromHttp<T> for U where U: TryFromHttp<T> {
    fn from_http(from: T) -> Self {
        U::try_from_http(from).unwrap()
    }
}

pub trait TryIntoHttp<T>: Sized {
    fn try_into_http(self) -> anyhow::Result<T>;
}

impl<T, U> TryIntoHttp<U> for T where U: TryFromHttp<T>, {
    fn try_into_http(self) -> anyhow::Result<U> {
        U::try_from_http(self)
    }
}

pub trait IntoHttp<T>: Sized {
    fn into_http(self) -> T;
}

impl<T, U> IntoHttp<U> for T where U: FromHttp<T>, {
    fn into_http(self) -> U {
        U::from_http(self)
    }
}