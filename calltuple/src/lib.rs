pub trait CallTuple<Args> {
    type Output;
    fn call_tuple(&self, args: Args) -> Self::Output;
}

macro_rules! fn_ext_impl {

    ($($A:ident)*) => {
        impl<F: ?Sized, $($A,)* O> CallTuple<($($A,)*)> for F
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

fn_ext_impl!();
fn_ext_impl!(A);
fn_ext_impl!(A B);
fn_ext_impl!(A B C);
fn_ext_impl!(A B C D);
fn_ext_impl!(A B C D E);
fn_ext_impl!(A B C D E G);
fn_ext_impl!(A B C D E G H);
fn_ext_impl!(A B C D E G H I);
fn_ext_impl!(A B C D E G H I J);
fn_ext_impl!(A B C D E G H I J K);
fn_ext_impl!(A B C D E G H I J K L);
fn_ext_impl!(A B C D E G H I J K L M);
fn_ext_impl!(A B C D E G H I J K L M N);
fn_ext_impl!(A B C D E G H I J K L M N P);
fn_ext_impl!(A B C D E G H I J K L M N P Q);
fn_ext_impl!(A B C D E G H I J K L M N P Q R);
fn_ext_impl!(A B C D E G H I J K L M N P Q R S);
fn_ext_impl!(A B C D E G H I J K L M N P Q R S T);
fn_ext_impl!(A B C D E G H I J K L M N P Q R S T U);
fn_ext_impl!(A B C D E G H I J K L M N P Q R S T U V);
fn_ext_impl!(A B C D E G H I J K L M N P Q R S T U V W);
fn_ext_impl!(A B C D E G H I J K L M N P Q R S T U V W X);
fn_ext_impl!(A B C D E G H I J K L M N P Q R S T U V W X Y);
fn_ext_impl!(A B C D E G H I J K L M N P Q R S T U V W X Y Z);
