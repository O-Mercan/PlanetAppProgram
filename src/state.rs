use borsh::{BorshSerialize, BorshDeserialize};


#[derive(BorshSerialize, BorshDeserialize, Debug, PartialEq, Clone)]
pub struct Explorer {
    pub name: String,
    pub surname: String,
    pub age: u8,
    pub planet_amount: u8,
    pub explorer_account: [u8; 32],
}

#[derive(BorshSerialize, BorshDeserialize, Debug, PartialEq, Clone)]
pub struct Planet {
    pub name: String,
    pub planet_age: u8,
    pub explorer_account: [u8; 32],
    pub is_confirmed: u8,
}
