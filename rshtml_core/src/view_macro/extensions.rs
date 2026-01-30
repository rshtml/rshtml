use winnow::{
    Parser,
    error::{AddContext, ParserError, StrContext, StrContextValue},
    stream::Stream,
};

pub trait ParserDiagnostic<I, O, E>: Parser<I, O, E> + Sized
where
    I: Stream,
    E: ParserError<I> + AddContext<I, StrContext>,
{
    fn expected(self, msg: &'static str) -> impl Parser<I, O, E>;
    fn label(self, msg: &'static str) -> impl Parser<I, O, E>;
}

impl<I, O, E, P> ParserDiagnostic<I, O, E> for P
where
    I: Stream,
    P: Parser<I, O, E> + Sized,
    E: ParserError<I> + AddContext<I, StrContext>,
{
    fn expected(self, msg: &'static str) -> impl Parser<I, O, E> {
        self.context(StrContext::Expected(StrContextValue::StringLiteral(msg)))
    }

    fn label(self, msg: &'static str) -> impl Parser<I, O, E> {
        self.context(StrContext::Label(msg))
    }
}
