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

meanwhile i made it faster by making things a fixed-size array of i16
(hoping most things get autovectorised, and avoiding options everywhere so checks
for ==0 and < 0 are hopefully also vectorised), this made it possible to solve
the first problem (~26s).
Adding early pruning if the minimum required is above the current best gave another factor of like 10,
but the real juicy optim was skipping ahead if only a single button is left that can do one of the
presses; this cut down the first input problem to 30ms and made it possible to solve the second
input problem (until now it had remained always unsolved). This is now running on server
and it does look like it's running much faster than before. The end is in sight.

Probably this does mean that a better optimization would've been trying
first the buttons which have fewest overlaps (i.e., the ones where, if the k-th joltage value is > 0,
fewest button contribute to k). Unsure if sorting each recursion or hunting button indices would've
been the right thing...

anyway as i write this the server has churned through most of the problems and now
only 12/16 cores are working, which means only 3 out of 4 parallel blocks haven't finished
which hopefully means only 12 or slightly more problems are still being worked on. Nice!

the bad news is, well, those are pretty slow still, and the last ones to finish didn't even have
that large output values, so the "complexity" is varying very much based on how "lucky"
a problem instance is.
To be fair it does seem very closely related to number of buttons first and
max jolts second... so yeah cutting down on the buttons early is probably the way to go
