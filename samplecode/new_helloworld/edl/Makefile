SGX_SDK ?= /opt/intel/sgxsdk
SGX_MODE ?= SW
SGX_ARCH ?= x64

TOP_DIR := ../incubator-teaclave-sgx-sdk
include $(TOP_DIR)/buildenv.mk

ifeq ($(shell getconf LONG_BIT), 32)
	SGX_ARCH := x86
else ifeq ($(findstring -m32, $(CXXFLAGS)), -m32)
	SGX_ARCH := x86
endif

ifeq ($(SGX_ARCH), x86)
	SGX_COMMON_CFLAGS := -m32
	SGX_LIBRARY_PATH := $(SGX_SDK)/lib
	SGX_ENCLAVE_SIGNER := $(SGX_SDK)/bin/x86/sgx_sign
	SGX_EDGER8R := $(SGX_SDK)/bin/x86/sgx_edger8r
else
	SGX_COMMON_CFLAGS := -m64
	SGX_LIBRARY_PATH := $(SGX_SDK)/lib64
	SGX_ENCLAVE_SIGNER := $(SGX_SDK)/bin/x64/sgx_sign
	SGX_EDGER8R := $(SGX_SDK)/bin/x64/sgx_edger8r
endif
Crypto_Library_Name := sgx_tcrypto
KeyExchange_Library_Name := sgx_tkey_exchange
ProtectedFs_Library_Name := sgx_tprotected_fs

RustEnclave_C_Files := $(wildcard ./enclave/*.c)
RustEnclave_C_Objects := $(RustEnclave_C_Files:.c=.o)
RustEnclave_Include_Paths := -I$(CUSTOM_COMMON_PATH)/inc -I$(CUSTOM_EDL_PATH) -I$(SGX_SDK)/include -I$(SGX_SDK)/include/tlibc -I$(SGX_SDK)/include/stlport -I$(SGX_SDK)/include/epid -I ./enclave -I./include

RustEnclave_Link_Libs := -L$(CUSTOM_LIBRARY_PATH) -lenclave
RustEnclave_Compile_Flags := $(SGX_COMMON_CFLAGS) $(ENCLAVE_CFLAGS) $(RustEnclave_Include_Paths)
RustEnclave_Link_Flags := -Wl,--no-undefined -nostdlib -nodefaultlibs -nostartfiles -L$(SGX_LIBRARY_PATH) \
	-Wl,--whole-archive -l$(Trts_Library_Name) -Wl,--no-whole-archive \
	-Wl,--start-group -lsgx_tstdc -l$(Service_Library_Name) -l$(Crypto_Library_Name) $(RustEnclave_Link_Libs) -Wl,--end-group \
	-Wl,--version-script=enclave/Enclave.lds \
	$(ENCLAVE_LDFLAGS)

Enclave_EDL_Files := Enclave_t.c Enclave_t.h 
CUSTOM_EDL_PATH := ../incubator-teaclave-sgx-sdk/sgx_edl/edl

$(Enclave_EDL_Files): $(SGX_EDGER8R) Enclave.edl
	$(SGX_EDGER8R) --trusted Enclave.edl --search-path $(SGX_SDK)/include --search-path $(CUSTOM_EDL_PATH) --trusted-dir .
	@echo "GEN  =>  $(Enclave_EDL_Files)"

enclave/Enclave_t.o: $(Enclave_EDL_Files)
	@$(CC) $(RustEnclave_Compile_Flags) -c Enclave_t.c -o $@
	@echo "CC   <=  $<"

# $(RustEnclave_Name): enclave enclave/Enclave_t.o
# 	@$(CXX) enclave/Enclave_t.o -o $@ $(RustEnclave_Link_Flags)
# 	@echo "LINK =>  $@"

.PHONY: all
all: enclave/Enclave_t.o