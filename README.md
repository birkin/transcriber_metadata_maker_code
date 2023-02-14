Prepares metadata for the transcriber project. 

Overview -- this repo.

This code prepares a list of items from the BDR that are ready for transcription. The list is stored in a file that can be used by the transcriber project.

---

Overview -- whole project.

The transcriber project is a set of services that will allow individuals to transcribe audio and video files using Whisper. 

The public (to the Brown community) facing part of the project will be a website for people and API calls from other services.

If a person logs in, ze will be able access either a form to add an item to the transcription-queue, or transcription-queue entries. 

- The data-entry form will simply consist of the fields `title` and `url`. The webapp will also store the user-eppn and the datestamp of the submission. 

- The transcription-queue entries will show the above fields, as well as the `transcription`, `tool_metadata`, `transcription_date`, `transcriber`, `time_taken`, and `status` fields.

If the webapp's API is called, it will return...

---

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
- proceed with how to iterate through all pages of results
    - when to start storing stuff to the tracker?
- eventually add an "if tracker file exists -- skip its creation and just start processing"

---

[end]