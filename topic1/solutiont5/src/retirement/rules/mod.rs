//! 基于策略模式思想实现的简单退休政策实现
//! 
//! # 示例:
//! ```rust
//! // 根据需要添加对应退休政策实现
//! let retire_rules = CombinedRules { rules: vec![
//!         Box::new(rules::Rules20240913),
//!         Box::new(rules::Rules1978)
//!     ]
//! } ;
//! /// 根据列表中顺序获取总的, 到达退休状态需要工作的总时间
//! let working_time = retire_rules.calculate_working_date(&birth_date, &types);
//! ```
//!
//! ```rust
//! // 根据某个特定的规则获取对应的退休时间
//! let retire_working_time = rules::Rules1978.calculate_working_date(&birth_date, &types);
//! ```
mod rules_20240913;
mod rules_1978;
pub use rules_1978::Rules1978;
pub use rules_20240913::Rules20240913;
use crate::retirement::date::Date;
use crate::retirement::types::PersonnelCategory;

pub trait RetirementRules {
    /// 计算按照当前规则情况下工作时间会延长多少 工作时间(Date)
    ///
    /// # 输入值
    ///     date: 根据 Date 结构体规约的出生日期
    ///     types: 根据 PersonalCategory 规约的人员类别
    ///
    /// # 输出值
    ///     Option<Date>: 添补/削减总工作时间
    fn calculate_working_date(&self, date: &Date, types: &PersonnelCategory) -> Option<Date>; // working date
}

// 组合结构体，持有多个规则
pub struct CombinedRules {
    pub rules: Vec<Box<dyn RetirementRules>>,
}

impl RetirementRules for CombinedRules {
    fn calculate_working_date(&self, date: &Date, types: &PersonnelCategory) -> Option<Date> {
        let mut total_working_date: Option<Date> = None;
        for rule in &self.rules {
            if let Some(date) = rule.calculate_working_date(date, types) {
                total_working_date = total_working_date
                    .map(|d| d + date) // 如果当前已经赋值了,则向其中添加
                    .or_else(|| Some(date)); // 否则创建一个新的
            }
        }
        // total_working_date.map(|work_date| work_date + *date)
        total_working_date
    }
}