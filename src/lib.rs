#![deny(unsafe_code)]

pub mod magic;

pub fn libmagic_version() -> libc::c_int {
    crate::magic::version()
}

/// Functionality for [`Cookie`]
pub mod cookie {
    use std::convert::TryFrom;
    use std::ffi::CString;
    use std::path::Path;

    use crate::magic;

    bitflags::bitflags! {
         #[derive(std::default::Default, PartialEq, Eq, PartialOrd, Ord, Hash, Debug, Clone, Copy)]
        pub struct Flags: libc::c_uint {
            const _                 = !0;
            const DEBUG             = magic::MAGIC_DEBUG;
            const SYMLINK           = magic::MAGIC_SYMLINK;
            const COMPRESS          = magic::MAGIC_COMPRESS;
            const DEVICES           = magic::MAGIC_DEVICES;
            const MIME_TYPE         = magic::MAGIC_MIME_TYPE;
            const CONTINUE          = magic::MAGIC_CONTINUE;
            const CHECK             = magic::MAGIC_CHECK;
            const PRESERVE_ATIME    = magic::MAGIC_PRESERVE_ATIME;
            const RAW               = magic::MAGIC_RAW;
            const ERROR             = magic::MAGIC_ERROR;
            const MIME_ENCODING     = magic::MAGIC_MIME_ENCODING;
            const MIME              = Self::MIME_TYPE.bits()
                                    | Self::MIME_ENCODING.bits();
            const APPLE             = magic::MAGIC_APPLE;
            const EXTENSION         = magic::MAGIC_EXTENSION;
            const NODESC            = Self::EXTENSION.bits()
                                    | Self::MIME.bits()
                                    | Self::APPLE.bits();
            const NO_CHECK_COMPRESS = magic::MAGIC_NO_CHECK_COMPRESS;
            const NO_CHECK_TAR      = magic::MAGIC_NO_CHECK_TAR;
            const NO_CHECK_SOFT     = magic::MAGIC_NO_CHECK_SOFT;
            const NO_CHECK_APPTYPE  = magic::MAGIC_NO_CHECK_APPTYPE;
            const NO_CHECK_ELF      = magic::MAGIC_NO_CHECK_ELF;
            const NO_CHECK_TEXT     = magic::MAGIC_NO_CHECK_TEXT;
            const NO_CHECK_CDF      = magic::MAGIC_NO_CHECK_CDF;
            const NO_CHECK_CSV      = magic::MAGIC_NO_CHECK_CSV;
            const NO_CHECK_TOKENS   = magic::MAGIC_NO_CHECK_TOKENS;
            const NO_CHECK_ENCODING = magic::MAGIC_NO_CHECK_ENCODING;
            const NO_CHECK_JSON     = magic::MAGIC_NO_CHECK_JSON;
            const NO_CHECK_BUILTIN  = Self::NO_CHECK_COMPRESS.bits()
                                    | Self::NO_CHECK_TAR.bits()
                                    | Self::NO_CHECK_APPTYPE.bits()
                                    | Self::NO_CHECK_ELF.bits()
                                    | Self::NO_CHECK_TEXT.bits()
                                    | Self::NO_CHECK_CSV.bits()
                                    | Self::NO_CHECK_CDF.bits()
                                    | Self::NO_CHECK_TOKENS.bits()
                                    | Self::NO_CHECK_ENCODING.bits()
                                    | Self::NO_CHECK_JSON.bits();
        }
    }

    impl std::fmt::Display for Flags {
        fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
            bitflags::parser::to_writer(self, f)
        }
    }

    #[derive(thiserror::Error, Debug)]
    #[error("invalid database files path")]
    pub struct InvalidDatabasePathError {}

    pub struct DatabasePaths {
        filenames: Option<CString>,
    }

    #[cfg(target_os = "windows")]
    const DATABASE_FILENAME_SEPARATOR: &str = ";";
    #[cfg(not(target_os = "windows"))]
    const DATABASE_FILENAME_SEPARATOR: &str = ":";

    impl DatabasePaths {
        pub fn new<I, P>(paths: I) -> Result<Self, InvalidDatabasePathError>
        where
            I: IntoIterator<Item = P>,
            P: AsRef<Path>,
        {
            let filename = paths
                .into_iter()
                .map(|f| f.as_ref().to_string_lossy().into_owned())
                .collect::<Vec<String>>()
                .join(DATABASE_FILENAME_SEPARATOR);

            Ok(Self {
                filenames: match filename.is_empty() {
                    true => None,
                    _ => Some(CString::new(filename).map_err(|_| InvalidDatabasePathError {})?),
                },
            })
        }
    }

    impl Default for DatabasePaths {
        fn default() -> Self {
            Self { filenames: None }
        }
    }

    impl<P: AsRef<std::path::Path>, const N: usize> TryFrom<[P; N]> for DatabasePaths {
        type Error = InvalidDatabasePathError;

        /// Invokes [`DatabasePaths::new()`](DatabasePaths::new)
        fn try_from(value: [P; N]) -> Result<Self, <Self as TryFrom<[P; N]>>::Error> {
            Self::new(value)
        }
    }

    impl<P: AsRef<std::path::Path>> TryFrom<Vec<P>> for DatabasePaths {
        type Error = InvalidDatabasePathError;

        /// Invokes [`DatabasePaths::new()`](DatabasePaths::new)
        fn try_from(value: Vec<P>) -> Result<Self, <Self as TryFrom<Vec<P>>>::Error> {
            Self::new(value)
        }
    }

    impl<P: AsRef<std::path::Path>> TryFrom<&'_ [P]> for DatabasePaths {
        type Error = InvalidDatabasePathError;

        /// Invokes [`DatabasePaths::new()`](DatabasePaths::new)
        fn try_from(value: &[P]) -> Result<Self, <Self as TryFrom<&[P]>>::Error> {
            Self::new(value)
        }
    }

    macro_rules! databasepaths_try_from_impl {
        ($t:ty) => {
            impl TryFrom<$t> for DatabasePaths {
                type Error = InvalidDatabasePathError;

                /// Invokes [`DatabasePaths::new()`](DatabasePaths::new)
                fn try_from(value: $t) -> Result<Self, <Self as TryFrom<$t>>::Error> {
                    DatabasePaths::new(std::iter::once(value))
                }
            }
        };
    }

    // missing for now are:
    // - Cow<'_, OsStr>
    // - std::path::Component<'_>
    // - std::path::Components<'_>
    // - std::path::Iter<'_>
    databasepaths_try_from_impl!(&str);
    databasepaths_try_from_impl!(&std::ffi::OsStr);
    databasepaths_try_from_impl!(std::ffi::OsString);
    databasepaths_try_from_impl!(&std::path::Path);
    databasepaths_try_from_impl!(std::path::PathBuf);
    databasepaths_try_from_impl!(String);

    /// Error within several [`Cookie`] functions
    ///
    /// Most functions on a [`Cookie`] can return an error from `libmagic`,
    /// which unfortunately is not very structured.
    #[derive(thiserror::Error, Debug)]
    #[error("magic cookie error in `libmagic` function {}", .function)]
    pub struct Error {
        function: &'static str,
        //#[backtrace]
        source: crate::magic::CookieError,
    }

    #[doc(hidden)]
    #[derive(Debug)]
    pub enum Open {}

    #[doc(hidden)]
    #[derive(Debug)]
    pub enum Load {}

    mod private {
        pub trait Sealed {}

        impl Sealed for super::Open {}
        impl Sealed for super::Load {}
    }

    #[doc(hidden)]
    pub trait State: private::Sealed {}

    impl State for Open {}
    impl State for Load {}

    #[derive(Debug)]
    pub struct Cookie<S: State> {
        cookie: crate::magic::Cookie,
        marker: std::marker::PhantomData<S>,
    }

    #[derive(thiserror::Error, Debug)]
    #[error("magic cookie error in `libmagic` function {}", .function)]
    pub struct LoadError<S: State> {
        function: &'static str,
        //#[backtrace]
        source: crate::magic::CookieError,
        cookie: Cookie<S>,
    }

    impl<S: State> LoadError<S> {
        /// Returns the cookie in its original state
        pub fn cookie(self) -> Cookie<S> {
            self.cookie
        }
    }

    impl<S: State> Drop for Cookie<S> {
        /// Closes the loaded magic database files and deallocates any resources used
        #[doc(alias = "magic_close")]
        fn drop(&mut self) {
            crate::magic::close(&mut self.cookie);
        }
    }

    /// Operations that are valid in the `Open` state
    ///
    /// A new cookie created with [`Cookie::open`](Cookie::open) does not have any databases [loaded](Cookie::load).
    impl Cookie<Open> {
        pub fn open(flags: Flags) -> Result<Cookie<Open>, OpenError> {
            match crate::magic::open(flags.bits() as _) {
                Err(err) => Err(OpenError {
                    flags,
                    kind: match err.errno().kind() {
                        std::io::ErrorKind::InvalidInput => OpenErrorKind::UnsupportedFlags,
                        _ => OpenErrorKind::Errno,
                    },
                    source: err,
                }),
                Ok(cookie) => {
                    let cookie = Cookie {
                        cookie,
                        marker: std::marker::PhantomData,
                    };
                    Ok(cookie)
                }
            }
        }
    }

    impl Cookie<Load> {
        pub fn file<P: AsRef<Path>>(&self, filename: P) -> Result<String, Error> {
            let c_string = CString::new(filename.as_ref().to_string_lossy().into_owned()).unwrap();
            match crate::magic::file(&self.cookie, c_string.as_c_str()) {
                Ok(res) => Ok(res.to_string_lossy().to_string()),
                Err(err) => Err(Error {
                    function: "magic_file",
                    source: err,
                }),
            }
        }
        pub fn buffer(&self, buffer: &[u8]) -> Result<String, Error> {
            match crate::magic::buffer(&self.cookie, buffer) {
                Ok(res) => Ok(res.to_string_lossy().to_string()),
                Err(err) => Err(Error {
                    function: "magic_buffer",
                    source: err,
                }),
            }
        }
    }

    /// Operations that are valid in any state
    impl<S: State> Cookie<S> {
        pub fn load(self, filenames: &DatabasePaths) -> Result<Cookie<Load>, LoadError<S>> {
            match crate::magic::load(&self.cookie, filenames.filenames.as_deref()) {
                Err(err) => Err(LoadError {
                    function: "magic_load",
                    source: err,
                    cookie: self,
                }),
                Ok(_) => {
                    let mut cookie = std::mem::ManuallyDrop::new(self);

                    let cookie = Cookie {
                        cookie: crate::magic::Cookie::new(&mut cookie.cookie),
                        marker: std::marker::PhantomData,
                    };
                    Ok(cookie)
                }
            }
        }
        pub fn load_buffers(self, buffers: &[&[u8]]) -> Result<Cookie<Load>, LoadError<S>> {
            match crate::magic::load_buffers(&self.cookie, buffers) {
                Err(err) => Err(LoadError {
                    function: "magic_load_buffers",
                    source: err,
                    cookie: self,
                }),
                Ok(_) => {
                    let mut cookie = std::mem::ManuallyDrop::new(self);

                    let cookie = Cookie {
                        cookie: crate::magic::Cookie::new(&mut cookie.cookie),
                        marker: std::marker::PhantomData,
                    };
                    Ok(cookie)
                }
            }
        }
        pub fn set_flags(&self, flags: Flags) -> Result<(), SetFlagsError> {
            let ret = crate::magic::setflags(&self.cookie, flags.bits() as _);
            match ret {
                // according to `libmagic` man page this is the only flag that could be unsupported
                Err(err) => Err(SetFlagsError {
                    flags: Flags::PRESERVE_ATIME,
                    source: err,
                }),
                Ok(_) => Ok(()),
            }
        }

        pub fn compile(&self, filenames: &DatabasePaths) -> Result<(), Error> {
            match crate::magic::compile(&self.cookie, filenames.filenames.as_deref()) {
                Err(err) => Err(Error {
                    function: "magic_compile",
                    source: err,
                }),
                Ok(_) => Ok(()),
            }
        }

        pub fn check(&self, filenames: &DatabasePaths) -> Result<(), Error> {
            match crate::magic::check(&self.cookie, filenames.filenames.as_deref()) {
                Err(err) => Err(Error {
                    function: "magic_check",
                    source: err,
                }),
                Ok(_) => Ok(()),
            }
        }

        pub fn list(&self, filenames: &DatabasePaths) -> Result<(), Error> {
            match crate::magic::list(&self.cookie, filenames.filenames.as_deref()) {
                Err(err) => Err(Error {
                    function: "magic_list",
                    source: err,
                }),
                Ok(_) => Ok(()),
            }
        }
    }

    /// Error within [`Cookie::open()`](Cookie::open)
    ///
    /// Note that a similar [`cookie::SetFlagsError`](SetFlagsError) can also occur
    #[derive(thiserror::Error, Debug)]
    #[error("could not open magic cookie: {}",
        match .kind {
            OpenErrorKind::UnsupportedFlags => format!("unsupported flags {}", .flags),
            OpenErrorKind::Errno => "other error".to_string(),
        }
    )]
    pub struct OpenError {
        flags: Flags,
        kind: OpenErrorKind,
        //#[backtrace]
        source: crate::magic::OpenError,
    }

    /// Kind of [`OpenError`]
    #[derive(Debug)]
    enum OpenErrorKind {
        /// Unsupported flags given
        UnsupportedFlags,
        /// Other kind
        Errno,
    }

    /// Error within [`Cookie::set_flags()`](Cookie::set_flags)
    ///
    /// Note that a similar [`cookie::OpenError`](OpenError) can also occur
    #[derive(thiserror::Error, Debug)]
    #[error("could not set magic cookie flags {}", .flags)]
    pub struct SetFlagsError {
        flags: Flags,
        //#[backtrace]
        source: crate::magic::SetFlagsError,
    }
} // mod cookie

pub use crate::cookie::Cookie;
