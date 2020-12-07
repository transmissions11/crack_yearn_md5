# crack_yearn_md5

This is a Rust project that is desgined to reverse engineer the password to the [Yearn v2 site](https://v2.yearn.finance) from a 4x md5 hashed version of it (`dbba1bfe930d953cabcc03d7b6ab05e`) visible in their JavaScript.

A copy of the chunk file from which the hash was extracted from [is saved in this repository](https://github.com/TransmissionsDev/crack_md5/blob/master/main_chunk_with_hash_from_yearn_src.js).

### Some hints were given by the Yearn team:

- It will be in this format: `xxxxxxxxxxxxxxxxx........................................................!1` (replace the 17 `x`s with an actual 17 character string)
- The 17 `x`s will only be chars in this set (can be repeated): `[b, d, e, i, l, m, o, s, t, u, -]`

### This project attempts to crack the hash in the following way:

- Takes the charset and creates all possible 17 letter combinations with it (this takes A LOT OF TIME)
- Appends `........................................................!1` to each combo and runs an md5 hash function 4 times on it like so: `md5(md5(md5(md5(combo))))`
- Checks each hash against: `dbba1bfe930d953cabcc03d7b6ab05e`, if it matches, the program exits and logs out the unhashed password.
