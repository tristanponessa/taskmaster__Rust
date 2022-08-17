crate shellwords: parse cmd
cmdmust remain simple like in supervisorctl
no pipes, contenations,sell ops ....

crate std::process:Command launches the cmd 
states
"Note that the arguments are not passed through a shell, but given literally to the program. This means that shell syntax like quotes, escaped characters, word splitting, glob patterns, substitution, etc. have no effect."