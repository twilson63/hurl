#!/usr/bin/env bash

_hurl_completions() {
  local cur prev opts
  cur="${COMP_WORDS[COMP_CWORD]}"
  prev="${COMP_WORDS[COMP_CWORD-1]}"
  
  if [[ "$prev" == "hurl" ]]; then
    opts="get post put delete patch head options --version --help"
    COMPREPLY=($(compgen -W "${opts}" -- ${cur}))
    return 0
  fi
  
  if [[ "$prev" == "-H" ]] || [[ "$prev" == "--header" ]]; then
    opts="Content-Type: Authorization: Accept: User-Agent: Accept-Language: Accept-Encoding:"
    COMPREPLY=($(compgen -W "${opts}" -- ${cur}))
    return 0
  fi
  
  if [[ "$prev" == "-o" ]] || [[ "$prev" == "--output" ]]; then
    _filedir
    return 0
  fi
  
  if [[ "$prev" == "-d" ]] || [[ "$prev" == "--data" ]]; then
    COMPREPLY=()
    return 0
  fi
  
  if [[ "$prev" == "-u" ]] || [[ "$prev" == "--user" ]]; then
    COMPREPLY=()
    return 0
  fi
  
  if [[ "$prev" == "--timeout" ]]; then
    COMPREPLY=()
    return 0
  fi
  
  if [[ "$prev" == "-b" ]] || [[ "$prev" == "--cookie" ]]; then
    COMPREPLY=()
    return 0
  fi
  
  if [[ "$cur" == -* ]]; then
    opts="
      --header
      --data
      --user
      --password
      --timeout
      --cookie
      --output
      --insecure
      --follow
      --max-redirects
      --proxy
      --cert
      --key
      --cacert
      --cert-type
      --key-type
      --verbose
      --silent
      --show-headers
      --colorize
      --no-colorize
      --pretty
      --raw
      --json
      --form
      --multipart
      --auth
      --auth-type
      --compressed
      --request
      --method
      --url
      --version
      --help
    "
    COMPREPLY=($(compgen -W "${opts}" -- ${cur}))
    return 0
  fi
  
  if [[ "$cur" == http* ]] || [[ "$cur" == ftp* ]]; then
    COMPREPLY=()
    return 0
  fi
  
  _filedir
}

complete -o bashdefault -o default -o nospace -F _hurl_completions hurl
