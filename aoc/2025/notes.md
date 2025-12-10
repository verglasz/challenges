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

## day 10

well p2 looks very annoying...
~~my current best idea is to do something like, save all combinations of buttons and the highest~~

no nevermind, the order of buttons doesn't matter, so a graph search is very redundant...
this should have a reasonable recursive solution after all...

the difficult thing is knowing how to prune.
if we sort buttons based on how many joltages they increase, is it true that
  the first solution found (if we try buttons in  order with backtracking) is the minimum?
  it seems likely enough... but doesn't seem to be true on the test sigh

ok well this is a linear integer optimization problem... idk how to do these quickly
it'll require some reading

a button is a vector (of 1s and 0s) in Z^k where k = num joltages,
i need to satisfy Σ_i α_i b_i = J while minimising  Σ_i α_i.

should be straightforward as an optimization problem tbh

(first i noted what's equivalent to saying that
Σ_i α_i (b_i • I) = Σ_i α_i n_i = J • I
where I is all-ones so n_i is the number of 1s in b_i, naturally this constrains α_i a lot
since n_i >= n_j if i < j, i (and J•I is fixed) i thought this meant
that α_i must be greedily maximised but clearly this isn't true because of the obvious reasons

idk i probably should use some optimized ILP solver...
