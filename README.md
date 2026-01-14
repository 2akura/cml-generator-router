# cml-generator-router

```
RAW INPUT (JSON string)
        │
        ▼
Parse JSON into Config struct
        │
        ▼
Destructure Config:
(enable, hostname, banner, password, domain_name)
        │
        ▼
Check each field:
┌───────────────┬─────────────┐
│ Field present?│ Not empty?  │
└───────────────┴─────────────┘
        │
        ▼
If true → push (key, value) into `preserved` Vec<(String,String)>
        │
        ▼
Call `take_and_return(preserved)` 
        │
        ▼
Inside `take_and_return`:
  - Extract keys from Vec<(String,String)>
  - Take the first key as String (`un_vec`)
        │
        ▼
Return String key to `takejson`
        │
        ▼
Parse String key into `Indexer` enum
        │
        ▼
Match Result:
 ┌───────────────┐
 │ Ok(variant)   │ → Call `dispatching(variant)` → get start_index
 └───────────────┘
        │
        ▼
Inside `dispatching`:
  - Match Indexer variant → return corresponding u8
        │
        ▼
FSM-ready index (`start_index`) can now be used
```
