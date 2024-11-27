use crate::date::Date;
use crate::rules::RetirementRules;
use crate::types::PersonnelCategory;

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
