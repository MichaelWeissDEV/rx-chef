This is tesseract-rs 0.4.0 (Apache-2.0), with two local compatibility fixes:

- Its FFI declarations link Tesseract statically on Windows. The native CI
  installs the `x64-windows-static` vcpkg archive, which cannot satisfy the
  original declarations' DLL import symbols.
- Its optional build dependency uses zip 9 instead of the vulnerable zip 8.
  The archive entry handling in build.rs follows zip 9's Result API.

Keep this patch aligned with upstream when updating tesseract-rs.
