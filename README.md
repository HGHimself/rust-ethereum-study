# Rust Ethereum Study

> The only thing that really worried me was the ether. There is nothing in the world more helpless and irresponsible and depraved than a man in the depths of an ether binge, and I knew we'd get into that rotten stuff pretty soon. - Hunter S. Thompson

### To Run
- `sudo npm install -g ganache-cli` to get the package that will run a private local blockchain
- `ganache-cli -d` to start the private local blockchain
- `rustup install nightly` to get your toolchain in order
- `cargo run` to fire off a test

## What is ethereum
https://www.youtube.com/watch?v=jxLkbJozKbY

Ethereum is a decrentalized platform that allows people to build something called smart contracts. These smart contracts are similar to object oriented classes in that they have data and functions locked away within them. The functionality is immutable but the data within can be modified at a cost to the user making these transactions. Whenever a part of state gets modified, the transaction gets sent to a queue where miners will do the updating work for the aforementioned cost. Higher cost means your transaction leaves gets selected by miners and leaves the queue faster.

Decentralized applications (dapp) can use this as a smart database that will be a part of their backend. Some examples can be as simple as storing a single number, but some extreme apps are comparable to more classical examples. The ethereum website has a showcase of fantastic tools and such built on the platform.

A good example of a dapp would be a crowdfunding application. A smart contract gets written to hold logic for people to donate money, increasing the total donation until the limit is reached. When the limit gets reached, the donation goes to the recipient. If it never gets met, everyone gets the refund. The positives to this are the fact that the logic is locked away and immuatble. Once deployed, the rules mentioned a priori can only happen no matter what. Downside is that if someone wrote a bad contract, then you are stuck with it. This makes testing very important.

## What is a Smart Contract
https://www.youtube.com/watch?v=

We will use a smart contract to hold the balance for the users who have store credit. This will look like a key value pair as so:
```javascript
balances = {
  user1: 100,
  user2: 0,
}
```

I am assuming we want people to be able to do a handful of actions, the most important three being `addCredit`, `redeemCredit`, and `queryCredit` functions. This would cover the bare minimum. It would also be important to note that we want to limit who can make these transactions. For the add and redeem credit, we need to limit the action to the owner, aka us. Noone else should be able to add or remove credit; this is very important. Unsure at the moment what would be necessary for the query balance feature.

We need to figure out how we want to segment where the credit is destined for. I doubt we want people to use credits amongst stores, so we will have to designate some aspect of these tokens in order to identify them. We can do this in two ways, dependent on a couple factors which I will outline soon.

First we can keep all of the data in a singular smart contract. The data representation would look like so: `balances[storeId][userId] = some-int`. This downside is how much data are we forcing into one smart contract. We need to find out how much we can pack into one contract at a time and if there are any limitations. If there are limitations, we can get around this by going with option number 2. This will allow a user to kick off their own smart contract which will be a carbon copy of other smart contracts, holding only their data for their store. This is way more expensive but potentially could scale more.

The second option, generating unique smart contracts, would be neat because once a smart contract is deployed, we will never be able to modify it. It is completely immutable (other than the inner state variables). All the functionality is locked away. If we have a way to make many smart contracts (at a cost of course), we would have an easier time fixing issues as the arise than if we were to lock all of our user's content in a single monolithic smart contract. This could be no big deal though so we need to think harder about it.

#### Costs of operation
In order to deploy a smart contract, we have to pay for gas with some denomination of ether. This will be a bigger cost than making subsequent `write` transactions to the contract. Luckily `read` actions are free from charge. The reason for this is because actions that modify state have to go onto the blockchain; forcing the entire network to update. On the other hand, just querying the state is easy because a given node can just use its own copy of the blockchain to infer the global state. I am unsure yet to what the overall costs are but I will add this soon based on decisions we make about issues noted above.


## What are tokens
https://www.youtube.com/watch?v=cqZhNzZoMh8
https://ethereum.org/en/developers/docs/standards/tokens/
https://ethereum.org/en/developers/docs/standards/tokens/erc-20/


#### Credits
Seeming as we need to make a name for the credits for the purpose of this study, we will pull inspiration from literature. **Air Dollars** comes from H.G. Well's *The Shape of Things to Come*. Our symbol will be **ADZ**, and the smallest denomination we support is to the hundredth decimal place.

> The air-dollar was not a metallic coin at all; it was a series of paper notes, which represented distance, weight, bulk, and speed. Each note was good for so many kilograms in so much space, for so many kilometres at such a pace. The value of an air-dollar had settled down roughly to a cubic metre weighing ten kilograms and travelling two hundred kilometres at a hundred kilometres an hour


### Questions
##### How do we link a user to their balance?
- We can keep it within the blockchain by using a wallet account (wallets are hard and scary)
- We can keep their info within shopify as a customer perhaps (responsibility on us, not extendable outside shopify)
- We can generate a key for the user to hold onto (responsibility on user)

##### How do we segment one shop from another?
- We can have a mega smart contract holding all shops and all of their credits per user (quite a bit of programming and uses a lot of space)
- We can spawn a new credit setup for each new shop (incurs slightly more cost)
