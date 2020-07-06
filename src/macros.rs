/// Build a `Value` conveniently.
///
/// The syntax should be intuitive if you are familiar with JSON. You can also
/// inline simple Rust expressions, including custom values that implement
/// `serde::Serialize`. Note that this macro returns `Result<Value, Error>`,
/// so you should handle the error appropriately.
///
/// ```
/// use serde_cbor::cbor;
///
/// let bytes = serde_bytes::Bytes::new(b"\x00\x01\x02");
/// let value = cbor!({
///     "code" => 415,
///     "message" => null,
///     "continue" => false,
///     "extra" => { "numbers" => [8.2341e+4, 0.251425] },
///     "data" => bytes,
/// }).unwrap();
/// ```

#[macro_export]
macro_rules! cbor {
    (@map {$($key:expr => $val:expr),*} $(,)?) => {{
        #[allow(unused_mut)]
        let mut map = ::std::collections::BTreeMap::new();

        $(
            map.insert(
                $crate::value::to_value(cbor!( $key )?)?,
                $crate::value::to_value(cbor!( $val )?)?,
            );
        )*

        $crate::Value::Map(map)
    }};

    (@map {$($key:expr => $val:expr),*} { $($nkey:tt)* } => $($next:tt)*) => {
        cbor!(
            @map
            { $($key => $val),* }
            cbor!({ $($nkey)* })? =>
            $($next)*
        )
    };

    (@map {$($key:expr => $val:expr),*} [ $($nkey:tt)* ] => $($next:tt)*) => {
        cbor!(
            @map
            { $($key => $val),* }
            cbor!([ $($nkey)* ])? =>
            $($next)*
        )
    };

    (@map {$($key:expr => $val:expr),*} $nkey:expr => { $($nval:tt)* }, $($next:tt)*) => {
        cbor!(
            @map
            { $($key => $val,)* $nkey => cbor!({ $($nval)* })? }
            $($next)*
        )
    };

    (@map {$($key:expr => $val:expr),*} $nkey:expr => [ $($nval:tt)* ], $($next:tt)*) => {
        cbor!(
            @map
            { $($key => $val,)* $nkey => cbor!([ $($nval)* ])? }
            $($next)*
        )
    };

    (@map {$($key:expr => $val:expr),*} $nkey:expr => $nval:expr, $($next:tt)*) => {
        cbor!(
            @map
            { $($key => $val,)* $nkey => cbor!($nval)? }
            $($next)*
        )
    };

    (@array [$($val:expr),*] $(,)?) => {
        $crate::Value::Array(
            vec![$( cbor!($val)? ),*]
        )
    };

    (@array [$($val:expr),*] { $($item:tt)* }, $($next:tt)*) => {
        cbor!(
            @array
            [ $($val,)* cbor!({ $($item)* })? ]
            $($next)*
        )
    };

    (@array [$($val:expr),*] [ $($item:tt)* ], $($next:tt)*) => {
        cbor!(
            @array
            [ $($val,)* cbor!([ $($item)* ])? ]
            $($next)*
        )
    };

    (@array [$($val:expr),*] $item:expr, $($next:tt)*) => {
        cbor!(
            @array
            [ $($val,)* $item ]
            $($next)*
        )
    };

    ({ $($next:tt)* }) => {(||{
        ::core::result::Result::<_, $crate::Error>::from(Ok(cbor!(@map {} $($next)* ,)))
    })()};

    ([ $($next:tt)* ]) => {(||{
        ::core::result::Result::<_, $crate::Error>::from(Ok(cbor!(@array [] $($next)* ,)))
    })()};

    ($val:expr) => {{
        #[allow(unused_imports)]
        use $crate::Value::Null as null;
        $crate::value::to_value(&$val)
    }};
}
