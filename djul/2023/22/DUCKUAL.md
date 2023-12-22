# THE DUCKUAL

dAnkan has knocked his head silly and has some difficulty undestanding what you are saying. <br> To
make things clearer, he gives you this manual which details how ducks communicate and think.

## The Tasks dAnkan can remember tasks that are given to him. Tasks are made of subtasks, each of which is ended by a `.`.

Here are some examples of tasks:
 - `waddle. waddle.` dAnkan will walk forwards twice.
 - `rotate_right. waddle. quack.`, turn right walk forwards and quack.
 - `can_waddle?waddle.`, if cw (which is explained later), then walk.
 - `!can_waddle?quack.`, if not cw, then walk.
 - `[1+2]=5.`, set memory at 3 to 5.
 - `do task1. do task2.`, perform tasks task1 then task2.

## The Values Values are expressions that contain numbers, arithmetic,
## memory references and whether dAnkan can walk forwards.<br><br>
## They are evaluated right to left because dAnkan is always right.
<img src="https://cdn.betterttv.net/emote/61943b0054f3344f8805f2d5/3x.png" width="48" height="48" style="float:right" />

Here are some examples and their results:
 - `5 + 4 / 2` becomes 7
 - `(5 + 4) / 2` becomes 4
 - `8 / 4 + 4` becomes 1
 - `!1+1` becomes 0.
 - `!1-1` becomes 1.

 dAnkan only knows about whole numbers, ie [0, ∞).

 There exists two duck specific numbers `can_waddle` (`cw`) and `in_number` (`in`) which can be used
 to determine whether dankan can waddle forwards, or is in a number room respectively. They yield 1
 if true and 0 otherwise.<br><br> It is possible to read values from dAnkans memory using square
 brackets.
 - `[2]` will become the value at memory index 2.
 - `[[0] + 4]` will become the value at memory address `[0] + 4`

## Memory It is possible for dAnkan to remember numbers. He has a total of 4096 javascript floating
point numbers at his disposal, ranging from 0-4095. This means that although dankan do not undestand
negative numbers, he can remember them.

- `[0] = [0] + 1.`
- `[2 + [0]] = 15.`

## Pondering dAnkan is exceptional at pondering, he can ponder whether a value is greater than zero
and act accordingly.<br>He does so by starting a subtask with a value, followed up by a "?" and
subtask.

- `[0]?can_waddle?waddle.`
- `can_waddle?{waddle. rotr}.`

## Quacktrace If something goes amiss, dAnkan will return and describe which tasks he trying to
perform.

## List of instructions and shortcuts
 - `do taskname.` - perform task
 - `waddle.` `w.` - walk forwards
 - `rotate_right.` `rr.` - rotate dAnkan 90 degrees right
 - `rotate_left.` `rl.` - rotate dAnkan 90 degrees left
 - `chill value.` - waits value milliseconds before resuming.
 - `quack.` `q.` - quacks
 - `fin.` `f.` - finish a task and go back to what was being done before.
 - `stop.` - stop and return to starting position.

## Also important dAnkan can only understand ~1024 characters at a time,<br> and he will truncate
any keyword that is longer than 16 characters.

