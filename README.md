# RustRestTemplate
Sample Rust Application that can accept REST requests using Rocket.rs

Configuration of the application parameters, such as `STAGE`,`PORT`, `WORKERS (no. of cpus * 2)`, etc. can be set through Enviromnemnt variables. For example:

```
ROCKET_ENV=production ROCKET_PORT=8008 ROCKET_WORKERS=4  cargo run
```

For more information on the different parameters visit the Rocket.rs [configuration guide](https://rocket.rs/v0.4/guide/configuration/)