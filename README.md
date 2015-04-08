# spotify-rs

[![Build Status](https://travis-ci.org/jonalmeida/spotify-rs.svg?branch=master)](https://travis-ci.org/jonalmeida/spotify-rs)

A safe libspotify wrapper for Rust

> **This is still experimental. DO NOT USE**
>
> TBD More docs and info

# Installation

```
git clone <repo-link-here>
```

You also require the libspotify binary which needs to be downloaded separately from the Spotify Developer website.

## Linux

Download the latest version of libspotify from the developer website [here][linux-dev-download], the install it by following these commands in your terminal:
```
wget https://developer.spotify.com/download/libspotify/libspotify-12.1.51-Linux-x86_64-release.tar.gz
tar zxfv libspotify-12.1.51-Linux-x86_64-release.tar.gz
cd libspotify-12.1.51-Linux-x86_64-release/
sudo make install prefix=/usr/local
sudo ldconfig
```

## Mac
Easiest way is to use HomeBrew:
```
brew install libspotify
```



[linux-dev-download]: https://developer.spotify.com/technologies/libspotify/
