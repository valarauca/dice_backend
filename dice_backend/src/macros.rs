
/// _unreachable_panic is macro intended to take the place of 
/// 'panic!' and 'unreachable!' within the code base. Its job
/// is to hint to the compiler that certain branches are
/// unreachable.
///
/// It does this via the configuration option
/// 'drop_unreachable_branches' configuration option.
///
/// This is wildly unsafe, and will splatter UB throughout
/// the entire codebase, but it is not enabled by default.
#[macro_export]
macro_rules! _unreachable_panic {
    () => {
        if cfg!(feature="drop_unreachable_branches") {
           unsafe { ::std::hint::unreachable_unchecked() }
        } else {
            panic!()
        }
    };
    ($arg: expr) => {
        if cfg!(feature="drop_unreachable_branches") {
           unsafe { ::std::hint::unreachable_unchecked() }
        } else {
            panic!($arg)
        }
    };
    ($arg: expr,) => {
        _unreachable_panic!($arg)
    };
    ($fmt:expr, $($arg:tt)+) => {
        if cfg!(feature="drop_unreachable_branches") {
           unsafe { ::std::hint::unreachable_unchecked() }
        } else {
            panic!($fmt, $($arg)+)
        }
    };
}

