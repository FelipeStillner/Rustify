# Rustify

An Spotify clone made with Rust

Diagram: https://drive.google.com/file/d/1I6j-wyG_S0vyEOSwC5Y-UFeAC9ABB5c8/view?usp=sharing

Libraries:
* rodio     ("0.17.3"): For play audio files
* sqlite    ("0.33.0"): For store data in a DB
* crossterm ("0.27.0"): For edit the terminal

Todo:

* edit, add - musics, playlists and users
  * remove mu(mus)
* Change the thread from only when playing to everywhere, To solve the problem of input thread doesnt stop
  
Done:
* get updated info about the music/playlist
  * time
  * name
  * etc.
* pause, interrupt - musics and playlists
  * restart
  * etc.
