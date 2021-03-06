pub use self::arch::{FLT_RADIX};
pub use self::arch::{FLT_MANT_DIG};
pub use self::arch::{DBL_MANT_DIG};
pub use self::arch::{LDBL_MANT_DIG};
pub use self::arch::{FLT_DIG};
pub use self::arch::{DBL_DIG};
pub use self::arch::{LDBL_DIG};
pub use self::arch::{FLT_MIN_EXP};
pub use self::arch::{DBL_MIN_EXP};
pub use self::arch::{LDBL_MIN_EXP};
pub use self::arch::{FLT_MIN_10_EXP};
pub use self::arch::{DBL_MIN_10_EXP};
pub use self::arch::{LDBL_MIN_10_EXP};
pub use self::arch::{FLT_MAX_EXP};
pub use self::arch::{DBL_MAX_EXP};
pub use self::arch::{LDBL_MAX_EXP};
pub use self::arch::{FLT_MAX_10_EXP};
pub use self::arch::{DBL_MAX_10_EXP};
pub use self::arch::{LDBL_MAX_10_EXP};
pub use self::arch::{FLT_MAX};
pub use self::arch::{DBL_MAX};
pub use self::arch::{LDBL_MAX};
pub use self::arch::{FLT_EPSILON};
pub use self::arch::{DBL_EPSILON};
pub use self::arch::{LDBL_EPSILON};
pub use self::arch::{FLT_MIN};
pub use self::arch::{DBL_MIN};
pub use self::arch::{LDBL_MIN};

#[cfg(target_arch = "x86_64")]
#[path = "x86_64.rs"]
mod arch;

#[cfg(target_arch = "x86")]
#[path = "x86.rs"]
mod arch;

