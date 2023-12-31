pub use crate::impl_serde_display_fromstr;

/// Simply implements `Serialize` and `Deserialize` traits for types with `Display` and `FromStr` impls.
#[macro_export]
macro_rules! impl_serde_display_fromstr {
    ($($comp:ty),+) => {
        $(
            impl ::serde::Serialize for $comp {
                fn serialize<S: ::serde::Serializer >(&self, serializer: S) -> Result<S::Ok, S::Error> {
                    serializer.collect_str(self)
                }
            }

            impl<'de> ::serde::Deserialize<'de> for $comp {
                fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
                    where D: ::serde::Deserializer<'de>
                {
                    let s = String::deserialize(deserializer)?;
                    ::std::str::FromStr::from_str(&s).map_err(::serde::de::Error::custom)
                }
            }
        )+
    };
}