const path = require("path");
const ffi = require("ffi-napi");

// Pick the correct library file name depending on the platform
const libPath = path.resolve(__dirname, "../target/debug", {
  darwin: "libsubxt_ffi.dylib",
  linux:  "libsubxt_ffi.so",
  win32:  "subxt_ffi.dll"
}[process.platform]);

// Declare the FFI interface
const lib = ffi.Library(libPath, {
  "do_transfer": ["int", ["string", "uint64"]]
});

function doTransfer(destHex, amount) {
  const code = lib.do_transfer(destHex, amount);
  if (code === 0) {
    console.log("✓ transfer succeeded");
  } else {
    console.error("✗ transfer failed, code =", code);
  }
}

// Example usage:
const dest    = "0x8eaf04151687736326c9fea17e25fc5287613693c912909cb226aa4794f26a48";
const amount  = 1_000_000_000_000;            // fits in u64
doTransfer(dest, amount);
