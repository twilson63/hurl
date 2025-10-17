#!/usr/bin/env fish

set -l hurl_commands get post put delete patch head options

complete -c hurl -n '__fish_use_subcommand_from_list' -f -a 'get' -d 'Send a GET request'
complete -c hurl -n '__fish_use_subcommand_from_list' -f -a 'post' -d 'Send a POST request'
complete -c hurl -n '__fish_use_subcommand_from_list' -f -a 'put' -d 'Send a PUT request'
complete -c hurl -n '__fish_use_subcommand_from_list' -f -a 'delete' -d 'Send a DELETE request'
complete -c hurl -n '__fish_use_subcommand_from_list' -f -a 'patch' -d 'Send a PATCH request'
complete -c hurl -n '__fish_use_subcommand_from_list' -f -a 'head' -d 'Send a HEAD request'
complete -c hurl -n '__fish_use_subcommand_from_list' -f -a 'options' -d 'Send an OPTIONS request'

complete -c hurl -s h -l help -d 'Show help message'
complete -c hurl -s V -l version -d 'Show version'
complete -c hurl -s v -l verbose -d 'Verbose output'
complete -c hurl -s s -l silent -d 'Silent mode'

complete -c hurl -s H -l header -d 'Add header' -x
complete -c hurl -s d -l data -d 'Send data' -x
complete -c hurl -s u -l user -d 'User credentials' -x
complete -c hurl -s p -l password -d 'User password' -x
complete -c hurl -s o -l output -d 'Output file' -x

complete -c hurl -s b -l cookie -d 'Send cookies' -x
complete -c hurl -s t -l timeout -d 'Timeout in seconds' -x
complete -c hurl -s f -l follow -d 'Follow redirects'
complete -c hurl -s i -l show-headers -d 'Include headers in output'
complete -c hurl -s I -l show-response-headers -d 'Show response headers only'

complete -c hurl -s L -l location -d 'Follow redirects'
complete -c hurl -s k -l insecure -d 'Allow insecure SSL connections'
complete -c hurl -l compressed -d 'Request compressed response'
complete -c hurl -l proxy -d 'Use proxy server' -x

complete -c hurl -l cert -d 'Client certificate' -x
complete -c hurl -l key -d 'Certificate key' -x
complete -c hurl -l cacert -d 'CA certificate' -x

complete -c hurl -s m -l method -d 'HTTP method' -x -a 'GET POST PUT DELETE PATCH HEAD OPTIONS'
complete -c hurl -l auth -d 'Authentication type' -x -a 'basic digest bearer oauth2'

complete -c hurl -l colorize -d 'Colorize output'
complete -c hurl -l no-colorize -d 'Disable colorized output'
complete -c hurl -l pretty -d 'Pretty print JSON'
complete -c hurl -l raw -d 'Raw output'
complete -c hurl -l json -d 'JSON mode'
complete -c hurl -l form -d 'Form data mode'
