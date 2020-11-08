use std::fmt::{self, Display};

use serenity::model::{id::UserId, misc::Mentionable};

#[derive(Debug, Clone)]
pub enum KaisaneeSpecifier {
    Me,
    All,
    Users(Vec<UserId>),
}

impl Default for KaisaneeSpecifier {
    fn default() -> Self {
        KaisaneeSpecifier::All
    }
}

impl Display for KaisaneeSpecifier {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            KaisaneeSpecifier::Me => f.write_str("あなた"),
            KaisaneeSpecifier::All => f.write_str("全員"),
            KaisaneeSpecifier::Users(ids) => {
                if let Some(id) = ids.first() {
                    write!(f, "{}", id.mention())?;
                }

                for id in ids[1..].into_iter() {
                    write!(f, ", {}", id.mention())?;
                }

                Ok(())
            }
        }
    }
}
