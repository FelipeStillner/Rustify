# Rustify

An Spotify clone made with Rust

Diagram: https://drive.google.com/file/d/1I6j-wyG_S0vyEOSwC5Y-UFeAC9ABB5c8/view?usp=sharing

Libraries:

* rodio     ("0.17.3"): For play audio files
* sqlite    ("0.33.0"): For store data in a DB
* crossterm ("0.27.0"): For edit the terminal

ToDo:

* edit, add, remove - musics, playlists and users
* Import and export musics
* Change the thread from only when playing to everywhere, To solve the problem of input thread doesnt stop
* Add author for each music
  * Also add info about him
* Insert musician interface
* insert more info about each music
  1. Author
  1. Producer
  1. Genre
* Add Album
* Add podcasts
* Improve the privacy for:
  * Users
  * Musics
  * Playlists
  * etc.

Talvez:

* Separar o back-end: colocar o banco de dados em uma API em um programa separado e fazer requests por meio de do protocola http
* Colocar todo o projeto em um docker
