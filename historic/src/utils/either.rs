macro_rules! either {
    ($name:ident( $fst:ident, $($variant:ident),* )) => {
        #[derive(Clone, Copy, Debug)]
        pub enum $name<$fst, $($variant),*> {
            $fst($fst),
            $($variant($variant),)*
        }

        impl<$fst, $($variant),*> Iterator for $name<$fst, $($variant),*>
        where
            $fst: Iterator,
            $($variant: Iterator<Item = $fst::Item>,)*
        {
            type Item = $fst::Item;

            fn next(&mut self) -> Option<Self::Item> {
                match self {
                    $name::$fst(inner) => inner.next(),
                    $( $name::$variant(inner) => inner.next(), )*
                }
            }
        }

        impl <$fst, $($variant),*> futures::stream::Stream for $name<$fst, $($variant),*>
        where
            $fst: futures::stream::Stream,
            $($variant: futures::stream::Stream<Item = $fst::Item>,)*
        {
            type Item = $fst::Item;

            fn poll_next(
                self: std::pin::Pin<&mut Self>,
                cx: &mut std::task::Context<'_>,
            ) -> std::task::Poll<Option<Self::Item>> {
                use std::pin::Pin;

                // SAFETY: This is safe because we never move the inner value out of the Pin.
                unsafe {
                    match self.get_unchecked_mut() {
                        $name::$fst(inner) => Pin::new_unchecked(inner).poll_next(cx),
                        $( $name::$variant(inner) => Pin::new_unchecked(inner).poll_next(cx), )*
                    }
                }
            }
        }
    }
}

either!(Either(A, B));
