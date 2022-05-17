## Inspiration  
When we see the reverse Gas model of Dfinity, we consider that it will keep consuming Canister's cycles balance if it suffers malicious calls from attackers. So based on this inspiration, we thought that developers have the need to monitor Cycles balance and prevent DDos attacks. Inspired by this, we decided to develop the product.

## What it does  
Based on the reverse Gas model of IC platform, we consider Ddos attack as a headache. The purpose of this product is to help developers defend against DDos attacks and to limit traffic by sending messages to developers who register for the service when a Ddos attack occurs.

## How we built it  

A ddos attack is characterized by a rapid decrease in the cycle balance of the canister. If this feature is monitored, the code of the target canister's prove of work mechanism will be called. The product part of the prove of work will be in the form of a third-party library.  

IC's System call provides the [canister_inspect_message] C interface, which will let the request do some judgment before entering the canister, accepting the request into the canister if it meets the conditions, discarding the request if it does not, and not consuming the cycle. we have the [canister_ inspect_message] request to generate a random number on the chain, with the random number requested by the user over to do the hash function, requiring Hash(random_on_chain,random_off_chain) to meet certain conditions. Because of the one-way nature of the hash function and the sensitivity to the input, it can achieve the purpose of the PROVE of work.

Because the cycle is consumed will lead to canister data loss, so the user has the need to do renewal vault, if when the user canister is DDoS attack, cycle balance is less or insufficient, in order to ensure that the canister continues to operate normally, DDD to the attacked canister to recharge cycle, the part used This part uses the feature of transferring cycles between canisters.

## Challenges we ran into  

The developer is required to register in the canister of the query cycle service and reference a third-party ddos attack library. Because it is impossible to distinguish between users and attackers when DDos attacks occur, the developer's front-end should also implement the logic to re-produce random numbers to request again when the call fails, otherwise it will reduce the user experience to let users request the front-end interface multiple times.

## Accomplishments that we're pround of  

We are glad to establish the connection between DDos attack and workload proof to achieve the effect that we need to do workload proof to limit the flow when DDos attack occurs.

## What we learned  

By participating in the hackathon, we learned the advantages of IC's blockchain architecture over previous generations of blockchains. It makes us more confident to build new projects on IC.

## What's next for Dfinity DDos Defender  
to do