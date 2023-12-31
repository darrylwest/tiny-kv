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
* loaddb -> Result\<usize\>
* savedb -> Result<\<usize\>

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

## Uses

* udp socket service - uses this as a backing store; includes a repl
* otp-session-service - one-time-password and sessions
* auth-service - registration, access to otp/sessions; stores user id, email, first/last, date created, last updated, etc
* configuration-service - to enable dynamic config loading and reloading
* messaging-service - store for future delivery key recipient: xxx, send_at: xxx, message: bla bla bla
* event-queue-service - post to queue, events broadcast to subscribers
* inventory reference, inventory control
* todo, shopping, event lists
* fitness-tracker

###### dpw | 2023.12.12

