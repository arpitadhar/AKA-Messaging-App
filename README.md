# AKA-Messaging-App
"The best messaging app ever"

## Sources

Diesel crate: 
- https://diesel.rs/guides/getting-started
- https://diesel.rs/guides/all-about-inserts.html

Exit code issue:
- https://youtu.be/EkI0AmHmZT8?si=3c48bsWSo-yrrFNT&t=1484

## Instructions for setting up on local machine

1. Download RUST and make sure it is updated to its latest version (check rustc --version) 
2. Download files from main branch (front-end, migration, etc)
3. The Diesel crate must be set up manually before use, in the terminal
    - cd to the rust server's root directory
    - Install the CLI tool, use command
      - cargo install diesel_cli
    - establish the database that the rust server will connect to
      - echo DATABASE_URL=postgres://username:password@localhost/diesel_demo > .env
      - diesel setup
5. Run the server by going to the terminal and using the command cargo run
6. Enter 0.0.0.0:3000 into your browser to see the server 
7. Run the client side by going to index.html and clicking run 
8. Enter a username and message to be posted on the client side
9. If the rust server ends unexpectedly with "exit code 3" after posting from the front-end, refer to the link below

   https://github.com/diesel-rs/diesel/discussions/2947#discussioncomment-2025857
   This is an issue with some older versions of postgres
