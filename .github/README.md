<div align="center">

<img src="media/fib.jpeg" height="100">

# _☰fib_ ⚡️

## A Fibonacci calculator written in multiple languages to see what out-preforms what.
<img src="http://badgen.net/github/commits/rhhen122/fib/">
<a href="https://vimp.rhhen.xyz/Licenses/lookinggood/non/UNLICENSE.html">
<img src="http://badgen.net/static/license/VIMPNL/black"></a>

<br>
<a href="/extra/COPYRIGHT">Copyright</a>
|
<a href="https://vimp.rhhen.xyz/Licenses/lookinggood/non/UNLICENSE.html">License</a>
|
<a href="http://codeberg.org/rhhen122/fib">Codeberg</a>
|
<a href="http://github.com/rhhen122/fib">Github</a>
|
<a href="http://en.wikipedia.org/wiki/Fibonacci_sequence">Fibonacci Sequence Wikipedia</a>
|
<a href="http://en.wikipedia.org/wiki/Fibonacci">Leonardo Fibonacci Wikipedia</a>
|
<a href="http://roky.rhhen.xyz">Who is rhhen122?</a>
|
<a href="https://rhhen122.github.io/fib/">Web Docs</a>

<h4>Special thanks to:

🐳 <a href="http://www.docker.com">Docker</a>
|
👨‍💻 <a href="http://code.visualstudio.com">Visual Studio Code</a>
|
🐱 <a href="http://github.com">Github</a>
|
❄️ <a href="http://codeberg.org">Codeberg</a>
|
💫 <a href="http://www.debian.org/">Debian</a>
|
🌲 <a href="http://git-scm.com">Git</a>
<br>
For making this project possible!
</h4>
</div>
<br>
<img align="left" src="media/image.png" height="300">

######

If you are not very technologically sassy then don't worry! We have done all the hard work for you using `Docker`

First install the codebase, then run one of the commands inside of the `fib` dir and boom!

Within seconds your runnning the code on your machine without ever installing any dependencies.

When you run the program you are starting up a tiny linux machine install the dependencies to that and then running it on the machine.

<br>

#

### Install
```
git clone http://github.com/rhhen122/fib.git ./fib
cd fib ; rm -rf .git ; echo 'For all commands related to Docker please reference the /docker/ dir'
```
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
docker build -t fib-dock-rb ./RUBY-Docker
docker run fib-dock-rb
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

#### Go
```
docker build -t fib-dock-go ./GO-Docker
docker run fib-dock-go
```

#### C++
```
docker build -t fib-dock-cpp ./CPP-Docker
docker run fib-dock-cpp
```

#### Python
```
docker build -t fib-dock-py ./BF-Docker
docker run fib-dock-py
```

### Credits
Max Hendra

##### <a href="https://war.ukraine.ua/support-ukraine/">Support Ukraine! 🇺🇦</a>