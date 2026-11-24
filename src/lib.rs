#![doc = include_str!("../README.md")]
#[macro_export]
macro_rules! serde_repr {
    (
        #[repr($repr:ty)]
        $(#[$meta:meta])*
        $vis:vis enum $ident:ident {
            $(
                $(#[$vmeta:meta])*
                $variant:ident $( = $lit:literal)?
            ),*  $(,)?
        }
    ) => {

        #[repr($repr)]
        $(#[$meta])*
        $vis enum $ident {
            $($(#[$vmeta])* $variant $(= $lit)?),*
        }

        #[automatically_derived]
        impl $crate::r#priv::Serialize for $ident {
            fn serialize<S>(&self, serializer: S) -> $crate::r#priv::Result<S::Ok, S::Error>
            where
                S: $crate::r#priv::Serializer
            {
                let value = match self {
                    $($ident::$variant => $ident::$variant as $repr),*
                };
                $crate::r#priv::Serialize::serialize(&value, serializer)
            }
        }

        #[automatically_derived]
        impl<'de> $crate::r#priv::Deserialize<'de> for $ident {
            fn deserialize<D>(deserializer: D) -> $crate::r#priv::Result<Self, D::Error>
            where
                D: $crate::r#priv::Deserializer<'de>
            {
                struct _Discriminant;

                #[allow(non_upper_case_globals)]
                impl _Discriminant {
                    $(const $variant: $repr = $ident::$variant as _;)*
                }

                const _VARIANTS: &[&dyn $crate::r#priv::fmt::Display] = &[
                    $(&_Discriminant::$variant),*
                ];

                match <$repr as $crate::r#priv::Deserialize>::deserialize(deserializer)? {
                    $(_Discriminant::$variant => $crate::r#priv::Result::Ok($ident::$variant),)*
                    other => $crate::r#priv::Result::Err(
                        $crate::r#priv::Error::custom($crate::r#priv::ErrorFormat(&other, _VARIANTS))
                    )
                }
            }
        }
    };
}

#[doc(hidden)]
pub mod r#priv {
    pub use core::fmt;
    pub use core::result::Result;

    pub use serde_core::de::Error;
    pub use serde_core::{Deserialize, Deserializer, Serialize, Serializer};

    pub struct ErrorFormat<'a>(
        pub &'a (dyn fmt::Display + 'a),
        pub &'a [&'a (dyn fmt::Display + 'a)],
    );

    impl fmt::Display for ErrorFormat<'_> {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            let ErrorFormat(other, variants) = self;
            match variants {
                [] => write!(f, "invalid value: {other}"),
                [variant] => {
                    write!(f, "invalid value {other}, expected {variant}")
                }
                [variant1, variant2] => write!(
                    f,
                    "invalid value {other}, expected {variant1} or {variant2}"
                ),
                [variant1, rest @ ..] => {
                    write!(f, "invalid value {other}, expected one of {variant1}")?;
                    for variant in rest {
                        write!(f, ", {variant}")?;
                    }
                    Ok(())
                }
            }
        }
    }
}
