use super::App;

#[derive(Clone)]
pub struct Request<A>
where
    A: App,
{
    pub app: A,
    pub message: A::Message,
}

pub trait FromRequest<A>: Sized
where
    A: App,
{
    fn from_request(request: Request<A>) -> Option<Self>;
}

mod _impl_from_request {
    use super::*;

    impl<A, T> FromRequest<A> for (T,)
    where
        A: App,
        T: FromRequest<A>,
    {
        fn from_request(request: Request<A>) -> Option<Self> {
            let r = T::from_request(request)?;
            Some((r,))
        }
    }

    macro_rules! f {
        (($($Ts:ident),*)) => {
            impl<A, $($Ts,)*> FromRequest<A> for ($($Ts,)*)
            where
                A: App,
                $(
                    $Ts: FromRequest<A>,
                )*
            {
                fn from_request(request: Request<A>) -> Option<Self> {
                    Some((
                        $(
                            $Ts::from_request(request.clone())?
                        ),*
                    ))
                }
            }
        };
    }

    f!((T1, T2));
    f!((T1, T2, T3));
    f!((T1, T2, T3, T4));
    f!((T1, T2, T3, T4, T5));
    f!((T1, T2, T3, T4, T5, T6));
    f!((T1, T2, T3, T4, T5, T6, T7));
    f!((T1, T2, T3, T4, T5, T6, T7, T8));
    f!((T1, T2, T3, T4, T5, T6, T7, T8, T9));
    f!((T1, T2, T3, T4, T5, T6, T7, T8, T9, T10));
}
