//! 基于 RetirementRules 规约实现的针对于 国发【1978】104 号
//! 《國務院關於安置老弱病殘幹部的暫行辦法》
//! 《国务院关于工人退休、退职的暂行办法》的实现
//!
//! # 参考文献:
//! - http://www.gd.gov.cn/zwgk/wjk/zcfgk/content/post_2531473.html
//!
//! # 主要特点:
//! - 男性年满 60 周岁
//! - 女性(党政机关、群众团体、企业、事业单位的干部) 年满 55 周岁
//! - 女性(工人) 年满 50 周岁

use crate::retirement::date::Date;
use crate::retirement::rules::RetirementRules;
use crate::retirement::types::PersonnelCategory;

pub struct Rules1978;

const MAN_DELAY_MAX_LIMIT: isize = 60;
const FEMALE_WORKERS_DELAY_MAX_LIMIT: isize = 50;
const FEMALE_CADRES_DELAY_MAX_LIMIT: isize = 55;

impl RetirementRules for Rules1978 {
    fn calculate_working_date(&self, date: &Date, types: &PersonnelCategory) -> Option<Date>  {
        match types {
            PersonnelCategory::FemaleCadres => Some(Date::new_abs(FEMALE_CADRES_DELAY_MAX_LIMIT, 0)),
            PersonnelCategory::FemaleWorkers => Some(Date::new_abs(FEMALE_WORKERS_DELAY_MAX_LIMIT, 0)),
            PersonnelCategory::Man => Some(Date::new_abs(MAN_DELAY_MAX_LIMIT, 0)),
        }
    }
}
