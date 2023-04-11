pub trait TupleCaller<Args> {
    type Output;
    fn call_tuple(&self, args: Args) -> Self::Output;
}

macro_rules! tuple_caller_impl {

    ($($A:ident)*) => {
        impl<F: ?Sized, $($A,)* O> TupleCaller<($($A,)*)> for F
        where
            F: Fn($($A),*) -> O
        {
            type Output = O;
            #[allow(non_snake_case)]
            fn call_tuple(&self, ($($A,)*): ($($A,)*)) -> O {
                self($($A,)*)
            }
        }
    }
}

tuple_caller_impl!();
tuple_caller_impl!(A);
tuple_caller_impl!(A B);
tuple_caller_impl!(A B C);
tuple_caller_impl!(A B C D);
tuple_caller_impl!(A B C D E);
tuple_caller_impl!(A B C D E G);
tuple_caller_impl!(A B C D E G H);
tuple_caller_impl!(A B C D E G H I);
tuple_caller_impl!(A B C D E G H I J);
tuple_caller_impl!(A B C D E G H I J K);
tuple_caller_impl!(A B C D E G H I J K L);
tuple_caller_impl!(A B C D E G H I J K L M);
tuple_caller_impl!(A B C D E G H I J K L M N);
tuple_caller_impl!(A B C D E G H I J K L M N P);
tuple_caller_impl!(A B C D E G H I J K L M N P Q);
tuple_caller_impl!(A B C D E G H I J K L M N P Q R);
tuple_caller_impl!(A B C D E G H I J K L M N P Q R S);
tuple_caller_impl!(A B C D E G H I J K L M N P Q R S T);
tuple_caller_impl!(A B C D E G H I J K L M N P Q R S T U);
tuple_caller_impl!(A B C D E G H I J K L M N P Q R S T U V);
tuple_caller_impl!(A B C D E G H I J K L M N P Q R S T U V W);
tuple_caller_impl!(A B C D E G H I J K L M N P Q R S T U V W X);
tuple_caller_impl!(A B C D E G H I J K L M N P Q R S T U V W X Y);
tuple_caller_impl!(A B C D E G H I J K L M N P Q R S T U V W X Y Z);
