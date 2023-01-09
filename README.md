# Open-Idempotency-System
Main repository for the Open-Idempotency System 

## Setup

### Windows

1. Install cygwin with the following:
    - gcc-g++
    - gdb
    - cmake
    - make
    - bash
    - grep
2. Add C:\cygwin64\bin to your path
3. Install rust from [here](https://www.rust-lang.org/tools/install)
4. Download and unzip the windows protobuf-compiler [here](https://github.com/protocolbuffers/protobuf/releases/)
5. Take the unzipped file and place it at C:/protoc and add it to your path

### Linux

Run the following command:
```bash
apt install -y protobuf-compiler build-essential libssl-dev gcc g++  gdb ninja-build valgrind cmake libprotobuf-dev &&
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```



## Common Errors

### Failed to run custom build 

#### Proto file does not reside in any path
   Override the MAIN_PROTO_FILE environment variable with the absolute path of the proto files in the repository.
In a docker container this may be something like /src/protos/idempotency.proto
