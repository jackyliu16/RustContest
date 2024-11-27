use std::io::ErrorKind;
use std::str::FromStr;

#[derive(Debug)]
pub(crate) enum PersonnelCategory {
    Man,
    FemaleCadres,
    FemaleWorkers,
}

impl FromStr for PersonnelCategory {
    type Err = std::io::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "原法定退休年龄50周岁女职工" => Ok(PersonnelCategory::FemaleWorkers),
            "男职工" => Ok(PersonnelCategory::Man),
            "原法定退休年龄55周岁女职工" => Ok(PersonnelCategory::FemaleCadres),
            _ => Err(std::io::Error::new(ErrorKind::InvalidData, "未识别到该员工类别"))
        }
    }
}