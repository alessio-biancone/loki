source $__fish_data_dir/completions/git.fish

function __loki_query
    loki youtrack \
        --token perm:YWxlc3Npby5iaWFuY29uZQ==.NTUtNDk0.AAsmMB37h9paAxgOKFpDHBQeBku0Mv \
        --query "project: {C&AM - Intermediaries Experience & Network Management} -Unscheduled #Unassigned #{To Do}" \
        --endpoint https://prima-assicurazioni-spa.myjetbrains.com/youtrack/api \
    | sed -r -e 's#([^|]*)[|]([^|]*)[|]([^|]*)#\1/temp/\2\t\3#g'
end

# add git alias if not present already
git config --global --get alias.create-branch > /dev/null; or git config --global alias.create-branch "!git checkout -b"

# autocompletion for new git alias
complete -c git -n '__fish_git_using_command create-branch' -d "creates a branch using loki::youtrack as provider" -xka '(__loki_query)'
