# Use
First import sgx_new_edl at both your app side code and enclave side code.

copy the build.rs in directory app to your untrust side code.

copy the build.rs in directory enclave1 to your trust side code.

if your trust side code didn't named app,change the name in build.rs and Makefile.

if your trust side code didn't named enclave1,change the name in build.rs and Makefile.

use #[enclave(ecall)] to define ecall,and write the ecall in dir enclave, but not in lib.rs.

use #[enclave(ocall, enclave_name)] to define ocall bind to enclave_name.
## Build
```bash
make
```
## Run
```bash
cd bin && ./app
```

