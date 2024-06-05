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
    CreditLimitReached,
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
    ) -> Result<bool, Error> {
        let mut result: Result<bool, Error> = Ok(true);
        let a = match i64::try_from(amount) {
            Ok(am) => am,
            Err(..) => return Err(Error::BadAmount { amount }),
        };
        let mut reset_flag: bool = false;
        {
            // scope for user_option
            let mut user_option: Option<&mut User> = None;
            for user_d in self.users.iter_mut() {
                if user_d.name == username_debit {
                    user_option = Some(user_d);
                    break;
                }
            }
            match user_option {
                None => {
                    return Err(Error::UserNotFound {
                        username: username_debit,
                    })
                }
                Some(user_d) => match user_d.balance.checked_sub(a) {
                    Some(b) => {
                        if b >= 0 || b.unsigned_abs() <= user_d.credit_line {
                            user_d.balance = b
                        } else {
                            return Err(Error::CreditLimitReached);
                        }
                    }
                    None => {
                        return Err(Error::BalanceUnderflow {
                            username: username_debit,
                        })
                    }
                },
            }
        }
        {
            // renewed scope for user_option
            let mut user_option: Option<&mut User> = None;
            for user_c in self.users.iter_mut() {
                if user_c.name == username_credit {
                    user_option = Some(user_c);
                    break;
                }
            }
            match user_option {
                None => {
                    result = Err(Error::UserNotFound {
                        username: username_credit,
                    });
                    reset_flag = true
                }
                Some(user_c) => match user_c.balance.checked_add(a) {
                    Some(b) => user_c.balance = b,
                    None => {
                        result = Err(Error::BalanceOverflow {
                            username: username_credit,
                        });
                        reset_flag = true
                    }
                },
            }
        }
        if reset_flag {
            for user_d in self.users.iter_mut() {
                if user_d.name == username_debit {
                    user_d.balance += a;
                    return result;
                }
            }
        }
        result
    }

    // Transfer funds rewritten using indices
    pub fn transfer_funds_alternate(
        &mut self,
        username_debit: String,
        username_credit: String,
        amount: u64,
    ) -> Result<(), Error> {
        let a = match i64::try_from(amount) {
            Ok(am) => am,
            Err(..) => return Err(Error::BadAmount { amount }),
        };
        let mut index_d_option: Option<usize> = None;
        let mut index_c_option: Option<usize> = None;
        for (index, user) in self.users.iter().enumerate() {
            if user.name == username_debit {
                index_d_option = Some(index);
            }
            if user.name == username_credit {
                index_c_option = Some(index);
            }
            if let (Some(..), Some(..)) = (index_d_option, index_c_option) {
                break;
            }
        }
        let (i_d, i_c) = match (index_d_option, index_c_option) {
            (None, _) => {
                return Err(Error::UserNotFound {
                    username: username_debit,
                })
            }
            (Some(..), None) => {
                return Err(Error::UserNotFound {
                    username: username_credit,
                })
            }
            (Some(index_d), Some(index_c)) => (index_d, index_c),
        };
        let balance_d = self.users[i_d].balance;
        let balance_c = self.users[i_c].balance;
        let balance_d_new = balance_d.checked_sub(a).ok_or(Error::BalanceUnderflow {
            username: username_debit,
        })?;
        let balance_c_new = balance_c.checked_sub(a).ok_or(Error::BalanceUnderflow {
            username: username_credit,
        })?;
        self.users[i_d].balance = balance_d_new;
        self.users[i_c].balance = balance_c_new;
        Ok(())
    }

    /// Compute interest for each user
    pub fn accrue_interest(&mut self) -> Result<(), Error> {
        let mut interest: i64;
        let d_i = match i64::try_from(self.debit_interest) {
            Ok(i) => i,
            Err(..) => {
                return Err(Error::BadInterest {
                    interest: self.debit_interest,
                })
            }
        };
        let c_i = match i64::try_from(self.credit_interest) {
            Ok(i) => i,
            Err(..) => {
                return Err(Error::BadInterest {
                    interest: self.credit_interest,
                })
            }
        };
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
        let mut flag: bool = false;
        for user in bank.users.iter() {
            for u in self.users.iter_mut() {
                if user.name == u.name {
                    flag = true;
                    u.balance =
                        u.balance
                            .checked_add(user.balance)
                            .ok_or(Error::BalanceOverflow {
                                username: u.name.clone(),
                            })?
                }
            }
            if !flag {
                self.users.push(user.clone());
            }
            flag = false;
        }
        Ok(())
    }
}
