from ctypes import cdll

lib = cdll.LoadLibrary("target/release/libextensive.dylib")

lib.process()

print("done!")
