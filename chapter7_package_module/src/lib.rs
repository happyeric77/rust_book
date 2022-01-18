/**
 * 1. Simple package has one lib crate 
 *      - mother module includes 1. care children module & 2. housekeeping module
 *      - father module includes 1. make money module 
 * 2. On the Crate level, there is a general level level function called "start_weekday_life" which makes call both mother module methods and father module methods
 *      - Absolute & relative path to call module function: Ref1 & Ref2
 *      - Make public access of module or module method using pub keyword
 */

pub mod mother {
    mod care_children {
        fn bring_to_school(){}
    }
    pub mod housekeeping {
        pub fn wash_cloth(){}
    }
}

pub mod father {
    pub mod make_money {
        pub fn go_company() {}
        fn biz_trip() {}
        fn take_day_off() {}
    }
}

fn start_weekday_life() {
    // Ref1: absolute path to call a module function
    crate::mother::housekeeping::wash_cloth();
    // Ref2: relative path to call a module function
    father::make_money::go_company();
}