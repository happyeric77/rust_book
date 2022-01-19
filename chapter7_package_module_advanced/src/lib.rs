
/* OVERVIEW
 * 1. ONE LEVEL single page module migration to different rust file: Ref1: fahter module one level migration
 * 2. TWO OR MORE LEVEL single page module migration to different rust folders and files.
 */ 



/****************************************************************************************************\
 * Ref1: fahter module one level migration                                                          **
 *      - Step1: comment out module content. only leave module declaration here                     **
 *      - Step2: Create a new rust file called father.rs (need to match the declared name here).    **
 *      - Step3: Copy the module content and past into father.rs                                    **
 ****************************************************************************************************/
pub mod father;
// {
//     pub mod make_money {
//         pub fn go_company() {}
//         fn biz_trip() {}
//         fn take_day_off() {}
//     }
// }



/************************************************************************************************************\
 *  Ref2: Mother module two level migration                                                                 **
 *      - Step1: comment out module content. only leave module declaration here                             **
 *      - Step3: Copy the module content and past into mother.rs                                            **
 ******* Until now the process is the same as level one father process. Now we further breakdown mother.rs ***
 *      - Step4 ~: See comment in mother.rs                                                                 **
 ************************************************************************************************************/ 
pub mod mother; 
// {
//     mod care_children {
//         fn bring_to_school(){}
//     }
//     pub mod housekeeping {
//         pub fn wash_cloth(){print!("{}",String::from("cloths washed\n"))}
//     }
// }

use crate::mother::housekeeping;

pub fn start_weekday_life() {
    // Call using absolute path to call a module function
    crate::mother::housekeeping::wash_cloth();
    // Call using relative path to call a module function
    father::make_money::go_company();

    // Call using Use keyword
    housekeeping::wash_cloth();
}