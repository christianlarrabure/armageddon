# The Unofficial Armageddon Client

This is the WIP of the Unofficial Armageddon Client, a MUD client using Rust and Angular.

# Scripts

The Unofficial Armageddon Client supports lua scripting for hashtag commands. You should have a scripts folder where you have a scripts.json file like this:

`{ "help": { "src": "commands/help.lua" }, ... }`

In the above example, there is a command #help which leads to scripts/commands/help.lua.

## Hashtag Commands

Hashtag Commands are executed with the #verb args format. We enable several global variables and functions for your usage.

### Global Variables

#### verb

The verb variable determines what verb has been used to run this command. This is useful if you have several hashtag commands pointing to the same script.

#### argsstr

The argstr variable determines any arguments that were included after the verb.

### Global Functions

#### send(message)

Sends an input directly to Armageddon. This should be used carefully -- botting is forbidden by the game. It can be used for aliases, shortcuts, setting prompts, etc. When in doubt, ask staff.

#### tell(message)

Prints the message to the terminal of the user. This looks exactly like output sent by Armageddon.
