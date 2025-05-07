# Licensed to the Apache Software Foundation (ASF) under one
# or more contributor license agreements.  See the NOTICE file
# distributed with this work for additional information
# regarding copyright ownership.  The ASF licenses this file
# to you under the Apache License, Version 2.0 (the
# "License"); you may not use this file except in compliance
# with the License.  You may obtain a copy of the License at
#
#   http://www.apache.org/licenses/LICENSE-2.0
#
# Unless required by applicable law or agreed to in writing,
# software distributed under the License is distributed on an
# "AS IS" BASIS, WITHOUT WARRANTIES OR CONDITIONS OF ANY
# KIND, either express or implied.  See the License for the
# specific language governing permissions and limitations
# under the License.
Rust_Enclave_Name := libenclave.a
Rust_Enclave_Files := $(wildcard src/*.rs)
Rust_Target_Path := $(CURDIR)/../../../xargo

ifeq ($(MITIGATION-CVE-2020-0551), LOAD)
export MITIGATION_CVE_2020_0551=LOAD
else ifeq ($(MITIGATION-CVE-2020-0551), CF)
export MITIGATION_CVE_2020_0551=CF
endif

.PHONY: all

all: $(Rust_Enclave_Name)

$(Rust_Enclave_Name): $(Rust_Enclave_Files)
ifeq ($(XARGO_SGX), 1)
	RUST_TARGET_PATH=$(Rust_Target_Path) xargo build --target x86_64-unknown-linux-sgx --release
	cp ./target/x86_64-unknown-linux-sgx/release/libtest_enclave.a ../lib/libenclave.a
# else
# 	cargo build --release --verbose
# 	cp ./target/release/libtest_enclave.a ../lib/libenclave.a
# endif
else
	cargo build
	cp ./target/debug/libenclave1.a ../lib/libenclave1.a
endif
#   = note: "cc" "-Wl,--version-script=/tmp/rustcn70efR/list" "-m64" "/tmp/rustcn70efR/symbols.o" "/root/tee/test_enclave/target/release/deps/test_enclave.test_enclave.92037451-cgu.0.rcgu.o" "/root/tee/test_enclave/target/release/deps/test_enclave.test_enclave.92037451-cgu.1.rcgu.o" "/root/tee/test_enclave/target/release/deps/test_enclave.test_enclave.92037451-cgu.2.rcgu.o" "/root/tee/test_enclave/target/release/deps/test_enclave.test_enclave.92037451-cgu.3.rcgu.o" "/root/tee/test_enclave/target/release/deps/test_enclave.test_enclave.92037451-cgu.4.rcgu.o" "/root/tee/test_enclave/target/release/deps/test_enclave.test_enclave.92037451-cgu.5.rcgu.o" "/root/tee/test_enclave/target/release/deps/test_enclave.test_enclave.92037451-cgu.6.rcgu.o" "/root/tee/test_enclave/target/release/deps/test_enclave.4n1ard07vnfybl0u.rcgu.o" "-Wl,--as-needed" "-L" "/root/tee/test_enclave/target/release/deps" "-L" "/opt/intel/sgxsdk/sgxsdk/lib64" "-L" "/root/tee/test_enclave/target/release/build/sgx_backtrace_sys-def8748d2fb344cd/out/libbacktrace/" "-L" "/root/tee/test_enclave/target/release/build/sgx_backtrace_sys-def8748d2fb344cd/out/libbacktrace" "-L" "/root/tee/test_enclave/target/release/build/sgx_unwind-5967c78a3d058c31/out/libunwind/src/.libs" "-L" "/root/.rustup/toolchains/nightly-2022-10-22-x86_64-unknown-linux-gnu/lib/rustlib/x86_64-unknown-linux-gnu/lib" "-lsgx_urts_sim" "-Wl,-Bstatic" "/root/tee/test_enclave/target/release/deps/libsgx_serialize-39d7c35c5b29adbf.rlib" "/root/tee/test_enclave/target/release/deps/libsgx_tstd-ff7f97719f252dac.rlib" "/root/.rustup/toolchains/nightly-2022-10-22-x86_64-unknown-linux-gnu/lib/rustlib/x86_64-unknown-linux-gnu/lib/libpanic_unwind-15accacd1251d941.rlib" "/root/.rustup/toolchains/nightly-2022-10-22-x86_64-unknown-linux-gnu/lib/rustlib/x86_64-unknown-linux-gnu/lib/libunwind-757bd6fa410f1121.rlib" "/root/.rustup/toolchains/nightly-2022-10-22-x86_64-unknown-linux-gnu/lib/rustlib/x86_64-unknown-linux-gnu/lib/liblibc-3e961d059b9bcde7.rlib" "/root/.rustup/toolchains/nightly-2022-10-22-x86_64-unknown-linux-gnu/lib/rustlib/x86_64-unknown-linux-gnu/lib/libcfg_if-a51843dfc5fc8b4b.rlib" "/root/tee/test_enclave/target/release/deps/libsgx_tprotected_fs-1c6b8194cd995c38.rlib" "/root/tee/test_enclave/target/release/deps/libsgx_trts-93b731bcc2afaa7c.rlib" "/root/tee/test_enclave/target/release/deps/libsgx_libc-1f386b5358127e6c.rlib" "/root/tee/test_enclave/target/release/deps/libsgx_types-8c0737556bb8ff2c.rlib" "/root/tee/test_enclave/target/release/deps/libsgx_alloc-f76a2f294a589f87.rlib" "/root/tee/test_enclave/target/release/deps/libsgx_unwind-a53ff9a3f3274575.rlib" "/root/tee/test_enclave/target/release/deps/libhashbrown_tstd-59ec94415a526a28.rlib" "/root/.rustup/toolchains/nightly-2022-10-22-x86_64-unknown-linux-gnu/lib/rustlib/x86_64-unknown-linux-gnu/lib/liballoc-c6c03e024a2f1e46.rlib" 
#   "/root/.rustup/toolchains/nightly-2022-10-22-x86_64-unknown-linux-gnu/lib/rustlib/x86_64-unknown-linux-gnu/lib/librustc_std_workspace_core-522518611024dce5.rlib
#   " "/root/.rustup/toolchains/nightly-2022-10-22-x86_64-unknown-linux-gnu/lib/rustlib/x86_64-unknown-linux-gnu/lib/libcore-05898138a596088a.rlib" "/root/.rustup/toolchains/nightly-2022-10-22-x86_64-unknown-linux-gnu/lib/rustlib/x86_64-unknown-linux-gnu/lib/libcompiler_builtins-b7c79d85cf21a511.rlib" "-Wl,-Bdynamic" "-lsgx_tstdc" "-lgcc_s" "-lutil" "-lrt" "-lpthread" "-lm" "-ldl" "-lc" "-lsgx_trts" "-lsgx_pthread" "-lsgx_trts" "-lsgx_tstdc" "-Wl,--eh-frame-hdr" "-Wl,-znoexecstack" "-L" "/root/.rustup/toolchains/nightly-2022-10-22-x86_64-unknown-linux-gnu/lib/rustlib/x86_64-unknown-linux-gnu/lib" "-o" "/root/tee/test_enclave/target/release/deps/libtest_enclave.so" "-Wl,--gc-sections" "-shared" "-Wl,-zrelro,-znow" "-Wl,-O1" "-nodefaultlibs"