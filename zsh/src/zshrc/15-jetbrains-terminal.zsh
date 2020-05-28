######################### START JETBRAINS TERMINAL HACK ########################
# Fix jetbrains terminal incorrectly set $0 to jetbrains-toolbox
#
# This bug affect linux only
# Reproduce steps
#   1) Start jetbrains-toolbox
#   2) Start any IDE
#   3) Open terminal panel
#   4) Run any command, $0 will be set to jetbrains-toolbox absolute path
#
# If we get back to this again, but problem still exist, let report to jetbrains.
#
if [[ -n $TOOLBOX_VERSION && $TERMINAL_EMULATOR == "JetBrains-JediTerm" ]]; then
    # This bug affect linux only
    if [[ $OSTYPE == linux* ]]; then
        case $JETBRAINS_TERMINAL_HACK in
            "")
                export JETBRAINS_TERMINAL_HACK=1
                exec $SHELL
                ;;
            1)
                export JETBRAINS_TERMINAL_HACK=2
                if [[ $0 == *jetbrains-toolbox ]]; then
                    exec $SHELL
                else
                    echo "Jetbrains terminal hack is no longer need"
                fi
                ;;
            2 | *)
                ;;
        esac
    fi
fi
########################## END JETBRAINS TERMINAL HACK #########################
