use crate::date::Date;
use crate::retirement::date::Date;
use crate::retirement::types::PersonnelCategory;
use crate::rules::RetirementRules;
use crate::types::PersonnelCategory;

/// 《 全国人民代表大会常务委员会关于实施渐进式延迟法定退休年龄的决定 》
/// https://www.gov.cn/yaowen/liebiao/202409/content_6974294.htm
/// - 男职工 和 原法定退休年龄为五十五周岁的女职工，
///     - 法定退休年龄**每四个月延迟一个月**，分别逐步**延迟至六十三周岁和五十八周岁**；
/// - 原法定退休年龄为五十周岁的女职工，
///     - 法定退休年龄**每二个月延迟一个月**，**逐步延迟至五十五周岁**。
pub struct Rules20240913;


const MAN_DELAY_MAX_LIMIT: isize = 63;
const FEMALE_WORKERS_DELAY_MAX_LIMIT: isize = 55;
const FEMALE_CADRES_DELAY_MAX_LIMIT: isize = 58;

const MAN_DELAY_START: isize = 1965;
const MAN_DELAY_END: isize = 1977;
const FEMALE_WORKERS_DELAY_START: isize = 1975;
const FEMALE_WORKERS_DELAY_END: isize = 1985;
const FEMALE_CADRES_DELAY_START: isize = 1970;
const FEMALE_CADRES_DELAY_END: isize = 1982;

impl RetirementRules for Rules20240913 {
    fn calculate_working_date(&self, date: &Date, types: &PersonnelCategory) -> Option<Date> {
        match types {
            PersonnelCategory::Man => {
                if date.year < MAN_DELAY_START { return None}
                if date.year >= MAN_DELAY_END { return Some(Date::new_abs(3, 0))}

                let diff = difference_between_birth_time_and_calibration_time(date, MAN_DELAY_START);
                Some(Date::new_abs(0, (diff / 4) as isize + 1))
            },
            PersonnelCategory::FemaleCadres => {
                if date.year < FEMALE_CADRES_DELAY_START { return None}
                if date.year >= FEMALE_CADRES_DELAY_END { return Some(Date::new_abs(3, 0))}

                let diff = difference_between_birth_time_and_calibration_time(date, FEMALE_CADRES_DELAY_START);
                Some(Date::new_abs(0, (diff / 4) as isize + 1))
            },
            PersonnelCategory::FemaleWorkers => {
                if date.year < FEMALE_WORKERS_DELAY_START { return None}
                if date.year >= FEMALE_WORKERS_DELAY_END { return Some(Date::new_abs(5, 0))}

                let diff = difference_between_birth_time_and_calibration_time(date, FEMALE_WORKERS_DELAY_START);
                Some(Date::new_abs(0, (diff / 2) as isize + 1))
            },
        }
    }
}

/// 出生日期距离标定日期差距的月份数量
/// 返回： 差距的月份数量
fn difference_between_birth_time_and_calibration_time(birth_time: &Date, year: isize) -> usize {
    if birth_time.year < year { panic!("input error in rules calculation") }
    ((birth_time.year - year) * 12 + birth_time.month as isize) as usize - 1
}