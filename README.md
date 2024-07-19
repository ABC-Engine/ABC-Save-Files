# The goal
The goal with this repo is to use serde to make a save file system where you can easily save multiple components in a save file with differences between versions. 
The ideal situation would be deserializing into a (Option<T0>, Option<T1>, Option<T2>). This way if you add new info that needs to be saved to your save file, the old one doesn't break.
