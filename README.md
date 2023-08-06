# Login Flow Function

## Overview

This is a simple flow function written in Rust that responds to HTTP requests with a login status. The function takes `username` and `password` as query parameters in the request and returns a JSON response indicating the login success or failure status.

## Prerequisites

Before you can use this flow function, make sure you have the following installed:

- Rust programming language: https://www.rust-lang.org/learn/get-started
- Cargo package manager (comes with Rust): https://doc.rust-lang.org/cargo/getting-started/installation.html
- `curl` command-line tool (optional, for testing)
- Not a prerequisite, but I did use Postman for testing this out: https://www.postman.com/

## Setup

1. Clone this repository to your local machine:

   ```bash
   git clone https://github.com/Zekrom-7780/flow_functions_rust
   cd flow_functions_rust
   ```
2. Build the Rust project:
   
```bash
   cargo build --release
   ```
## Usage

Once the flow function is deployed, you can use it by making HTTP requests to its endpoint. The function expects the username and password as query parameters in the request.
Example Request:

Make an HTTP GET request using tools like curl, Postman, or web browsers:
```bash
   curl -v 'https://your-flow-function-url?username=abc&password=xyz'
   ```

Now, for this function ,the flow function URL is : https://code.flows.network/lambda/5xcbnHixwT

So, the curl function becomes 
```bash
   curl -v 'https://code.flows.network/lambda/5xcbnHixwT?username=abc&password=xyz'
   ```




