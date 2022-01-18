/**
 * 1. Use lib crate in any binary crate: use <package_name> (because the lib crate is named as package name)
 */

use chapter7_package_module_advanced;

fn main() {
    chapter7_package_module_advanced::mother::housekeeping::wash_cloth();
    chapter7_package_module_advanced::start_weekday_life()
}