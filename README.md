# Tiny K/V

```bash
 _______ __                __  __                 ______ ___         __              
|_     _|__|.-----.--.--. |  |/  |.-----.--.--.  /  /   |   |.---.-.|  |.--.--.-----.
  |   | |  ||     |  |  | |     < |  -__|  |  |,' ,'|   |   ||  _  ||  ||  |  |  -__|
  |___| |__||__|__|___  | |__|\__||_____|___  /__/   \_____/ |___._||__||_____|_____|
                  |_____|               |_____|                                      
```

A tiny key/value store with encrypted backups.

## API

* get key -> Option\<value\>
* set key value -> value
* del key -> bool
* keys -> Vec\<\&str\>
* dbsize -> usize
* savedb -> usize

## As a library

* for rust projects
* thread safe
* async compatible?

## As a service

* language agnostic
* socket listeners

## Implementation

Option A: HashMap<String, String> ; simple
Option B: db = HashMap<String, Vec<String>> for key/values where values are pushed for each set then evaluated in the background.


###### dpw | 2023.12.07

