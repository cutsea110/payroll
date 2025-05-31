# payroll-http

## Usage

Run payroll http server like as below:

```bash
$ RUST_LOG=trace cargo run -p payroll-http
```

And then, you make request as below:

```bash
$ curl -X POST \
       -H 'Content-Type: text/plain' \
	   -d $'AddEmp 1429 "Bob" "Home" S 3215.88\nPayday 2025-01-31' \
	   http://localhost:7878
```

