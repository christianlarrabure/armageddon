function HelpGeneral()
  tell("Welcome to the official Armageddon Client. This is a listing of some useful commands you can use.")
  tell("#connect - Will let you connect to the game.")
  tell("#prompt - Sets your prompt.")
end

function HelpConnect()
  tell("#connect")
  tell("This command establishes a connection to the server. If you have lost connection, you can type it to re-connect as well.")
end

function HelpPrompt()
  tell("#prompt")
  tell("This command sets your prompt to the appropriate one. This is important to enable the stat bars.")
end

local _argstr = string.lower(argstr)

if string.len(argstr) == 0 then
  HelpGeneral()
elseif string.find(_argstr, "#connect") then
  HelpConnect()
elseif string.find(_argstr, "#prompt") then
  HelpPrompt()
else
  tell("We can't find any help on that topic.")
end
