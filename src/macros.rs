macro_rules! impl_received_invalid {
    ($Error: ident, $Name: ident) => {
        #[derive(Debug, Copy, Clone)]
        pub struct $Error;

        impl std::fmt::Display for $Error {
            fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
                write!(
                    f,
                    concat!(
                        "The OpenGL driver returned an invalid ",
                        stringify!($Name),
                        "."
                    )
                )
            }
        }

        impl std::error::Error for $Error {
            fn description(&self) -> &'static str {
                concat!(
                    "The OpenGL driver returned an invalid ",
                    stringify!($Name),
                    "."
                )
            }
        }
    };
    ($Error: ident($Inner: ident), $Name: ident) => {
        #[derive(Debug, Copy, Clone)]
        pub struct $Error($Inner);

        impl std::fmt::Display for $Error {
            fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
                write!(
                    f,
                    concat!(
                        "The OpenGL driver returned an invalid ",
                        stringify!($Name),
                        ": {:?}."
                    ),
                    &self.0
                )
            }
        }

        impl std::error::Error for $Error {
            fn description(&self) -> &'static str {
                concat!(
                    "The OpenGL driver returned an invalid ",
                    stringify!($Name),
                    "."
                )
            }
        }
    };
}
