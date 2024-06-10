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
    users: Vec<User>,
    pub name: String,
    credit_interest: u64,
    debit_interest: u64,
}

pub enum Error {
    UserNotFound { username: String },
    BalanceOverflow { username: String },
    BalanceUnderflow { username: String },
    BadAmount { amount: u64 },
    CreditLimitReached { username: String },
    LiabilitiesOverflow,
    AssetsOverflow,
    InterestOverflow { username: String },
    BadInterest { interest: u64 },
}

/// Implementation of methods for bank
impl Bank {
    /// Compute total liabilities and assets and return as a tuple
    pub fn calc_balance(&self) -> Result<(u64, u64), Error> {
        let mut liabilities: u64 = 0;
        let mut assets: u64 = 0;
        for user in self.users.iter() {
            if user.balance > 0 {
                liabilities = match liabilities.checked_add(user.balance.unsigned_abs()) {
                    Some(b) => b,
                    None => return Err(Error::LiabilitiesOverflow),
                }
            } else {
                assets = match assets.checked_add(user.balance.unsigned_abs()) {
                    Some(b) => b,
                    None => return Err(Error::AssetsOverflow),
                }
            }
        }
        Ok((liabilities, assets))
    }

    /// Transfer fund from debit-user to credit-user
    pub fn transfer_funds(
        &mut self,
        username_debit: String,
        username_credit: String,
        amount: u64,
    ) -> Result<(), Error> {
        let a = i64::try_from(amount).map_err(|_| Error::BadAmount { amount })?;
        let mut index_d_option: Option<usize> = None;
        let mut index_c_option: Option<usize> = None;
        for (index, user) in self.users.iter().enumerate() {
            if user.name == username_debit {
                index_d_option = Some(index);
            }
            if user.name == username_credit {
                index_c_option = Some(index);
            }
            if let (Some(_), Some(_)) = (index_d_option, index_c_option) {
                break;
            }
        }
        let (i_d, i_c) = match (index_d_option, index_c_option) {
            (None, _) => {
                return Err(Error::UserNotFound {
                    username: username_debit,
                })
            }
            (Some(_), None) => {
                return Err(Error::UserNotFound {
                    username: username_credit,
                })
            }
            (Some(index_d), Some(index_c)) => (index_d, index_c),
        };
        let balance_d = self.users[i_d].balance;
        let balance_c = self.users[i_c].balance;
        let balance_d_new = balance_d.checked_sub(a).ok_or(Error::BalanceUnderflow {
            username: username_debit.clone(),
        })?;
        if balance_d_new < 0 && balance_d_new.unsigned_abs() > self.users[i_d].credit_line {
            return Err(Error::CreditLimitReached {
                username: username_debit.clone(),
            });
        }
        let balance_c_new = balance_c.checked_add(a).ok_or(Error::BalanceOverflow {
            username: username_credit.clone(),
        })?;
        self.users[i_d].balance = balance_d_new;
        self.users[i_c].balance = balance_c_new;
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
        for user in self.users.iter_mut() {
            interest = if user.balance > 0 {
                user.balance
                    .checked_mul(d_i)
                    .ok_or(Error::InterestOverflow {
                        username: user.name.clone(),
                    })?
            } else {
                user.balance
                    .checked_mul(c_i)
                    .ok_or(Error::InterestOverflow {
                        username: user.name.clone(),
                    })?
            };
            user.balance =
                user.balance
                    .checked_add(interest / 10000)
                    .ok_or(Error::BalanceOverflow {
                        username: user.name.clone(),
                    })?;
        }
        Ok(())
    }

    /// Merge two banks and destroy the second
    pub fn merge_bank(&mut self, bank: Bank) -> Result<(), Error> {
        'outer_loop: for user in bank.users.iter() {
            for u in self.users.iter_mut() {
                if user.name == u.name {
                    u.balance =
                        u.balance
                            .checked_add(user.balance)
                            .ok_or(Error::BalanceOverflow {
                                username: u.name.clone(),
                            })?;
                    continue 'outer_loop;
                }
            }
            self.users.push(user.clone());
        }
        Ok(())
    }
}
