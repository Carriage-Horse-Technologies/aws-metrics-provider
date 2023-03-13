# aws-metrics-provider
aws-metrics-provider

## Using

### Debug

```Shell
cargo install cargo-lambda
cargo lambda watch
```

### Release build

```Shell
cargo lambda build --release
```

### Deploy

require: [Release build](#Release-build)

```Shell
cargo lambda deploy --enable-function-url --env-file <ENV_FILE> --role <ROLE>
```

## env

```Shell
region={region}
instance_id={EC2のInstanceId}
```

## Sample json

```json
{
    "cpuutilization": 0.08476589448238055,
    "disk_read_bytes": 0.0,
    "disk_write_bytes": 0.0,
    "network_in": 537.8545454545455,
    "network_out": 406.25454545454545
}
```
