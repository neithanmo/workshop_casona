
## Compile
```bash
cargo build --release
./target/release/signature
```
o tambien
### Signing
si queremos firmar un mensaje propio
```bash
cargo run -- sign "your message"
```
si firmamos el mensaje default
```bash
cargo run -- sign
```
### Verify
```bash
cargo run -- verify "my_signature" "my key" "optional message."
```
### Help
```bash
cargo run -- help
```
muestra algo como:
```bash
signature 0.1.0

USAGE:
    signature <SUBCOMMAND>

OPTIONS:
    -h, --help       Print help information
    -V, --version    Print version information

SUBCOMMANDS:
    help      Print this message or the help of the given subcommand(s)
    sign      
    verify    
```
