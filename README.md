# pdubs
a password manager from scratch #ThanksWeirdAl

# how to run
1. validate rust and cargo are installed
2. clone repo
3. navigate to `src` folder
4. run ```cargo build --release```
5. change permission on the artifact from the previous command... this is found `target/release/`: ``` chmod +x target/release/<file name here> ```
6. navigate the `release` folder
7. run ```./<file name here>```
`optionally add path to terminal profile to run from anywhere`

# expected features:
1. a generator - poc in develop
2. a vault
3. encryption?
-----
# future thoughts:
1. mfa
2. backup
3. strength checker
4. history
5. auto-fill
6. cross-platform ui?
-----
# further down:
1. tagging/org'ing
2. usage reporting
3. notes
4. sharing?

# generation thoughts:
1. Length - default to 24 characters but length must be adjustable
2. Complexity - default to MAX complexity but complexity must be adjustable
3. Uniqueness - every generation must produce something unique

Vault Thoughts: TBD

Encryption Thoughts: TBD

MFA Thoughts: TBD

Backup Thoughts: TBD

Strength Checker Thoughts: TBD

History Thoughts: TBD

AutoFill Thoughts: TBD

UI Thoughts: TBD

Organization Thoughts: TBD

Reporting Thoughts: TBD

Note Taking Thoughts: TBD

Sharing Thoughts: TBD
