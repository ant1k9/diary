set -l _diary_commands add edit show
set -l _diary_edit_commands edit rm

function _diary_activities
    yq '.[] | .name' < ~/.config/diary/config.yaml | tr -d '"'
end

complete -f -c diary \
    -n "not __fish_seen_subcommand_from $_diary_commands help" \
    -a help

complete -f -c diary \
    -n "not __fish_seen_subcommand_from $_diary_commands" \
    -a add \
    -d "add record about activity"

complete -f -c diary \
    -n "not __fish_seen_subcommand_from $_diary_commands" \
    -a edit \
    -d "edit record"

complete -c diary \
    -n "not __fish_seen_subcommand_from $_diary_commands" \
    -a show \
    -d "show saved records for activity"

complete -f -c diary \
    -n "__fish_seen_subcommand_from $_diary_commands; and not __fish_seen_subcommand_from (_diary_activities)" \
    -a "(_diary_activities)"

complete -f -c diary \
    -n "__fish_seen_subcommand_from $_diary_commands" \
    -l "date"

complete -f -c diary \
    -n "__fish_seen_subcommand_from show" \
    -l "first"

complete -f -c diary \
    -n "__fish_seen_subcommand_from show" \
    -l "last"
