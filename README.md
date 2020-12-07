# crack_yearn_md5

This is a Rust project that was able to reverse engineer the password to the [Yearn v2 site](https://v2.yearn.finance) from a 2x md5 hashed version of it (`86566bae1bb08cfe94a58cd86f391b10`) shared by one of their developers.

However this version of the project is not the first iteration as the Yearn team gave out many hints during the time I was developing my original, more complex, solution. 

The version you are viewing now is the "easier" solution which takes advantage of all the hints. 

#### This program actually cracked the code which is now known to be: `its-time-to-build........................................................!1`

This "easier" solution is outlined below, but if you'd like to see my solution the harder version of this challenge (an 4x md5 hashed version with no knowlege about the first words, the length of the words or if it was made of valid english words), [please click here to the v1 branch.](https://github.com/TransmissionsDev/crack_yearn_md5/tree/v1)

### All hints given by the Yearn team:

- It will be in this format: `xxxxxxxxxxxxxxxxx........................................................!1` (replace the 17 `x`s with an actual 17 character string)
- The 17 `x`s will only be chars in this set (can be repeated): `[b, d, e, i, l, m, o, s, t, u, -]`
- The 17 `x`s will be replaced with 4 words, each seperated by a dash (`-`) : the first word will have 3 characters, the second will have 4, the third will have 2, and the fourth will have 5. 
- The first word is: "its"

### This project is able to crack the hash in the following way:

- Takes a txt file with all English words (each on a new line)
- Creates 3 lists of possible words for each of the 3 unknown word slots by checking for every word in the dictionary if its length matches up with the hinted length (as well as if all the characters in the word are in the charset `[b, d, e, i, l, m, o, s, t, u, -]`)
- Creates every possible combination of the first word and all the possible valid second, third and fourth words.
- Appends `........................................................!1` to each combo and runs an md5 hash function 2 times on it like so: `md5(md5(combo))`
- Checks each hash against: `86566bae1bb08cfe94a58cd86f391b10`, if it matches, the program exits and logs out the unhashed password.
