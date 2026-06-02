//! ฟังก์ชันพื้นฐาน (built-in functions)
//! แต่ละกลุ่มแยกเป็นไฟล์: string, math, logic, date

pub mod collection;
pub mod date;
pub mod higher_order; // Phase 9: Lambda & higher-order functions
pub mod logic;
pub mod math;
pub mod string; // เพิ่ม

use crate::functions::FunctionRegistry;

/// ลงทะเบียน built-in ทั้งหมด (Phase 1 + Phase 6.1)
pub fn register_all(registry: &mut FunctionRegistry) {
    // ฟังก์ชัน string
    registry.register(string::len());
    registry.register(string::upper());
    registry.register(string::lower());
    registry.register(string::contains());
    registry.register(string::starts_with());
    registry.register(string::ends_with());
    registry.register(string::trim());
    registry.register(string::trim_start());
    registry.register(string::trim_end());
    registry.register(string::split());
    registry.register(string::replace());
    registry.register(string::substring());

    // ฟังก์ชัน math (min/max ถูกย้ายไป collection)
    registry.register(math::abs());
    registry.register(math::pi());
    registry.register(math::round());
    registry.register(math::ceil());
    registry.register(math::floor());
    registry.register(math::sqrt());
    registry.register(math::pow());
    registry.register(math::sin());
    registry.register(math::cos());
    registry.register(math::tan());
    registry.register(math::random());

    // ฟังก์ชัน logic
    registry.register(logic::if_fn());

    // ฟังก์ชัน array (Phase 6.1)
    registry.register(collection::sum());
    registry.register(collection::avg());
    registry.register(collection::min_arr());
    registry.register(collection::max_arr());
    registry.register(collection::join());
    registry.register(collection::count());

    // ฟังก์ชัน date (Phase 6.3)
    registry.register(date::now());
    registry.register(date::date_add());
    registry.register(date::date());
    registry.register(date::date_diff());
    registry.register(date::year());
    registry.register(date::month());
    registry.register(date::day());

    // Phase 9: Lambda & Higher-Order Functions
    registry.register(higher_order::map_fn());
    registry.register(higher_order::filter_fn());
    registry.register(higher_order::reduce_fn());
    registry.register(higher_order::sort_fn());
    registry.register(higher_order::sort_with_fn());
    registry.register(higher_order::unique_fn());
    registry.register(higher_order::group_by_fn());

    // Phase 11.6-11.7: set() and range()
    registry.register(higher_order::set_fn());
    registry.register(higher_order::range_fn());
}
