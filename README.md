Prepares metadata for the transcriber project. (TODO, describe overall project.)

Steps...

- Query all "videos" in the BDR. For each video...

- See if it has a a downloadable-file. If it does...

- Grab...
    - file url
    - collection(s) it's a part of
    - rights
    - title
    - mime-type
    - file size if available
    - duration if available    

---

next...
- refactor load_settings() so that it calls an envar_grabber() function with the desired key.
- maybe build a class -- or maybe just use "envy" again -- review that.
- proceed with how to iterate through all pages of results
    - when to start storing stuff to the tracker?
- eventually add an "if tracker file exists -- skip its creation and just start processing"

---

[end]