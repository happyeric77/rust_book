
pub mod father {
    pub mod make_money {
        pub fn go_company() {}
        fn biz_trip() {}
        fn take_day_off() {}
    }
}


pub mod mother {
    mod care_children {
        fn bring_to_school(){}
    }
    pub mod housekeeping {
        pub fn wash_cloth(){print!("{}",String::from("cloths washed\n"))}
    }
}

use crate::mother::housekeeping;

pub fn start_weekday_life() {
    // Ref1: absolute path to call a module function
    crate::mother::housekeeping::wash_cloth();
    // Ref2: relative path to call a module function
    father::make_money::go_company();

    // Ref3: Use keyword
    housekeeping::wash_cloth();
}