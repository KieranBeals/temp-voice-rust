# Setup
Download the source code and run `cargo build --release` (this step take a while so do the next in the mean time)

Make an app at https://discord.com/developers/application and save the application ID for later
Create a bot in the app settings and save the token for later.
Invite the bot by using [this utility](https://scarsz.me/authorize) or making a link and replace the 0's with the application ID https://discord.com/oauth2/authorize?scope=bot+applications.commands&client_id=00000000000000000000

Set env variables: DATABASE_PATH and DISCORD_TOKEN
Run `cargo run --release` in the directory.

Now you should have a bot in your discord server that is **online** if that is the case you can run `/add-creator-channel` and fill in the arguments
Placeholders that you can use with template name include:
- `%number%` a number that starts with 1 for every voice channel (and will stay in order)
- `%name%` the owner of the channel's display name
- `%room%` a word that is a synonym to room that's first letter is the same as the user's display name's first letter
- `%current_activity%` the user's current activity when joining the voice channel

## Examples
`%name%'s %room%`

- DrJ.Sins's Domain
- Inbound's Inn
- Inbound's Island
- ⱤoᵀᴛᵥƝₓˣ's Room
- ⱤoᵀᴛᵥƝₓˣ's Resort
- ⱤoᵀᴛᵥƝₓˣ's Retreat

`Ranked %number%`

- Ranked 1
- Ranked 3
- Ranked 4
- Ranked 5
- Ranked 7
