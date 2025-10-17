#compdef hurl

_hurl_http_methods() {
  local methods=('get' 'post' 'put' 'delete' 'patch' 'head' 'options')
  _describe 'HTTP methods' methods
}

_hurl_headers() {
  local headers=(
    'Content-Type:Set request content type'
    'Authorization:Set authorization header'
    'Accept:Set accept header'
    'Accept-Encoding:Set encoding'
    'Accept-Language:Set language'
    'User-Agent:Set user agent'
    'Cache-Control:Set cache control'
    'Cookie:Set cookies'
    'X-Requested-With:Mark as XMLHttpRequest'
  )
  _describe 'Common Headers' headers
}

_hurl_auth_types() {
  local types=('basic' 'digest' 'bearer' 'oauth2')
  _describe 'Authentication Types' types
}

_arguments \
  '1: :_hurl_http_methods' \
  '2: :_files' \
  '(-H --header)'{-H,--header}'[Add header]:header:' \
  '(-d --data)'{-d,--data}'[Send data]:data:' \
  '(-u --user)'{-u,--user}'[User credentials]:credentials:' \
  '(-p --password)'{-p,--password}'[User password]:password:' \
  '(-o --output)'{-o,--output}'[Output file]:file:_files' \
  '(-b --cookie)'{-b,--cookie}'[Send cookies]:cookies:' \
  '(-t --timeout)'{-t,--timeout}'[Timeout in seconds]:seconds:' \
  '(-f --follow)'{-f,--follow}'[Follow redirects]' \
  '(-i --show-headers)'{-i,--show-headers}'[Include headers in output]' \
  '(-I --show-response-headers)'{-I,--show-response-headers}'[Show response headers only]' \
  '(-L --location)'{-L,--location}'[Follow redirects]' \
  '(-k --insecure)'{-k,--insecure}'[Allow insecure SSL]' \
  '(--compressed)--compressed[Request compressed response]' \
  '(--proxy)--proxy[Use proxy]:proxy URL:' \
  '(--cert)--cert[Client certificate]:certificate file:_files' \
  '(--key)--key[Certificate key]:key file:_files' \
  '(--cacert)--cacert[CA certificate]:CA file:_files' \
  '(-m --method)'{-m,--method}'[HTTP method]:method:_hurl_auth_types' \
  '(--auth)--auth[Authentication type]:auth type:_hurl_auth_types' \
  '(-v --verbose)'{-v,--verbose}'[Verbose output]' \
  '(-s --silent)'{-s,--silent}'[Silent mode]' \
  '(--colorize)--colorize[Colorize output]' \
  '(--no-colorize)--no-colorize[No color output]' \
  '(--pretty)--pretty[Pretty print JSON]' \
  '(--raw)--raw[Raw output]' \
  '(--json)--json[JSON mode]' \
  '(--form)--form[Form data mode]' \
  '(-V --version)'{-V,--version}'[Show version]' \
  '(-h --help)'{-h,--help}'[Show help]'
