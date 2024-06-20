A tool used to ping list of domains

## Usage
```
$ echo www.facebook.com >> domains
$ echo www.google.com >> domains
$ echo www.x.com >> domains
$ echo non.existingdomain.com >> domains
$
$ cat domains | pinger -u -q
+ www.facebook.com
+ www.google.com
+ www.x.com
- non.existingdomain.com
```

## Help
```
Usage: pinger [OPTIONS]

Options:
  -v, --verbose            Verbose output
  -q, --quite              Quite mode, omits errors, and just output the results
  -t, --timeout <TIMEOUT>  Timeout [default: 5]
  -u                       Mark live/down hosts with +/-
  -r, --retry <RETRY>      The number of retries [default: 1]
  -h, --help               Print help
```
