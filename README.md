# RustRestTemplate
Sample Rust Application that can accept REST requests using Rocket.rs

Configuration of the application parameters, such as `STAGE`,`PORT`, `WORKERS (no. of cpus * 2)`, etc. can be set through Enviromnemnt variables. 

Example:

```
ROCKET_ENV=production ROCKET_PORT=8008 ROCKET_WORKERS=4  cargo run
```

For more information on the different parameters visit the Rocket.rs [configuration guide](https://rocket.rs/v0.4/guide/configuration/)



#### Sample Requests


* Sample POST Request: 

```
curl -i -X POST -H "Content-Type: application/json"  -d '[{"name":"Mario Basa","age":40,"dept":"Production","skills":[{"id":1,"desc":"sing"},{"id":2,"desc":"dance"}]}]' http://localhost:8000/employee
```

* Sample GET Request:

```
curl -X GET http://localhost:8000/hello/mario/23
```

