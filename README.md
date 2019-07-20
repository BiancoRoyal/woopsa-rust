[![Crates.io version](https://img.shields.io/crates/v/woopsa.svg)](https://crates.io/crates/woopsa)
[![Docs](https://docs.rs/woopsa/badge.svg)](https://docs.rs/woopsa/)
[![Coverage Status](https://coveralls.io/repos/github/BiancoRoyal/woopsa-rust/badge.svg?branch=master)](https://coveralls.io/github/BiancoRoyal/woopsa-rust?branch=master)
[![CI](https://travis-ci.org/BiancoRoyal/woopsa-rust.svg?branch=master)](https://travis-ci.org/BiancoRoyal/woopsa-rust/)


# woopsa-rust
A pure Rust library for [Woopsa](http://www.woopsa.org/).

## Features

## Install

## Contributing

Let's work together - send us your pull request!

### TODO

* stack
* tests
* documentation
* examples
* automated release process

## Authors
[Klaus Landsdorf][1]

## License
The code is licenced under [MPL-2.0](https://opensource.org/licenses/MPL-2.0). Like all open source code, you use this code at your own risk.

[1]:https://github.com/biancode



# Woopsa
This is the Rust Woopsa module.
To find out more about Woopsa and get the C#, Embedded C or JavaScript implementation, go to http://www.woopsa.org!

Woopsa is a protocol that's simple, lightweight, free, open-source, web and object-oriented, publish-subscribe, real-time capable and Industry 4.0 ready.
It contributes to the revolution of the Internet of Things.

Woopsa allows you to share the complete object model of your application in a way that's similar to OPC UA.
It's based on rock-solid foundations such as HTTP and JSON, which makes it work on the web out-of-the-box. 
Our mission is to get Woopsa on as many platforms as possible. 
Today, Rust, C# and JavaScript implementations exist, but there are much more to come!
As a node module, Woopsa is very useful if you wish to quickly create a RESTful API. 
On the server-side, just give Woopsa any JavaScript object with properties and functions, 
and the library will create a RESTful API allowing you to read/write properties and call functions, automagically!

To give it a test, just go on `http://{ip-of-your-server}/woopsa/read/Temperature` and see the magic! 
Woopsa is fully RESTful which means you can easily test it with your browser. 
It publishes your entire object's structure, and you can see all the data it publishes by going on `http://{ip-of-your-server}/woopsa/meta/` .

On the client-side, just get the Woopsa library on http://www.woopsa.org and you can immediately work with your object:

## Getting started
Our [Getting Started](http://www.woopsa.org/get-started/) tutorial allows you to get started quickly with Woopsa. 
It's really easy and we promise you'll be convinced!

