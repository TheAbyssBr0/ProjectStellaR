# ProjectStellaR
A stateless password manager that uses the Argon2 hashing algorithm for your terminal!

#### Main menu screenshot
![image](https://github.com/TheAbyssBr0/ProjectStellaR/assets/63530018/63198fc6-34b5-4ffd-979b-346559ebbb96)

#### Service auto-complete feature screenshot
![image](https://github.com/TheAbyssBr0/ProjectStellaR/assets/63530018/89d963ac-8119-4f80-8dc7-0a5762a7e5e6)



### What is a stateless password manager?
A password manager that does not store the passwords the user wishes to retrieve, but instead generates them on demand using a secure hashing algorithm (in this case Argon2). The biggest advantage of this over a traditional self-hosted password manager is that even after a catastrophic dataloss, all the passwords can be recovered as long as the inital parameters to are the same (think: username, password, etc.). This also means that you can access your passwords from anywhere as long as this software runs on the machine without needing to transfer your encrypted database of passwords. You also don't need to trust a massive 3rd party company who most likely have a bigger target on their back.

### That's pretty cool but what are the downsides?
A potential minor annoyance is the necessity to set up the accounts with the unique (and strong) password this password manager generates for you. 

To streamline this process, the password manager lets you select the properties of the password you want to generate i.e. password length, types of characters you want in your passwords (lowercase, uppercase, numbers, symbols). It even lets you have different passwords for the same service with a "password number" which it remembers for you. I personally like this byproduct as it makes me set up a password for any new service I sign up to with a strong password.

### Features of Stellar PassMan
* Argon2 is used as the hashing algorithm which does not run on GPUs and thus is not prone to dictionary attacks from someone with a lot of GPUs.
* The master password, the password you use to access all your other passwords, is zeroized after computation and is only accepted as input right before computation.
  - it is never stored in RAM for significant periods of time
  - this ensures that the security is on parity with more traditional password managers which also zeroize passwords after authentication.
* Auto-complete for services
  - It will remember anything you logged into using it previously and it will suggest them if you're typing out a similar name. Press arrow keys to select and Tab to complete!
* New users are asked for password confirmation so it should in theory catch typos so that account with wrong (and unknown) credentials cannot be made at the time of creation.
* Runs in a terminal
* Select password properties
  - Lengths can be anywhere between 4 and 255 although, 16 is highly recommended (it is default and you won't have to manually set it)! Any greater will most likely not be necessary but the choice is yours.
  - The password can be made up of any combination of uppercase, lowercase, numeric, and symbol characters. The choice (again) is yours!
* Password number
  - Still use Yahoo!? Did it get hacked again? That's alright just increment the password number and generate a password with this new number. It automatically becomes the default after first use and now anytime you want to retrieve your unleaked Yahoo! password, just type in Yahoo! in the services input.
* Authentication.
  - It will remind you if you type in your password incorrectly (or differently from the set password). Of course it doesn't save your password (that wouldn't be stateless) but it'll know when username and password doesn't match! 

### How do I use this password manager?
You can compile it yourself using cargo with `$ cargo build --release` or `$ cargo run -- release` in the project home directory (binary placed in path: `./target/release`.

Or you can download one of the precompiled binaries from the release section.

## Warning:
This is a powerful tool and it is possible to set up a super long and complicated and weird password and then forget the initial conditions that made that password, making it irrecoverable. Don't do this. Or at least try not to.

## What's with the name?
This is loosely based on ProjectStella (deleted and forgotten) which had a similar concept to this but development halted when my fellow collaborator and I got busy with our university courses. It also had an amazing GUI (special thanks to showvikbiswas on github) which I neither have the need for or will to recreate for this version. The R kinda looks like the number 2 and it is also written purely in Rust there ProjectStellaR!
