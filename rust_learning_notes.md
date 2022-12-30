# settings hashmap experimentation

If I create a settings-hashmap, and then want to get one of the settings, I can do it via...

```let search_url: String = settings["SEARCH_URL"].to_string();```

I was worried that accessing the setting like that would prevent being able to access it a second time, and that instead I'd need to use a reference to settings, like...

```&settings["SEARCH_URL"]...```

...but it seems that's not the case. If I'd've needed to use that reference, this would've worked:

```
let search_url: String = (&settings["SEARCH_URL"]).to_string();
```

In the above example, the parentheses around the settings-key are required, otherwise the compiler shows the error:

```
expected struct `std::string::String`, found `&std::string::String`
```
---

[end of notes]