# Serverless Swarm Toolkit

## 🚀 Overview

**Serverless Swarm Toolkit** is a command-line tool that lets you generate production-ready AWS serverless microservice stacks from a simple high-level YAML or JSON spec. Built for use with AWS CDK (initially), it scaffolds services with best practices, wiring together Lambda functions, API Gateway routes, DynamoDB tables, and more.

---

## 🎯 Features

- 📄 **YAML/JSON Spec Input**
- ⚙️ **Generates CDK App Structure**
- 🧠 **Best Practice Defaults**
- 📊 **Monitoring and Logging Included**
- 🔒 **Least Privilege IAM Roles**
- 🧪 **Basic Unit Test Stubs**
- 📦 **Plugin-Ready Design**
- 🦀 **Experimental Rust Runtime Support**

---

## 📦 Example Input Spec

```yaml
services:
  user-service:
    runtime: nodejs18.x
    endpoints:
      - path: /users
        method: GET
        lambda: getUsers
    database: usersTable
  order-service:
    runtime: rust
    endpoints:
      - path: /orders
        method: POST
        lambda: create_order
    database: ordersTable
```

---

## 🦀 Rust Runtime Support (Experimental)

To use Rust in your microservice:
- Add `runtime: rust` in your service spec
- Toolkit will generate a Rust project scaffold using `cargo-lambda`
- It compiles to a Lambda-compatible binary and zips it

### Example Folder Output:
```
order-service/
├── Cargo.toml
├── src/
│   └── main.rs
└── bootstrap (compiled binary)
```

### Scaffolded `main.rs` Example:
```rust
use lambda_http::{run, service_fn, Body, Error, Request, Response};

async fn function_handler(_event: Request) -> Result<Response<Body>, Error> {
    Ok(Response::new("Hello from Rust Lambda!".into()))
}

#[tokio::main]
async fn main() -> Result<(), Error> {
    run(service_fn(function_handler)).await
}
```

### Scaffolded `Cargo.toml`:
```toml
[package]
name = "create_order"
version = "0.1.0"
edition = "2021"

[dependencies]
lambda_http = "0.8"
tokio = { version = "1", features = ["macros", "rt-multi-thread"] }
```

### Requirements:
- Install [`cargo-lambda`](https://github.com/cargo-lambda/cargo-lambda):
```bash
cargo install cargo-lambda
```
- Compile with:
```bash
cargo lambda build --release --output-format zip
```

---

## 🛠 Usage

```bash
# Install
pip install serverless-swarm-toolkit  # or npm i -g serverless-swarm-toolkit

# Generate project
swarmgen generate -i spec.yaml -o ./my-service
```

This generates:
- CDK project with multiple stacks (1 per service)
- Lambda handler stubs (JS, Python, or Rust)
- DynamoDB table definitions
- IAM policies
- API Gateway routing
- CloudWatch log groups and alarms

---

## 📁 Output Folder Structure

```
my-service/
├── cdk.json
├── bin/
│   └── app.ts
├── lib/
│   ├── user-service-stack.ts
│   └── order-service-stack.ts
├── lambdas/
│   ├── getUsers/index.js
│   └── create_order/  # Rust project
│       ├── Cargo.toml
│       └── src/main.rs
└── templates/
    └── ...
```

---

## 🔮 Roadmap

- [x] YAML-to-CDK generator
- [x] Rust Lambda runtime support
- [ ] CloudWatch dashboards and alerts
- [ ] Terraform backend support
- [ ] Step Functions, SQS/SNS support
- [ ] Web UI spec designer
- [ ] VSCode extension

---

## 👨‍💻 Contributing

Pull requests are welcome! Please open an issue first to discuss major changes.

To run locally:
```bash
git clone https://github.com/HussainAther/serverless-swarm-toolkit.git
cd serverless-swarm-toolkit
pip install -e .  # or npm install
yarn dev          # if using JS
```

---

## 📄 License

MIT License

