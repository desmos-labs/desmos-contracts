# Contributing

- [Contributing](#contributing)
    - [Architecture Decision Records (ADR)](#architecture-decision-records-adr)
    - [Pull Requests](#pull-requests)
        - [Requesting Reviews](#requesting-reviews)
        - [Reviewing Pull Requests](#reviewing-pull-requests)
    - [Coding Style](#coding-style)
        - [Naming Conventions](#naming-conventions) 
        - [Execute/Query Actions](#executequery-actions)
        - [Messages](#messages)
        - [Field Types](#field-types)
        - [Instantiate/Execute Validation](#instantiateexecute-validation)
        - [Query](#query)
    - [Testing](#testing)
    - [Branching Model and Release](#branching-model-and-release)
        - [PR Targeting](#pr-targeting)
        - [Development Procedure](#development-procedure)
        - [Pull Merge Procedure](#pull-merge-procedure)

Thank you for considering making contributions to Desmos and related repositories!

Contributing to this repo can mean many things such as participating in discussion or proposing code changes. To ensure
a smooth workflow for all contributors, the general procedure for contributing has been established:

1. Either [open](https://github.com/desmos-labs/desmos-contracts/issues/new/choose) or
   [find](https://github.com/desmos-labs/desmos-contracts/issues) an issue you'd like to help with
2. Participate in thoughtful discussion on that issue
3. If you would like to contribute:
    1. If the issue is a proposal, ensure that the proposal has been accepted
    2. Ensure that nobody else has already begun working on this issue. If they have, make sure to contact them to
       collaborate
    3. If nobody has been assigned for the issue and you would like to work on it, make a comment on the issue to inform
       the community of your intentions to begin work
    4. Follow standard GitHub best practices: fork the repo, branch from the HEAD of `master`, make some commits, and
       submit a PR to `master`
        - For core developers working within the repo, to ensure a clear ownership of branches, branches must be
          named with the convention
          `{moniker}/{issue#}/branch-name`
    5. Be sure to submit the PR in `Draft` mode if you want to receive early feedback, even if it's incomplete as this
       indicates to the community you're working on something and allows them to provide comments early in the
       development process
    6. When the code is complete it can be marked `Ready for Review`

Note that for very small or blatantly obvious problems (such as typos) it is not required to an open issue to submit a
PR, but be aware that for more complex problems/features, if a PR is opened before an adequate design discussion has
taken place in a GitHub issue, that PR runs a high likelihood of being rejected.

Other notes:

- Looking for a good place to start contributing? How about checking out some
  [good first issues](https://github.com/desmos-labs/desmos-contracts/issues?q=is%3Aopen+is%3Aissue+label%3A%22good+first+issue%22)
- Please ensure that your code is lint compliant by running `cargo clippy`.

## Architecture Decision Records (ADR)

When proposing an architecture decision for a smart contract, please start by opening an
[issue](https://github.com/desmos-labs/desmos-contracts/issues/new/choose) or a
[discussion](https://github.com/desmos-labs/desmos-contracts/discussions/new) with a summary of the proposal. Once the proposal
has been discussed and there is rough alignment on a high-level approach to the design,
the [ADR creation process](https://github.com/desmos-labs/desmos-contracts/blob/master/docs/architecture/PROCESS.md) can begin. We
are following this process to ensure all involved parties are in agreement before any party begins coding the proposed
implementation. If you would like to see examples of how these are written, please refer to the current
[ADRs](https://github.com/desmos-labs/desmos-contracts/tree/master/docs/architecture) or to
[Cosmos ADRs](https://github.com/cosmos/cosmos-sdk/tree/master/docs/architecture).

## Coding Style

#### Naming Conventions

Each message must be in PascalCase like in the example below:

```rust
pub enum ExecuteMsg {
    IncrementCount {},
}
```

#### Execute/Query Actions

For each execute/query message a function that implements that functionality must be created.
In the case that two messages do similar operations it is allowed to create a single function for both actions.

### Messages

All the **execute** and **query** messages should be defined inside a `msg.rs` file.

#### Field Types

These are the types that we enforce for each use case:
* `String`: to represent an address, this is to force the address validation to convert an address to the `Addr` struct;
**NOTE**: This is just for the `Execute` and `Instantiate` messages. Inside the query responses we use the `Addr` type.
* `U\Int64\128`: to represent any number that is bigger than 32 bits,
this is to ensure any client that doesn’t have a proper support for such larger integer can handle them correctly.

#### Instantiate/Execute Validation

In order to keep the validation logic of the Execute/Instantiate messages in a single place we enforce that the 
messages must implement a `pub fn validate(&self) -> bool` method to make sure that the fields are correct.
For example, if a message have a start and end time to express a period of time, t
his method should ensure that the start time is before the end time. This method **should not** validate any address 
in the messages.

#### Query

For each query message we return an appropriate `QueryResponse` struct that contains the queried data. 
For example, if there's a `QueryAdmin` message, must be created a `QueryAdminResponse` struct containing 
the contract admin address.

## Pull Requests

PRs should be categorically broken up based on the type of changes being made (for example, `fix`, `feat`,
`refactor`, `docs`, and so on). The *type* must be included in the PR title as a prefix (for example,
`fix: <description>`). This convention ensures that all changes that are committed to the base branch follow the
[Conventional Commits](https://www.conventionalcommits.org/en/v1.0.0/) specification. Additionally, each PR should only
address a single issue.

### Requesting Reviews

In order to accommodate the review process, the author of the PR must complete the author checklist to the best of their
abilities before marking the PR as "Ready for Review". If you would like to receive early feedback on the PR, open the
PR as a "Draft" and leave a comment in the PR indicating that you would like early feedback and tagging whoever you
would like to receive feedback from.

### Reviewing Pull Requests

All PRs require at least two review approvals before they can be merged (one review might be acceptable in the case of
minor changes to docs or other changes that do not affect production code). The PR template has a reviewers checklist
that must be completed before the PR can be merged. Each reviewer is responsible for all checked items unless they have
indicated otherwise by leaving their handle next to specific items. In addition, use the following review explanations:

- `LGTM` without an explicit approval means that the changes look good, but you haven't thoroughly reviewed the reviewer
  checklist items.
- `Approval` means that you have completed some or all of the reviewer checklist items. If you only reviewed selected
  items, you must add your handle next to the items that you have reviewed. In addition, follow these guidelines:
    - You must also think through anything which ought to be included but is not
    - You must think through whether any added code could be partially combined (DRYed) with existing code
    - You must think through any potential security issues or incentive-compatibility flaws introduced by the changes
    - Naming must be consistent with conventions and the rest of the codebase
    - Code must live in a reasonable location, considering dependency structures (for example, not importing testing
      modules in production code, or including example code modules in production code).
    - If you approve the PR, you are responsible for any issues mentioned here and any issues that should have been
      addressed after thoroughly reviewing the reviewer checklist items in the pull request template.
- If you sat down with the PR submitter and did a pairing review, add this information in the `Approval` or your PR
  comments.
- If you are only making "surface level" reviews, submit any notes as `Comments` without adding a review.

## Testing

To ensure that the smart contracts behave correctly we enforce the presence of **unit tests**. In the case the contract
is also interacting with one or more contracts we enforce the presence of
**integration tests** to check if the state of the other contracts are coherent with the action performed from the
contract under test.

### Check success of an action

It is recommended to use `unwrap` over `assert!(r.is_ok())` to check that an action’s returned `Result` is `Ok`. Doing
this, more details will be returned when the `Result` is not `Ok` since the `unwrap` function will panic writing on the
test log the error. If necessary, after calling `unwrap`, the unwrapped structure can be checked using `assert_eq!`. It
is also **mandatory** to check that the contract has the expected state after the execution.

### Check failure of an action

It is recommended to use `unwrap_err` over `assert!(r.is_err())` to check that an action’s returned `Result` is `Err`.
Doing this, more details will be returned when the `Result` is not `Err` since the `unwrap_err` function will 
panic writing on the test log the occurred errors.
Where possible, after unwrapping the error, it is **mandatory** to check that the returned error is what the contract 
should return in that failure case.

### Single responsibility 
Each test should be scoped to a **single case**. For example if an execution action have 2 possible
cause of failure there should be 3 tests, 2 that tests the error cases and 1 to check the proper execution.

### Naming conventions
The tests name should describe explicitly in the title what it’s going to test and follow the following naming convention:

* Proper execution tests: `..._properly`;
* Failure tests: `..._error`.

## Branching Model and Release

User-facing repos should adhere to the [trunk based development branching model](https://trunkbaseddevelopment.com/).

### PR Targeting

Ensure that you base and target your PR on the `master` branch.

All feature additions and bug fixes should be targeted against `master`.

### Development Procedure

- the latest state of development is on `master`
- `master` must never fail `cargo test`
- `master` should not fail `cargo clippy`
- no `--force` onto `master` (except when reverting a broken commit, which should seldom happen)
- create a development branch either on github.com/desmos-labs/desmos-contracts, or your fork (using `git remote add origin`)
- before submitting a pull request, begin `git rebase` on top of `master`

### Pull Merge Procedure

- ensure pull branch is rebased on `master`
- run `cargo test` to ensure that all tests pass
- merge pull request
