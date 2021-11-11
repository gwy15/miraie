use core::future::Future;

/// `Func` 主要是解决 rust 只提供 `Fn(T) -> O` 不提供 `Fn<((T,), O)>` 的问题
pub trait Func<I, Fut>: Send + Sync + Clone + Copy + 'static
where
    I: Send + 'static,
    Fut: Future + Send,
{
    fn call(&self, input: I) -> Fut;
}

#[rustfmt::skip]
mod _impl_func {
    use super::*;

    macro_rules! f {
        (($($Ts:ident),*), ($($Ns:tt),*)) => {
            impl<F, Fut, $($Ts,)*> Func<( $($Ts, )*), Fut> for F
            where
                F: Fn( $($Ts,)* ) -> Fut + Send + Sync + Clone + Copy + 'static,
                $(
                    $Ts: Send + 'static,
                )*
                Fut: Future + Send
            {
                #[allow(unused)]
                fn call(&self, params: ( $($Ts,)* )) -> Fut {
                    (self)(
                        $(params.$Ns, )*
                    )
                }
            }
        };
    }
    
    f!((), ());
    f!((T1), (0));
    f!((T1, T2), (0, 1));
    f!((T1, T2, T3), (0, 1, 2));
    f!((T1, T2, T3, T4), (0, 1, 2, 3));
    f!((T1, T2, T3, T4, T5), (0, 1, 2, 3, 4));
    f!((T1, T2, T3, T4, T5, T6), (0, 1, 2, 3, 4, 5));
    f!((T1, T2, T3, T4, T5, T6, T7), (0, 1, 2, 3, 4, 5, 6));
    f!((T1, T2, T3, T4, T5, T6, T7, T8), (0, 1, 2, 3, 4, 5, 6, 7));
    f!((T1, T2, T3, T4, T5, T6, T7, T8, T9), (0, 1, 2, 3, 4, 5, 6, 7, 8));
    f!((T1, T2, T3, T4, T5, T6, T7, T8, T9, T10), (0, 1, 2, 3, 4, 5, 6, 7, 8, 9));
}
