# Richie Rich Box Tutorial

## Introduction

Welcome to the Richie Rich Box tutorial! In this tutorial, we will explore a simple contract that demonstrates how to implement viewing keys and permits on Secret contracts. By following along with this tutorial, you will gain hands-on experience writing the critical parts of code needed to create authenticated queries.

We have a viewing key and query permit pathway that complements this box. The pathway provides a more detailed explanation of these concepts, while this box focuses on providing practical experience.

You can work through this Secret box using either gitpod or your local environment. If you prefer to use your local environment, you can follow the “Getting Started” steps [here](tbc) to set everything up.

## Contract overview

The Richie Rich contract is designed to solve the Millionaire’s problem. In short, this problem involves two people who want to know which of them is richer without revealing their actual networth. There are many solutions to this problem, but on Secret Network, we can take advantage of the privacy inherent in Secret contracts to create a basic implementation. 

The Richie Rich contract allows two users to submit their networth to the contract. Once the contract receives both inputs, it is possible to perform one of two queries for each account:
- Return the result of the comparison (ie: if the account owner is the richer of the two users)
- Return the result of the comparison, in addition to the networth value the user submitted

Both queries are authenticated and require the use of a viewing key or query permit, which are the core focus of this lesson. The contract makes use of these keys and permits to provide users with control over who is able to view the result.

Outside of these authenticated queries, the core contract implementation is deliberately simple and in some cases naive, as the focus is walking through implementing viewing keys and queries. 

> **What's wrong with this simple implementation?**
> 
> Our focus with RichieRich is to demonstrate viewing keys and permits. So these parts are secure. However, the contract itself has a critical privacy vulnerability. A side channel attack can reveal the networth of the other user. 
>
> Let's start with the most naive implementation of all, where two users submit their networth, and are able to revise their inputs any time. Suppose one user (Alice) has submitted her networth, then the other user (Bob) can submit “dummy” inputs starting from 0 SCRT and progressively increment the amount until he sees the result switch from is_richest == False to is_richest == True. This effectively reveals Alice’s exact networth. Notice that Alice can also perform the same attack on Bob, assuming that Bob does not perform the attack first.  
>
> This attack cannot be done on the RichieRich contract, as it only allows one input per user. Once it accepts two user inputs, it stops accepting anything else. This prevents someone from entering values arbitrarily as described above. However, this exact attack can still be performed by the second user, by utilizing a side channel. Bob could create many forks of the mainnet and try different networth inputs on each, until he determines Alice’s exact networth. While this attack is more complex and only compromises the first user, it is still relatively trivial for a determined attacker. This vulnerability is also described in the Secret docs’ [description of some vulnerabilities](https://docs.scrt.network/secret-network-documentation/overview-ecosystem-and-technology/techstack/privacy-technology/theoretical-attacks#more-advanced-tx-replay-attacks-search-to-decision-for-millionaire-s-problem).

> **Allowing only one round seems inconvenient. Are there ways to improve UX?**
> 
> Of course. If you are interested, you can improve the contract such that after taking two inputs, another round begins. You can even allow multiple rounds to run concurrently. Doing this does not increase or decrease the severity of the vulnerability described above. It is not difficult to implement these; although it requires more lines of code. It is not required for our purposes though, as we can easily instantiate new contracts each time we wish to start a new round.

## Web application overview

In addition to the contract itself, we also provide a web application frontend that allows you to interact with the contract. We will use secret.js and Vue.js, walking through the required code to get a front end application to interact with the Secret contract. The frontend GUI design itself is intended to help developers understand how authenticated queries work by providing a visual way to interact with the contract. As a result, the GUI is not streamlined in a way that is suitable or intended for end-users.

Of course, if you want, you can implement an improved GUI more suitable for an end-user facing app, which will primarily be a "pure" Web2 undertaking. We won't cover that as our focus is the interface between the front-end and the contract. 

## Tutorial starting point

Now it’s time to get our hands dirty!

Start by opening the Secret box on Gitpod or in your local environment. You can follow the “Getting Started” steps in the [README of this repo](https://github.com/secretuniversity/richie-rich-vuejs-box/blob/main/README.md) to set everything up.

At the starting point, if you are using gitpod, your environment should be properly set up and your workspace should include:
- A running LocalSecret blockchain instance
- An initial version of the contract uploaded to LocalSecret
- An incomplete web app launched, which includes this tutorial

Additionally, you should also have three terminal windows open:
- LocalSecret: The first terminal displays the blockchain starting up and producing blocks
- Secret Box workspace: The second terminal is where you will compile and deploy your contract and enter commands as you go through this tutorial
- Web application frontend: The third terminal is where you will launch your application server after LocalSecret is running and the Secret contract has been created

If you are running locally, make sure to have these three terminals open as well.

The files you will be working with will be:
- src/* : these are the contract source files
- app/src/components/SecretBox/* : these are the front-end source files

In these files, look for sections marked with the comments `// complete code here`. These are the core parts of code required to implement authenticated queries.


## Implementing viewing keys

Our contract already implements the execute functions we need. However, the queries have not yet been implemented. At this point, the contract can accept two query messages `all_info` and `am_i_richest` but simply returns a blank response to the caller.

So let's do something about it and start implementing viewing key queries on this contract. 

### msg.rs

Begin by opening the src/msg.rs file, and find the QueryMsg enum.

```rust
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum QueryMsg {
    AllInfo { 
        addr: Addr,
        key: String,
    },
    AmIRichest {
        addr: Addr,
        key: String,
    },
    //
    // complete code here
    // 
}
```

The above code defines the QueryMsg variants for our two viewing key queries. Notice they each accept two fields: addr and key. These two fields are the minimum required for any viewing key query, as these two fields are required for the contract to authenticate the viewing key. This is discussed in more detail in our viewing key and query permit pathway.

Everything is already done here, so there is nothing further for you to do. The incomplete code is for permits, which we will do later.

Below, you will find the code that defines a method for QueryMsg:

```rust
impl QueryMsg {
    pub fn get_validation_params(&self) -> (/* complete code here */) {
        //
        // complete code here
        //
    }
}
```

> **Exercise: Complete the code**
>
> This method should return the address and viewing key for all possible viewing key query variants in QueryMsg. It’s also a good idea to verify that the address is in a valid format, which will require an additional input field.

<details> <summary> Hint 1:</summary>

The return type should be 
```
StdResult<(Addr, String)>
```
</details>

<details> <summary> Hint 2:</summary>

The method should look at the possible variants of QueryMsg, and return the corresponding address and key. So, first line of code in the method should be:
```
        match self {
```
</details>

The answer to the above exercise is:
```rust
impl QueryMsg {
    pub fn get_validation_params(&self, api: &dyn Api) -> StdResult<(Addr, String)> {
        match self {
            Self::AllInfo { addr, key } => {
                let address = api.addr_validate(addr.as_str())?;
                Ok((address, key.clone()))
            }
            Self::AmIRichest { addr, key } => {
                let address = api.addr_validate(addr.as_str())?;
                Ok((address, key.clone()))
            },
        }
    }
}
```

A note on the `&dyn Api` syntax:

To verify that the address is in a valid format, we use `addr_validate` method of the Api trait. In order to do this, we add the `api` input field which has the type signature `&dyn Api`. If you’re unfamiliar, this is a trait object. Trait objects are commonly used in CosmWasm. Essentially, trait objects do not specify the required concrete type, but instead any type that implements the required trait is accepted. The concrete trait can only be known at runtime. Using trait objects instead of concrete types provides more flexibility, while retaining the safety guarantees that Rust provides. One downside is that Rust’s compiler cannot check for all possible errors at compile time. Another downside is a small performance penalty, which may be significant in systems engineering but is immaterial in the context of smart contracts. If you wish to learn more, The Rust Book provides an in-depth explanation of trait objects.



### contract.rs

Open the src/contract.rs file. At the query entry point function, you will find these lines of incomplete code:

```rust
    let q_response = match msg {
        QueryMsg::AllInfo { .. } => {
            //
            // complete code here
            // 
            ()
        },
        QueryMsg::AmIRichest { .. } => {
            //
            // complete code here
            // 
            ()
        },
        // ...
    };
    to_binary( /* complete code here */ "placeholder")
```

> **Exercise: Complete the code**
>
> Here, we need to first obtain the address and viewing key from the query message. Then, we need to check if the viewing key is valid, and handle the success and error outcomes.

<details> <summary> Hint 1:</summary>

We should utilize the method we just defined in msg.rs. 
</details>

<details> <summary> Hint 2:</summary>

The ViewingKey struct has an associated function called `check` which verifies the viewing key.
</details>


<details> <summary> Solution: </summary>

```rust
    let q_response = match msg {
        QueryMsg::AllInfo { .. } => {
            let (address, validated_key) = msg.get_validation_params(deps.api)?;
            let result = ViewingKey::check(deps.storage, address.as_str(), validated_key.as_str());
            match result.is_ok() {
                true => query_all_info(deps, address),
                false => Err(StdError::generic_err("Wrong viewing key for this address or viewing key not set")),
            }
        },
        QueryMsg::AmIRichest { .. } => {
            let (address, validated_key) = msg.get_validation_params(deps.api)?;
            let result = ViewingKey::check(deps.storage, address.as_str(), validated_key.as_str());
            match result.is_ok() {
                true => query_richest(deps, address),
                false => Err(StdError::generic_err("Wrong viewing key for this address or viewing key not set")),
            }
        },
        // ...
    };
    to_binary(&q_response?)
```
</details>

Note that the code above has repeated parts. We wrote it this way for clarity for this lesson, but you should normally modularize your code. In this case, because both queries accept the same arguments, we can do this:

```rust
#[entry_point]
pub fn query(deps: Deps, env: Env, msg: QueryMsg) -> StdResult<Binary> {
    let q_response = match msg {
        QueryMsg::AllInfo { .. } => handle_viewing_key_query(deps, &msg, query_all_info),
        QueryMsg::AmIRichest { .. } => handle_viewing_key_query(deps, &msg, query_richest),
    };

    to_binary(&q_response?)
}

fn handle_viewing_key_query(
    deps: Deps,
    msg: &QueryMsg,
    query_fn: fn(Deps, Addr) -> StdResult<Binary>,
) -> StdResult<Binary> {
    let (address, validated_key) = msg.get_validation_params(deps.api)?;
    let result = ViewingKey::check(deps.storage, address.as_str(), validated_key.as_str());
    match result.is_ok() {
        true => query_fn(deps, address),
        false => Err(StdError::generic_err(
            "Wrong viewing key for this address or viewing key not set",
        )),
    }
}
```

Imagine if there were a large number of queries, consisting of non-authenticated queries, viewing key queries and permit queries. Separating these three types of queries would be immensely helpful.


## Implementing query permits

Query permits make use of digital signatures to validate a query. The account wishing to grant access creates a permit message that includes important information such as the permissions granted, tokens, and permit name. This message is signed by the account owner, creating a cryptographic digital signature that is infeasible to forge without access to the private key. This whole process is done off-chain. The permit iself is a data structure that contains the digital signature, the plaintext permit message, and the public key of the signer.

When a user wishes to make a permit query, they send this permit along with the query message. The contract verifies the signature against the message and public key of the signer. If it is valid, it proceeds with the private query by executing the query message. 

If you want to understand more on how all these work, we have an in-depth discussion in our viewing key and permit pathway.

Now, let's implement query permits. 

### msg.rs

At this point, the only query messages the contract accepts are the two viewing key queries. So, let's define the permit query messages that our contract can accept.

**Exercise**

Add a new variant to the QueryMsg enum called `WithPermit`. It should accept two fields: `permit` and `query`.

```rust
pub enum QueryMsg {
    AllInfo { 
        addr: Addr,
        key: String,
    },
    AmIRichest {
        addr: Addr,
        key: String,
    },
    //
    // complete code here
    // 
}
```

Additionally, we have two other enums that are incomplete. These should give you hints on what types the fields in `WithPermit` should have.

```rust
pub enum QueryWithPermit {
    //
    // complete code here
    //
}

pub enum RichieRichPermissions {
    //
    // complete code here
    //
}
```
<details> <summary> Hint 1: </summary>
The QueryWithPermit type defines the set of query messages that our contract can accept when a permit is provided. In our case, we support two query messages: AllInfo and AmIRichest. 
</details>

<details> <summary> Hint 2: </summary>
The Permit type is a generic type that represents a permit for a specific set of permissions. We don't want the default permissions that are used for SNIPs, such as balance and history. Instead, we want our custom permissions to determine which of the two queries is allowed. We use the RichieRichPermissions enum to define this set of custom permissions that our contract supports.
</details>

<details> <summary> Solution: </summary>

```rust
pub enum QueryMsg {
    //...
    WithPermit {
        permit: Permit<RichieRichPermissions>,
        query: QueryWithPermit,
    },
}

pub enum QueryWithPermit {
    AllInfo {  },
    AmIRichest {  },
}

pub enum RichieRichPermissions {
    AllInfo,
    AmIRichest,
}
```

</details>

In our solution, RichieRichPermissions defines which of the two queries is allowed with a given permit. So, if the permit has the AmIRichest permission, the caller cannot query `am_i_richest`.

An alternative design is to have two variants along the lines of `AnyQuery` and `ResultOnly`. The first permission allows the caller to perform either query, while the second only allows the `am_i_richest` query. 

### contract.rs

Now let's look at contract.rs. The first thing to do is to add our new variant to the match arm in the query entry point.

```rust
#[entry_point]
pub fn query(deps: Deps, env: Env, msg: QueryMsg) -> StdResult<Binary> {
    let q_response = match msg {
        // ...

        //
        // complete code here
        // 
    };

    to_binary( /* complete code here */ "placeholder")
}
```

<details> <summary> Hint 1: </summary>
This arm should call the `permit_queries` function. 
</details>

Now let's complete the permit_queries function, which has the bulk of the logic required to process permit queries.

```rust
fn permit_queries(deps: Deps, env: Env, permit: Permit /* add generic */, query: QueryWithPermit) -> (/* complete code here */) {
    // Validate permit content
    let contract_address = env.contract.address;
        //
        // complete code here
        // 

    // Permit validated! We can now execute the query.
        //
        // complete code here
        // 

}
```
<details> <summary> Hint 2: </summary> The permit argument should have a generic type parameter for RichieRichPermissions. This specifies that the permit is for the RichieRichPermissions type. </details>

<details> <summary> Hint 3: </summary> To validate the permit content, we can use the secret_toolkit::permit::validate function. This function takes in several arguments including deps, PREFIX_REVOKED_PERMITS, &permit, contract_address.into_string(), and None. It returns the account associated with the permit if validation is successful. </details>

<details> <summary> Hint 4: </summary> After the permit is validated, we can execute the query by matching on the query argument. For each variant of the QueryWithPermit enum, we need to check if the permit has the required permission using the check_permission method. If it does, we can call the appropriate query function. If it does not, we can return an error indicating that the permit does not have the required permission. </details>


<details> <summary> Solution: </summary>

```rust
fn permit_queries(deps: Deps, env: Env, permit: Permit<RichieRichPermissions>, query: QueryWithPermit) -> StdResult<QueryAnswer> {
    // Validate permit content
    let contract_address = env.contract.address;

    let account = secret_toolkit::permit::validate(
        deps,
        PREFIX_REVOKED_PERMITS,
        &permit,
        contract_address.into_string(),
        None,
    )?;

    // Permit validated! We can now execute the query.
    match query {
        QueryWithPermit::AllInfo {} => {
            if !permit.check_permission(&RichieRichPermissions::AllInfo) {
                return Err(StdError::generic_err(format!(
                    "No permission to query, got permissions {:?}",
                    permit.params.permissions
                )));
            }

            query_all_info(deps, deps.api.addr_validate(&account)?)
        }
        QueryWithPermit::AmIRichest {  } => {
            if !permit.check_permission(&RichieRichPermissions::AmIRichest) {
                return Err(StdError::generic_err(format!(
                    "No permission to query, got permissions {:?}",
                    permit.params.permissions
                )));
            }

            query_richest(deps, deps.api.addr_validate(&account)?)
        }
    }
}
```
</details>


## Redeploying contract

Our contract is now complete. Let's make sure it compiles, then redeploy it to our local blockchain.

In your second terminal, run the following commands:
```sh
# optional first line
cargo check && make test

# compile and compress the contract into a wasm bytecode
make build

# run the shellscript to upload and instantiate an updated contract
./scripts/create_secret_box.sh
```

> **Stuck? Tests failing? Not compiling?** 
>
> The complete contract and front-end app code can be found in the app/solutions folder.


The shellscript additionally changes the environment variables (such as SECRET_BOX_ADDRESS) so our other scripts, such as the frontend app, will interact with the new contract.

(Optional) You can interact with the contract using secretcli if you want. For example, you can have two users input their networth and perform a viewing key query. 

```sh
docker exec localsecret secretcli tx compute execute $  
TODO
```

We know if a caller sends the message `all_info` and `am_i_richest`, the contract will respond properly by first checking the validity of the viewing key, then responding with the private data if the key is valid.


## Revising the frontend



## Congratulations

Congratulations on completing this tutorial!

We at [Secret University](https://scrt.university) hope you've not only enjoyed working through the **Exercise** steps, but that you've also learned a bit of what Secret Contracts are all about.

## Further Reading

- [Viewing keys and permit pathway](TBC TODO)

- [Rust Book](https://doc.rust-lang.org/book/) or the [Rustlings](https://github.com/rust-lang/rustlings) course.

- Another fun way to learn Rust that's interactive is [Tour of Rust](https://tourofrust.com).

- Secret's CosmWasm is based on vanilla CosmWasm, but there are some differences due to the privacy capabilities of the network. However, the CosmWasm [docs](https://docs.cosmwasm.com/docs/1.0/) are still an excellent resource for anyone looking to develop smart contracts in the Cosmos ecosystem.

- For connecting the frontend to Secret Network and interacting, we recommend studying the [Secret.js](https://secretjs.scrt.network/) docs.

- If you're not familiar with Javascript or Typescript, we recommend these resources: 

    - [Learn Javascript Online](https://learnjavascript.online)
    - [Learn Typescript](https://www.typescriptlang.org/docs)


