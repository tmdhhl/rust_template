use strum::{AsRefStr, Display, EnumString, IntoStaticStr};

#[derive(IntoStaticStr, AsRefStr, EnumString, Default, Display)]
#[strum(serialize_all = "snake_case")]
pub enum Environment {
    #[default]
    Local,
    Production,
}
