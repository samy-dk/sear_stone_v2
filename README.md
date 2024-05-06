sear_stone_v2 is my second attempt at creating an aid for my Japanese study. It
combines a lot of what I've learned about rust over the past month or so and 
I'm honestly pretty proud of it so far!

# What does it do?
   When you run sear_stone_v2 and pass it one or multiple files, it scans them
for Japanese characters. It then adds these words to a list, combines it with 
any previously recorded words, and saves the list again with the new list of
words, sorted. 

   This is rather specific to me. Japanese doesn't separate words normally, but 
my teacher does to help me out when making these docs. There are two example 
Lessons that you can use to verify the code works for yourself!

# How to run
   Assuming you have rustc and it's suite of tools installed:

    git clone https://github.com/samy-dk/sear_stone_v2.git

    cd sear_stone_v2

   Then, you can use

    cargo run -- Lesson-001.txt Lesson-004.txt

   to build and run the code! Running this code will create a list of words 
in a file located at ./data/word_list.txt

   You can view the file there, or simply run

    cargo run -- -pr

   to print 10 random words, or 

    cargo run -- -pa

   to print all the words in the current list! Or, if you forget all of this,
run

    cargo run -- -h

   to print out a helpful doc on how to use the program!
