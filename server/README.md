🐚 ShellOrd Server Cross-Platform written in Java

- Plugins: A collection of plugins. (.jar or .py). 

## Requirements

- Python plugins requires Jython Standalone 2.7.2.
- ShellOrd Server requires Java 11+.

## Bypass localhost + capture requests

https://shellord.mibaltoalex.com:60443

shellord.mibaltoalex.com => 127.0.0.1

## Grabber requests (GET / POST)

https://shellord.mibaltoalex.com:60443/grabber/info?cookie=captured

`2022-03-12 11:08:20.842 SHELLORD::grabber: Request captured: 'Host: 127.0.0.1 ; GET /info?cookie=captured`

## Web server - Login

![shellord-login](../_img/shellord-login.gif)

## Example

Example launch calculator on victim (Windows)

![shellord-calc](../_img/shellord_calc.gif)

## Snippets

All snippets are packed, compressed and encrypted in `.mar` (MAR; MIBALTOALEX ARchive) files.
A snippet can contain URIs, binary code, scripts (bash, powershell, etc) and plain text (.c, .py, .java, .rs, .sh, .rb, .bat, .txt, ...).
Example to use parameters in snippets: `echo ~[[param1]]; base64 -w0 ~[[file]]`
