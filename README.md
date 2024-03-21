# Coffee

Coffee is a program that keeps your computer awake while you are not actually there monitoring it.
I needed this stuff as I wanted to prevent my laptop from falling asleep (or detecting that I was idle) 
while training large machine learning models. I suppose this program can have some other uses ;-)

What coffee does, is it periodically moves the mouse, then open notepad, type in some stuff and then 
closes it.


## building coffee

To build coffee you will need to have a rust toolchain installed on your machine. For more info about how
to do that, check here: https://www.rust-lang.org/learn/get-started

Once the toolchain is installed, you can compile the code with `cargo build --release` which creates the
desired binary in $project/target/relase. 

## Stopping coffee

Once you 've had enough, you can stop coffee with either Ctrl+C or by killing the process with the task manager
