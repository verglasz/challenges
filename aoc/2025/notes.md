# AoC 2025

Misc notes on the day's problem and how to improve perhaps?

## day 1

straightforward, in p2 took a bit too long to figure out the proper condition for
whether a partial turn crosses 0 or not.

### improvements
write down more clearly the logic of when something should turn?
it's fine to write long ass boolean expressions as long as i'm sure they work,
then i can see if i can clean them up later.

## day 2
a little anal, p2 required some rewriting, i went back-and-forth on
whether i'd do iteration on number of digits or iteration on repeats,
but the first idea (repeats) was right, then i forgot the intuitive
reason why which lead to some debugging

also again some memery around boolean checks which maybe could have been avoided by
writing it out in full in the dumbest possible way.

also the solution is half-optimized really, sure it's better than iterating over
each id in the range but ehh the ranges are smallish anyway and we're still iterating...
and it's not clear how to improve it since the math trick can't be used
because of "overlapping" index ids (ie, 12121212 is either 2×1212 or 4×12)
which also implies using a hashset to keep track of this...

### improvements
write down a comment clarifying why i decided one thing?
but it was intuitive rather than thought through so idk, tricky.

really just do the dumbest possible iteration thingy maybe...
