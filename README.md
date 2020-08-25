# Titan
A simple Gemini server. Configurable via TOML or Rust, depending on your needs. Very much under construction.

## What's all this then?

### Gemini:

Gemini is an application layer protocol for file distribution. It aims for simplicity, user privacy, and to foster a DIY attitude in its community. You can find out more at the [FAQs](https://gemini.circumlunar.space/docs/faq.html).

### Titan:
I started writing Titan because I was excited by the Gemini project and its goals to "strip the Web back to its essence". I'm also learning Rust and am constantly looking for new systems projects to work on in it. After reading the *very* short spec, I realized that I could probably implement a Gemini server without too much trouble. I've been learning a good bit about the Internet and how application layer protocols work along the way, so that's been cool too! Hopefully Titan can help more people start their own Gemini servers and start spreading more content around the tiny Internet.

## Getting started

### TLS Setup
First, install `openssl`. Next, run

```bash
openssl req -x509 -out <MYDOMAINNAME.TLD>.crt -keyout <MYDOMAINNAME.TLD>.key \                     
  -newkey rsa:2048 -nodes -sha256 \
  -subj '/CN=localhost' -extensions EXT -config <( \
   printf "[dn]\nCN=<MYDOMAINNAME.TLD>\n[req]\ndistinguished_name = dn\n[EXT]\nsubjectAltName=DNS:<MYDOMAINNAME.TLD>\nkeyUsage=digitalSignature\nextendedKeyUsage=serverAuth")
```

then run

```bash
openssl pkcs12 -export -out <MYDOMAINNAME.TLD>.pfx -inkey <MYDOMAINNAME.TLD>.key -in <MYDOMAINNAME.TLD>.crt
```

This second command will ask you to input a password. Make sure you pick a good one and that you remember it! Export the password as an environment variable named `TITAN_CERT_KEY`.

## Updates/Roadmap
See the [changelog](/CHANGELOG.md)!

## Aims
- A command line plug-n-play server that can be configured via a Titan.toml file, which will specify the routes;
- A Gemini server framework a-la-Express, which can be used to create custom Gemini servers with more functionality than the plug-n-play server;
- ~~Maybe some other stuff like a Markdown -> text/gemini compiler?~~ Maybe later, but for now check out [md2gemini](https://github.com/makeworld-the-better-one/md2gemini). 