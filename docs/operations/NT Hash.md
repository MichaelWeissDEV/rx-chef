# NT Hash

An NT Hash, sometimes referred to as an NTLM hash, is a method of storing passwords on Windows systems. It works by running MD4 on UTF-16LE encoded input. NTLM hashes are considered weak because they can be brute-forced very easily with modern hardware.

- Input: `String`
- Output: `String`
- CLI: `rxchef run "NT Hash"`
- Arguments: none

