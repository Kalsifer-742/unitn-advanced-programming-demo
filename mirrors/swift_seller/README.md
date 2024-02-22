# Swift Seller

**Swift Seller** is a tool developed during the Advanced Programming course held by prof. Patrignani of the University of Trento. This tool will be presented at the Software Faire, for anyone who may be interested in "buying" it.

## Description

This tool automates the sale of items to markets, reducing the interaction one has with them to a single function call.

## Features

These are the features we intend to add to the tool:

- automatically sell items to markets
- choose which items to sell and which ones to keep
- return specific errors based on the circumstances

## Work In Progress

We build this tool incrementally, by adding small capabilities, one at a time:

- [x] Pull the library from the register and have a running robot
- [x] Detect when the robot is near a market
- [x] Sell everything the robot holds in its backpack
- [x] Filter for what we want to sell and what we prefer not to

All the while we also need to take into consideration:

- [x] Documentation
- [x] Errors returned to the user
  - as of right now we consider returning a `LibError::OperationNotAllowed` when the robot is not near a `Market` and `LibError::NotEnoughSpace` when the robot cannot hold the coins earned from the sale.

What more can we do:
- [ ] More testing!