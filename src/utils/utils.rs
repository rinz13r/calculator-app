pub fn add<T>(u: T, v: T) -> T::Output
where
    T: std::ops::Add,
{
    u + v
}

pub fn sub<T>(u: T, v: T) -> T::Output
where
    T: std::ops::Sub,
{
    u - v
}


pub fn mul<T>(u: T, v: T) -> T::Output
where
    T: std::ops::Mul,
{
    u * v
}
