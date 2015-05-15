The first step that I made was to get the code to compile. The code
was 6 months out of date and lots of changes have been made to Rust
in that time. I updated the code to compile and run against the most
version of Rust.

The next step was to add a new feature to the Drum Sequencer. I saw that
I could take advantage of the "say" command on Macs and dish out another process
to communicate whatever sound I chose for the pattern that was being processed.
As the sequencer is being played a "sound" is immediately played to resemble what
what was outputted to the console by the sequencer.

The `parse_for_sound(sound: &String)` function can be more robust by examining which
patterns were specified for the song and then match over those specified patterns
and generate the appropriate sound given its input.
