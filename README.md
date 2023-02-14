# Prepares metadata for the transcriber project. 

on this page...
- Overview -- this repo.
- Overview -- transcription-queue web-app

---

### Overview -- this repo.

This code can be called to prepare a list of items from the BDR that can be transcribed -- or to upload that list to the transcription-queue web-app (see below).

---

### Overview -- transcription-queue web-app

The transcriber project is a set of services that will allow individuals to transcribe audio and video files using Whisper. 

The public (to the Brown community) facing part of the project will be a website for people and API calls from other services. This website acts as a queue for items to be transcribed.

If a person logs in, ze will be able access either a form to add an item to the transcription-queue, or a list of items in the transcription-queue. 

- The data-entry form will simply consist of the fields `title` and `url`. The webapp will also store the user-eppn and the datestamp of the submission. 

- The transcription-queue entries will show the above fields, as well as the `transcription`, `tool_metadata`, `transcription_date`, `transcriber`, `time_taken`, and `status` fields.

If the webapp's API is called with a GET, it will return the next item in the transcription-queue, and change the status to 'in-progress'. If called with a POST, the API will update the `transcription`, `tool_metadata`, `transcription_date`, `transcriber`, `time_taken`, and `status` fields, changing the status to 'complete'.

---

Logic steps...

- Iterate through all three types of facet calls to gather possible items to transcribe.

HEREZZ

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
- proceed with how to iterate through all pages of results
    - when to start storing stuff to the tracker?
- eventually add an "if tracker file exists -- skip its creation and just start processing"

---

[end]