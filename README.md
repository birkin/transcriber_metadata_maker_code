# Prepares metadata for the transcriber project. 

on this page...
- Overview -- this repo.
- Overview -- transcription-queue web-app

---

# Overview -- this repo.

This code can be called to prepare a list of items from the [BDR](https://repository.library.brown.edu/studio/) (Brown Digital Repository) that can be transcribed -- or to upload that list to the transcription-queue web-app (see below).

---

# Overview -- transcription-queue web-app

The transcriber project will be a set of services that will allow individuals to transcribe audio and video files using Whisper. 

## Webapp

The the Brown community (behind Shib) facing part of the project will be a webapp -- for people and for API calls from other services. This website will be both a queue for items to be transcribed, and will store transcriptions that can then be imported into other services like the BDR.

If a person logs in, [ze](https://blogs.illinois.edu/view/25/705317) will be able access either a form to add an item to the transcription-queue, or a list of items (both transcribed and to-be-transcribed). 

- The data-entry form will simply consist of the fields `title` and `url`. The webapp will also store the user-eppn (auth_ID) and the datestamp of the submission. 

- The transcription-queue entries will show the above fields, as well as the `transcription`, `tool_metadata`, `transcription_date`, `transcriber`, `time_taken`, and `status` fields. 

    - The `transcriber` field will be the "user-agent" of the service that performs the transcription.

    - The "status" field will be one of the following: `to_be_transcribed`, `in_progress`, `transcribed`.

- The overall flow will be:

    - Any designated user's script can the webapp's API with a GET to get the next item in the transcription-queue. This will change the status to 'in_progress'.

    - Once the user's script has transcribed the item, it will call the webapp's API with a POST, updating an item's `transcription`, `tool_metadata`, `transcription_date`, `transcriber`, `time_taken`, and `status` fields, changing the status to 'complete'.

## Calling scripts

A typical calling script will hit an API endpoint to get the next item in the transcription-queue, transcribe it, and then hit the API endpoint to update the transcription-queue entry.

An example of python code performing the transcription-step is [here](https://github.com/birkin/whisper_transcriptions_project/blob/main/try.py#L31-L33).

## To determine

- The webapp API handling a POST should validate the format of the data being sent. The example code, using [whisper](https://whisper.ai) for transcription, includes timestamped data very useful for other purposes.

- My initial testing shows that whisper can automatically transcribe Spanish audio, to use an example, into Spanish. I need to look into what metadata whisper provides so that the transcription-data posted back to the webapp contains language-metadata.

---

# This project

As noted above, the purpose of the code in this repository is to gather public audio and video items from the BDR that are candidates for transcription, and post the url to those items to the transcription-queue webapp.

This code will be using the BDR's excellent public [APIs](https://repository.library.brown.edu/studio/api-docs/) to prepare data. 

Note that this code is not downloading audio and video files. Rather, It is preparing a list of audio and video urls to populate the webapp-queue, described above. This will enable BDR transcription to take place in a way that will place very littl load on the BDR's servers.

This code is using the [Rust](https://www.rust-lang.org) programming language, which is not something we use in our department. But this is a side-project I'm doing on my own time, furthering my knowledge of Rust.

## Logic flow

- Define a series of facet calls to gather possible items to transcribe.

- Make all the facet calls and create a unique list of items.

- From the item metadata available from the facet-call, attempt to determine whether the item has an audio or video file for transcription.

---


[end]