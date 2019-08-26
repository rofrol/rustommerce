- [ ] don't compile iron
- [ ] get settings from env variable and as lower priority, from .env
- [ ] don't return index.html for /favicon.ico and /api/badurl or even /hello/YourName

<pre>
Roman Frołow
@rofrol
May 20 10:08
This is how I enable debug in actix-web https://github.com/rofrol/rust-shopping-bot/commit/cc040bb07aed36b5eb3da01ff20a214d54749400#diff-639fbc4ef05b315af92b4d836c31b023
DEBUG=true cargo run
What do think about this way?
Nikolay Kim
@fafhrd91
May 20 20:45
response should contain error text as well
_
@rofrol ^
</pre>

https://gitter.im/actix/actix?at=5ce260607c363c75a70f852c
