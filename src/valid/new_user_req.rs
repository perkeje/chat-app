use serde::Deserialize;
use validr::*;

#[derive(Clone, Deserialize, Debug)]
pub struct NewUserReq {
    pub email: Option<String>,
    pub pass: String,
}
impl Validation for NewUserReq {
    fn rules(&self) -> Vec<Rule<Self>> {
        vec![
            rule_email!(email),
            rule_length_min!(pass, 3),
            rule_required!(email),
            rule_required!(pass),
        ]
    }
    fn modifiers(&self) -> Vec<Modifier<Self>> {
        vec![modifier_lowercase!(email)]
    }
}
