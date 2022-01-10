# RisingWave
[![codecov](https://codecov.io/gh/singularity-data/risingwave/branch/main/graph/badge.svg?token=C5ZX0L0GWK)](https://codecov.io/gh/singularity-data/risingwave)
## Download
Run:
```shell
git clone https://github.com/singularity-data/risingwave.git
```

## Environment
* OS: macOS, Linux, or Windows (WSL or Cygwin)
* Java 11
* Rust
* Go
* CMake
* Protocol Buffers
* OpenSSL
* PostgreSQL (psql) (>= 14.1)

To install compilers in macOS, run:
```shell
brew install java11
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
brew install golang
brew install cmake
brew install protobuf
brew install openssl
```
Note that we only tested our code against Java 11. So please use the specific version!

## Development
You should have already seen multiple folders in our repo:
- The `java` folder contains the system's frontend code. The frontend includes parser, binder, planner,
optimizer, and other components. We use Calcite to serve as our query optimizer.
- The `rust` folder contains the system's backend code. The backend includes the streaming engine, OLAP
engine, storage engine and meta service.
- The `e2e_test` folder contains the latest end-to-end test cases.


### RiseLAB

RiseLAB is the new tool for developing RisingWave. You'll need to install tmux (>= 3.2a) beforehand.

```
brew install tmux wget
```

Then, simply run:

```
./riselab d # shortcut for ./riselab dev
./riselab ci-1node
./riselab ci-3node
./riselab dev-compute-node # compute node will need to be started by you
```

Everything will be set for you.

To stop the playground,

```
./riselab k # shortcut for ./riselab kill
```

And you can configure components for RiseLAB.

```
./riselab configure
```

For more information, refer to `README.md` under `rust/riselab`.

### Dashboard

To preview the web page, install Node.js, and

```
cd rust/meta/src/dashboard && npx reload -b
```

## Deployment
To run the system, you need to use at least four terminals:
- The Postgres shell is responsible for sending user command to the frontend server and displaying
results returned by the database.
- The frontend server receives the user command, performs parsing, binding, planning, optimization,
and then passes the physical plan to the corresponding to compute server(s).
- The compute server performs computation and then returns results to the frontend server.
- The meta server performs epoch, id generation and metadata management.

To start the meta server, create one terminal and then type:
```shell
make rust_build ## Not necessary if previously built
cd rust
./target/debug/meta-node --log4rs-config config/log4rs.yaml
```

To start the frontend server, create one terminal and then type:
```shell
cd java
./gradlew -p pgserver run 
# Or it can be specified by the parameter  ./gradlew -p pgserver run --args="-c ./src/main/resources/server.properties"
```

To start the compute server, create one terminal and then type:
```shell
make rust_build
cd rust
```
You can choose to use `minio` as the serving remote storage.
```shell
./target/debug/compute-node --log4rs-config config/log4rs.yaml
# With hummock state store
./target/debug/compute-node --log4rs-config config/log4rs.yaml --state-store hummock+minio://key:secret@localhost:2333/bucket
```
Alternatively, you can choose to use `Amazon S3` as the serving remote storage.
```shell
# With Amazon S3 and test accounts, follow https://singularity-data.larksuite.com/docs/docuszYRfc00x6Q0QhqidP2AxEg to set up the test credentials.
./target/debug/compute-node --log4rs-config config/log4rs.yaml --state-store hummock+s3://s3-ut
```

```shell
# With tikv state store
tiup playground
cargo build --features tikv
./target/debug/compute-node --log4rs-config config/log4rs.yaml --state-store "tikv://<pd_address>"
```

```shell
# With rocksdb state store
cargo build --features rocksdb-local
./target/debug/compute-node --log4rs-config config/log4rs.yaml --state-store rocksdb_local:///tmp/default
```

To start the Postgres shell, create one terminal and then type:
```shell
psql -h localhost -p 4567 -d dev
```

Now you can use the Postgres shell to try out your SQL command!

## Contributing
Thanks for your interest in contributing to the project, please refer to the [CONTRIBUTING.md](https://github.com/singularity-data/risingwave/blob/main/CONTRIBUTING.md).

## Toolchain
Currently, we are using nightly toolchain `nightly-2021-11-30`. If anyone needs to upgrade
the toolchain, be sure to bump `rust-toolchain` file as well as GitHub workflow.

## Documentation

The Rust codebase is documented with docstring, and you could view the documentation by:

```shell
make rust_doc
cd rust/target/doc
open risingwave/index.html
```
