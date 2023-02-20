# Open-Idempotency-System

Main repository for the Open-Idempotency System 

This system is a cache for idempotency apis. You can use this system to determine if a call has already been made
but if a call has not finished, it is up to you to determine your own rollback, strategy.


## Concurrency

What if a request from a frontend is made to the backend is made and the backend takes a while due to something really slow.
The front end will have a timeout and attempt to make another request.

There are two possible scenarios:

The backend failed and the request should be attempted again
The backend is still processing and processing another request will break idempotency.

## Caching System - not a full idempotency solution

The idempotency system tracks the idempotency keys in a caching layer but may not track a token as finished if:

The server crashes and is unable to tell the idempotency system that it failed.

There many other issues than can occur to synchronizing the idempotency system with your database. For this reason, it is recommended
to never kill your app and instead let your application shut down properly. It is also recommended to not do anything between finishing a call
to the idempotency system and committing to your database.

## The idempotency lifecycle

1. check_or_insert - checks the idempotency token, if it is not found it will insert it and return None Status.
Possible statuses are:
```
   None = 0; // no token found, it is inserted
   In_Progress = 1; // possible failure
   Completed = 2; // request is completed and response data is given
   Failed = 3; // cetain theere is a fialure
```
1. (n times) save_stage - Save each stage of the request to track which part of your code failed. If you have a transaction, this may only be the commit stage.
   If the token is not started or compeletd and error will be returned, ERR_NOT_STARTED, or ERR_COMPLETED
2. Save the response of the request. This is used to that subsequent requests can just return the response data.
   Possible errors include: ERR_NOT_STARTED

## Detecting an active requests

In addition to the save states, if using the streaming api, we can deduce if your request is still processing or not. 
This is critical to avoid processing a request twice if the last request is still processing.

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
