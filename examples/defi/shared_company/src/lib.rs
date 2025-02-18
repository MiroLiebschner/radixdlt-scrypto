use scrypto::prelude::*;
mod proposal;
use crate::proposal::Proposal;

/*
A SharedCompany which is a company owned and governed by its shareholders.
Shares can be bought (fixed rate for now).
Funds can be spend using [Proposal] if accepted by a majority (of company_voting_tokens which are distributed with shares).
This is only a demonstration. There are known security flaws in this design.
(like: Buy a lot of shares, make proposal to send you all company_radix, vote for it).
*/
blueprint! {

struct SharedCompany {
    company_radix:  Vault,
    company_shares: Vault,
    company_voting_token: Vault,
    share_counter: Decimal,
    price_share: Decimal,
    proposal_list: Vec<Address>,
}


impl SharedCompany {
    // Creates a new component
    pub fn new(price_share: Decimal) -> Component{


        // shares are equity in the company. They can be bought for a fixed rate or returned for a part of the company vault
        let shared_company_share_resource_def = ResourceBuilder::new()
        .metadata("name", "SharedCompany share").metadata("symbol", "SC")
        .new_token_fixed(1_000_000);

        // voting tokens are used to vote on proposals
        let shared_company_voting_token_resource_def = ResourceBuilder::new()
        .metadata("name", "SharedCompany voting token").metadata("symbol", "SCVT")
        .new_token_fixed(1_000_000);

        // stores the addresses of proposal-components
        let proposal_address_list: Vec<Address>= Vec::new();


    //populate the SharedCompany struct and instantiate a new component
    Self {
        company_shares: Vault::with_bucket(shared_company_share_resource_def),
        company_voting_token: Vault::with_bucket(shared_company_voting_token_resource_def),
        company_radix: Vault::new(RADIX_TOKEN),
        share_counter: Decimal(0.0 as i128),
        price_share: price_share,
        proposal_list: proposal_address_list
    }.instantiate()

    }

    /// buys an amount of shares and returns change
    pub fn buy_shares(&mut self, payment: Bucket) -> (Bucket, Bucket, Bucket) {
        let max_share_buy_power = payment.amount() / self.price_share;
        // Increase the share_counter so the amount of shares that are outstanding is tracked
        self.share_counter += max_share_buy_power;
        // take our price in XRD out of the payment and store it in the company vault
        let collected_xrd = payment.take(self.price_share * max_share_buy_power);
        self.company_radix.put(collected_xrd);
        // return the share(s) and change
        (self.company_shares.take(max_share_buy_power), payment, self.company_voting_token.take(max_share_buy_power))
    }

     /// sells an amount of shares and for a part of the companies xrd
     pub fn sell_shares(&mut self, shares: Bucket, voting_token: Bucket) -> (Bucket, Bucket) {
        if shares.amount() != voting_token.amount()  {return (shares, voting_token)};
        // calculates the percentage of all shares
        let percentage_of_all_shares = shares.amount() / self.share_counter;
        // Decreases the counter
        self.share_counter -= shares.amount();
        //ToDoBurns the shares instead
        self.company_shares.put(shares);
        //ToDO Burn the voting_token instead
        self.company_voting_token.put(voting_token);
        info!( "You are paid out {:?} Radix!", self.company_radix.amount());
        // returns the same percentage of the company xrd
        (self.company_radix.take(self.company_radix.amount() * percentage_of_all_shares), Bucket::new(RADIX_TOKEN))
    }

    // A proposal that if it is accepted sends funds away from the company
    pub fn make_proposal( &mut self,cost_as_number: u32, destination_adress: Address,reason: String,end_epoch: u64,){
        // Creates the proposal component
        let new_proposal = Proposal::new(self.company_radix.take(cost_as_number), destination_adress, reason, self.company_radix.resource_address(), end_epoch, self.share_counter / 2 + 1, self.company_voting_token.resource_def());
        //Here i want to pass the address of the new comonent
        self.proposal_list.push(new_proposal.address());
        info!("Proposal created! Destination adress if accepted: {}", destination_adress);
    }

    // Helper Methods which infor the user about interactions with the company.

    // Returns the price per share
     pub fn get_price(&self)  {
        info!( "Price is {}",  self.price_share);
    }

    // Returns the selling price per share
    pub fn get_seeling_price(&self) {
        info!("Selling price per share is {:?}", self.company_radix.amount() / self.share_counter)
    }


    // Returns all the open proposals
    pub fn get_proposal_list(&self){
        info!("{:?}", self.proposal_list);
    }
}
}
