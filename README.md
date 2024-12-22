# Fishpond Management System

## Overview

The **Fishpond Management System** is a comprehensive solution for managing fish farming operations. This system helps manage ponds, fish stocks, feed schedules, water quality monitoring, expenses, sales, and revenue calculations, ensuring an efficient and streamlined management process for fish farming businesses.

---

## Features

### Fishpond Management
- Register and manage multiple fishponds with details such as size, location, and type of fish.
- Track the total capacity and current stock of fish in each pond.

### Fish Stock Management
- Register and track fish stocks by species, weight, age, and pond assignment.
- Manage the movement of fish between ponds.

### Feeding Schedule
- Set and manage feeding schedules for ponds.
- Record feed amounts and feeding times for each pond.

### Water Quality Monitoring
- Log water quality parameters (e.g., pH, temperature, oxygen levels) for each pond.
- Analyze trends to ensure optimal conditions for fish growth.

### Sales and Revenue
- Record fish sales by weight, price per kilogram, and customer details.
- Calculate total revenue generated from fish sales.

### Expenses Tracking
- Record expenses for categories such as feed, maintenance, and labor.
- Calculate total expenses for individual ponds and the entire fish farming operation.

### Reports and Analytics
- Generate reports for revenue, expenses, and water quality trends.
- Calculate profit margins by comparing total revenue and expenses.

---
   
## Technologies Used
- **Rust**: Programming language for high-performance and memory-safe implementation.
- **IC Stable Structures**: Used for persistent storage of data.
- **Candid**: Interface definition for inter-canister calls and communication.

---

## Data Structures

### Core Entities
- **Fishpond**: Represents individual ponds with attributes such as size, location, and fish type.
- **FishStock**: Represents individual fish stocks in terms of species, weight, and age.
- **FeedingSchedule**: Stores feeding schedules, times, and feed amounts.
- **WaterQuality**: Logs water parameters like pH, temperature, and oxygen levels.
- **Sale**: Tracks details of fish sales, including revenue.
- **Expense**: Records operational expenses by category.

### Utility Enums
- **Message**: Handles system messages such as success, error, and validation responses.

---


## Requirements
* rustc 1.64 or higher
```bash
$ curl --proto '=https' --tlsv1.2 https://sh.rustup.rs -sSf | sh
$ source "$HOME/.cargo/env"
```
* rust wasm32-unknown-unknown targetz
```bash
$ rustup target add wasm32-unknown-unknown
```
* candid-extractor
```bash
$ cargo install candid-extractor
```
* install `dfx`
```bash
$ DFX_VERSION=0.15.0 sh -ci "$(curl -fsSL https://sdk.dfinity.org/install.sh)"
$ echo 'export PATH="$PATH:$HOME/bin"' >> "$HOME/.bashrc"
$ source ~/.bashrc
$ dfx start --background
```

If you want to start working on your project right away, you might want to try the following commands:

```bash
$ cd icp_rust_boilerplate/
$ dfx help
$ dfx canister --help
```

## Update dependencies

update the `dependencies` block in `/src/{canister_name}/Cargo.toml`:
```
[dependencies]
candid = "0.9.9"
ic-cdk = "0.11.1"
serde = { version = "1", features = ["derive"] }
serde_json = "1.0"
ic-stable-structures = { git = "https://github.com/lwshang/stable-structures.git", branch = "lwshang/update_cdk"}
```

## did autogenerate

Add this script to the root directory of the project:
```
https://github.com/buildwithjuno/juno/blob/main/scripts/did.sh
```

Update line 16 with the name of your canister:
```
https://github.com/buildwithjuno/juno/blob/main/scripts/did.sh#L16
```

After this run this script to generate Candid.
Important note!

You should run this script each time you modify/add/remove exported functions of the canister.
Otherwise, you'll have to modify the candid file manually.

Also, you can add package json with this content:
```
{
    "scripts": {
        "generate": "./did.sh && dfx generate",
        "gen-deploy": "./did.sh && dfx generate && dfx deploy -y"
      }
}
```

and use commands `npm run generate` to generate candid or `npm run gen-deploy` to generate candid and to deploy a canister.

## Running the project locally

If you want to test your project locally, you can use the following commands:

```bash
# Starts the replica, running in the background
$ dfx start --background

# Deploys your canisters to the replica and generates your candid interface
$ dfx deploy
```