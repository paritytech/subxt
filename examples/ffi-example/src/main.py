#!/usr/bin/env python3
import os, sys, ctypes, platform
from ctypes import c_char_p, c_uint64, c_int

# The library name depends on the playform type
if platform.system() == "Linux":
    libname = "libsubxt_ffi.so"
elif platform.system() == "Darwin":
    libname = "libsubxt_ffi.dylib"
elif platform.system() == "Windows":
    libname = "subxt_ffi.dll"
else:
    raise RuntimeError(f"Unsupported platform: {platform.system()}")

# Load the library
lib_path = os.path.join(os.path.dirname(__file__), "..", "target", "debug", libname)
lib = ctypes.CDLL(lib_path)

# Tell ctypes about our function signature, the one we defined in the Rust library
lib.do_transfer.argtypes = (c_char_p, c_uint64)
lib.do_transfer.restype  = c_int

def do_transfer(dest_hex: str, amount: int) -> int:
    """
    Perform a transfer.
    dest_hex:  hex-string of the 32-byte AccountId (e.g. "0x...")
    amount:    integer amount (fits in u64)
    Returns 0 on success, –1 on error.
    """
    # ensure we pass a C-string pointer
    dest_bytes = dest_hex.encode("utf8")
    return lib.do_transfer(dest_bytes, amount)

if __name__ == "__main__":
    # example usage
    dest = "0x8eaf04151687736326c9fea17e25fc5287613693c912909cb226aa4794f26a48"
    amt  = 1_000_000_000_000
    code = do_transfer(dest, amt)
    if code == 0:
        print("✓ transfer succeeded")
    else:
        print("✗ transfer failed")
