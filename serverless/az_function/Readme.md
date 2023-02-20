# Azure Function for Open Idempotency

https://learn.microsoft.com/en-us/azure/azure-functions/create-first-function-vs-code-other?tabs=rust%2Cwindows

## Complie custom handler

Create a file at .cargo/config. Add the following contents and save the file.

[target.x86_64-unknown-linux-musl]
linker = "rust-lld"

In the integrated terminal, compile the handler to Linux/x64. A binary named handler is created. Copy it to the function app root.
Bash

rustup target add x86_64-unknown-linux-musl
cargo build --release --target=x86_64-unknown-linux-musl
cp target/x86_64-unknown-linux-musl/release/handler .

If you are using Windows, change the defaultExecutablePath in host.json from handler.exe to handler. This instructs the function app to run the Linux binary.

Add the following line to the .funcignore file:

target

This prevents publishing the contents of the target folder.