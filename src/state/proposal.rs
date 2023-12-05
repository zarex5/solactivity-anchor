use anchor_lang::prelude::*;

#[account] //8 + 140 = 148
pub struct Proposal { //TODO: set properties back to non pub and use getters/setters?
    pub author: Pubkey,    //32
    pub program: Pubkey,   //32
    pub name: String,      //4 + 34: 38
    pub group: String,     //4 + 8: 12
    pub sub_group: String, //4 + 18: 22
    pub score: i32,        //4
}
