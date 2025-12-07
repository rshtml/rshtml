// use std::fmt;

// pub struct Block(pub Box<dyn Fn(&mut dyn std::fmt::Write) -> std::fmt::Result>);

// impl fmt::Display for Block {
//     fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
//         (self.0)(f)
//     }
// }

// impl<F> From<F> for Block
// where
//     F: Fn(&mut dyn fmt::Write) -> fmt::Result,
// {
//     fn from(f: F) -> Self {
//         Block(Box::new(f))
//     }
// }

use std::fmt;

// type X = dyn Fn(&mut dyn fmt::Write) -> fmt::Result;
// impl fmt::Display for X {
//     fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
//         (self)(f)
//     }
// }
// 1. Struct'a <'a> ekliyoruz.
// + 'static yerine + 'a diyoruz.
pub struct Block<'a>(pub Box<dyn Fn(&mut dyn fmt::Write) -> fmt::Result + 'a>);

// 2. Display implementasyonu
impl<'a> fmt::Display for Block<'a> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        (self.0)(f)
    }
}

// 3. From Implementasyonu
// Burada 'static YERİNE 'a kullanıyoruz.
impl<'a, F> From<F> for Block<'a>
where
    F: Fn(&mut dyn fmt::Write) -> fmt::Result + 'a,
{
    fn from(f: F) -> Self {
        Block(Box::new(f))
    }
}
