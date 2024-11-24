use crate::date::Date;
use crate::types::PersonnelCategory;

mod rules_20240913;
pub use rules_20240913::Rules20240913;
mod rules_1978;
pub use rules_1978::Rules1978;

pub trait RetirementRules {
    fn calculate_working_date(&self, date: &Date, types: &PersonnelCategory) -> Option<Date>; // working date
}

// 组合结构体，持有多个规则
pub struct CombinedRules {
    pub(crate) rules: Vec<Box<dyn RetirementRules>>,
}

impl RetirementRules for CombinedRules {
    fn calculate_working_date(&self, date: &Date, types: &PersonnelCategory) -> Option<Date> {
        let mut total_working_date: Option<Date> = None;
        for rule in &self.rules {
            if let Some(date) = rule.calculate_working_date(date, types) {
                if total_working_date.is_none() {
                    total_working_date = Some(date);
                } else {
                    total_working_date = Some(total_working_date.unwrap() + date);
                }
            }
        }
        if let Some(work_date) = total_working_date {
            Some(work_date + *date)
        } else {
            None
        }
    }
}