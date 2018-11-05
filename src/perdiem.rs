use chrono::{Date, Utc};
use failure::Error;
use std::str::FromStr;
use time::Duration;
use types::{TransactionList, TransactionListElement};
use {Context, EXPENSIFY_DATE_FORMAT};

impl TransactionList {
    pub fn from_per_diem(ctx: Context, period: TimePeriod, kind: Kind) -> Result<Self, Error> {
        Ok(TransactionList {
            transaction_list_type: "expenses".to_owned(),
            employee_email: ctx.user.email.clone(),
            transaction_list: period.into_transactions(&ctx, kind)?,
        })
    }
}

pub enum TimePeriod {
    Weekdays,
}

pub enum Kind {
    FullDay,
}

impl FromStr for TimePeriod {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, <Self as FromStr>::Err> {
        use self::TimePeriod::*;
        Ok(match s {
            "weekdays" => Weekdays,
            _ => bail!("Invalid time period specification: '{}'", s),
        })
    }
}

impl FromStr for Kind {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, <Self as FromStr>::Err> {
        use self::Kind::*;
        Ok(match s {
            "fullday" => FullDay,
            _ => bail!("Invalid per diem kind specification: '{}'", s),
        })
    }
}

fn to_date_string(d: &Date<Utc>) -> String {
    d.format(EXPENSIFY_DATE_FORMAT).to_string()
}

impl TimePeriod {
    fn into_transactions(
        self,
        ctx: &Context,
        _kind: Kind,
    ) -> Result<Vec<TransactionListElement>, Error> {
        use self::TimePeriod::*;

        let mut ts = Vec::new();
        match self {
            Weekdays => {
                let monday = ctx.monday_of_reference_date()?;
                let friday = monday.checked_add_signed(Duration::days(5 - 1)).unwrap();

                ts.push(TransactionListElement {
                    created: to_date_string(&monday),
                    currency: String::new(),
                    merchant: String::new(),
                    amount: 0,
                    category: String::new(),
                    tag: ctx.user.project.clone(),
                    billable: false,
                    reimbursable: false,
                    comment: format!("{} to {}", to_date_string(&monday), to_date_string(&friday)),
                });
            }
        }
        Ok(ts)
    }
}
