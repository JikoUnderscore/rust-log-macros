

pub mod log{

    pub const C_INFO: u8 = 0b00000001;
    pub const C_TEST: u8 = 0b00000010;
    pub const C_WARN: u8 = 0b00000100;

    pub static CHECK: u8 = C_INFO | C_TEST | C_WARN;

    macro_rules! INFO {
        ($($arg:tt)*) => (
            {
                use crate::log::{CHECK, C_INFO};
                if &CHECK & C_INFO == C_INFO {
                    println!("[INFO] {}", std::fmt::format(format_args!($($arg)*)));
                }
            }
        )
    }
    macro_rules! TEST {
        ($($arg:tt)*) => (
            {
                use crate::log::{CHECK, C_TEST};
                if &CHECK & C_TEST == C_TEST {
                    println!("[TEST] {}", std::fmt::format(format_args!($($arg)*)));
                }
            }
        )
    }

    macro_rules! WARN {
        ($($arg:tt)*) => (
            {
                use crate::log::{CHECK, C_WARN};
                if &CHECK & C_WARN == C_WARN {
                        eprintln!("[WARN] {}", std::fmt::format(format_args!($($arg)*)));
                }

            }
        )
    }



    pub(crate) use INFO;
    pub(crate) use TEST;
    pub(crate) use WARN;

}
