<div align="center">

<img src="fib.jpeg" height="100">

# fib
## A Fibonacci calculator written in multipul languages to see what out-preforms what.
<img src="http://badgen.net/github/commits/rhhen122/fib/">
<img href="https://vimp.rhhen.xyz/Licenses/lookinggood/non/UNLICENSE.html" src="http://badgen.net/static/license/VIMPNL/black">

<a href="/COPYRIGHT">Copyright</a>
|
<a href="http://codeberg.org/rhhen122/fib">Codeberg</a>
|
<a href="http://github.com/rhhen122/fib">Github</a>
|
<a href="http://en.wikipedia.org/wiki/Fibonacci_sequence">Fibonacci Sequence Wikipedia</a>
|
<a href="http://en.wikipedia.org/wiki/Fibonacci">Leonardo Fibonacci Wikipedia</a>
</div>
<img align="left" src="image.png" height="300">

######

Its simple, it procedurally generates The Fibonacci Sequence eternally.

Run it via a `Docker` container or run it locally!

Enjoy!

<br>
<br>
<br>
<br>
<br>
<br>
<br>
<br>
<br>

#

### Tutorial.rs
Go into the `Tutorial/tutorial.rs` file and change any of the functions to `0` to run them.
```
docker build -t fib-dock-tut ./Tutorial
docker run fib-dock-tut
```

### How to use

#### Ruby
```
docker pull rokyh/fib-dock
docker run rokyh/fib-dock
```

Or compile the image yourself. Run the following in your terminal:
```
docker build -t fib-dock ./
docker run fib-dock
```

This will make a `docker` container and run the script `fib-op.rb`

#### C
Compile from source:
```
docker build -t fib-dock-c ./C-Docker
docker run fib-dock-c
```

#### JS
```
docker build -t fib-dock-js ./JS-Docker
docker run fib-dock-js
```

#### PHP
```
docker build -t fib-dock-php ./PHP-Docker
docker run fib-dock-php
```

#### Rust
```
docker build -t fib-dock-rust ./RUST-Docker
docker run fib-dock-rust
```

#### Java
```
docker build -t fib-dock-java ./JAVA-Docker
docker run fib-dock-java
```

### Credits
Max Hendra

##### <a href="https://war.ukraine.ua/support-ukraine/">Support Ukraine! 🇺🇦</a>