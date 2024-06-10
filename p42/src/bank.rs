use std::collections::HashMap;

#[derive(Clone, Debug)]
/// Data type to hold users
pub struct User {
    name: String,
    credit_line: u64,
    balance: i64,
}

#[derive(Debug)]
/// Data type to hold banks
pub struct Bank {
    users: HashMap<String, User>,
    pub name: String,
    credit_interest: u64,
    debit_interest: u64,
}

pub enum Error {
    UserNotFound { uname: String },
    BalanceOverflow { uname: String },
    BalanceUnderflow { uname: String },
    BadAmount { amount: u64 },
    CreditLimitReached { uname: String },
    LiabilitiesOverflow,
    AssetsOverflow,
    InterestOverflow { uname: String },
    InterestUnderflow { uname: String },
    BadInterest { interest: u64 },
}

/// Implementation of methods for bank
impl Bank {
    /// Compute total liabilities and assets and return as a tuple
    pub fn calc_balance(&self) -> Result<(u64, u64), Error> {
        let mut liabilities: u64 = 0;
        let mut assets: u64 = 0;
        for user in self.users.values() {
            if user.balance > 0 {
                liabilities = liabilities
                    .checked_add(user.balance.unsigned_abs())
                    .ok_or(Error::LiabilitiesOverflow)?;
            } else {
                assets = assets
                    .checked_add(user.balance.unsigned_abs())
                    .ok_or(Error::AssetsOverflow)?;
            }
        }
        Ok((liabilities, assets))
    }

    /// Transfer fund from debit-user to credit-user
    pub fn transfer_funds(
        &mut self,
        uname_d: String,
        uname_c: String,
        amount: u64,
    ) -> Result<(), Error> {
        let a = i64::try_from(amount).map_err(|_| Error::BadAmount { amount })?;
        let user_d = self.users.get(&uname_d).ok_or(Error::UserNotFound {
            uname: uname_d.clone(),
        })?;
        let user_c = self.users.get(&uname_c).ok_or(Error::UserNotFound {
            uname: uname_c.clone(),
        })?;
        let balance_d = user_d.balance;
        let balance_c = user_c.balance;
        let balance_d_new = balance_d.checked_sub(a).ok_or(Error::BalanceUnderflow {
            uname: uname_d.clone(),
        })?;
        if balance_d_new < 0 && balance_d_new.unsigned_abs() > user_d.credit_line {
            return Err(Error::CreditLimitReached {
                uname: uname_d.clone(),
            });
        }
        let balance_c_new = balance_c.checked_add(a).ok_or(Error::BalanceOverflow {
            uname: uname_c.clone(),
        })?;
        self.users.get_mut(&uname_d).unwrap().balance = balance_d_new;
        self.users.get_mut(&uname_c).unwrap().balance = balance_c_new;
        Ok(())
    }

    /// Compute interest for each user
    pub fn accrue_interest(&mut self) -> Result<(), Error> {
        let mut interest: i64;
        let d_i = i64::try_from(self.debit_interest).map_err(|_| Error::BadInterest {
            interest: self.debit_interest,
        })?;
        let c_i = i64::try_from(self.credit_interest).map_err(|_| Error::BadInterest {
            interest: self.credit_interest,
        })?;
        for user in self.users.values_mut() {
            interest = if user.balance > 0 {
                user.balance
                    .checked_mul(d_i)
                    .ok_or(Error::InterestOverflow {
                        uname: user.name.clone(),
                    })?
            } else {
                user.balance
                    .checked_mul(c_i)
                    .ok_or(Error::InterestUnderflow {
                        uname: user.name.clone(),
                    })?
            };
            user.balance =
                user.balance
                    .checked_add(interest / 10000)
                    .ok_or(Error::BalanceOverflow {
                        uname: user.name.clone(),
                    })?;
        }
        Ok(())
    }

    /// Merge two banks and destroy the second
    pub fn merge_bank(&mut self, bank: Bank) -> Result<(), Error> {
        for (uname, user) in bank.users.iter() {
            if self.users.contains_key(uname) {
                let u = self.users.get_mut(uname).unwrap();
                u.balance = u
                    .balance
                    .checked_add(user.balance)
                    .ok_or(Error::BalanceOverflow {
                        uname: uname.clone(),
                    })?;
            } else {
                self.users.insert(uname.clone(), user.clone());
            }
        }
        Ok(())
    }
}
