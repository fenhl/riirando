New fill algorithm for the Ocarina of Time randomizer

# Innovations

* Unified algorithm for entrances and items, filled in one step. Might also be possible to extend to settings.
* Should give fewer retries, especially for entrances.
* Removes issues that can occur due to some items being shuffled before others.
* Allows true mixed pools entrance randomizer.

# Fill

1. Calculate the item pool.
2. Make a list of all the items. For items that appear multiple times, track each copy separately.
3. Make a list of all the _checks_ and, for each check, the _fillings_ (items or entrances) that can go there depending on the check's type, the randomizer settings, and any restrictions from plando.
4. Lay out the checks and fillings in a 2D matrix with Boolean values representing possible placements.
5. Out of the checks and fillings tied for fewest marked entries (fillings that can go there or checks where they can be placed, respectively), choose one randomly.
    * If there are 0 marked entries, we have reached a dead end. Roll back to before the last random choice, disallow the option that was rolled, and try again. If there was no random choice yet, the settings combination or plando is impossible.
        * Maybe it would be better to, instead of only rolling back the last random choice, start over from scratch but remember the disallowed combination of random choices in case it is rolled again. This might help in case the actual problem is near the start of the sequence of random choices.
    * Otherwise, pick a random one out of the possible options (if there is only one, this does not count as a random choice) and perform a search to check if it's possible. If yes, lock in that option (remove all other markings from that row and that column, and that row and that column are no longer eligible to be chosen), apply any consequences from the settings (e.g. 1 major item per dungeon, coupled entrances, hint area exclusion), and repeat step 5. If no, remove the marking from that option and repeat step 5.
    * If there are no checks or fillings left, we are done.

## Caveats for entrances

* It might make sense to store entrances as separate tables for each world, to avoid wasting a bunch of memory.
* One-ways need to be separate from two-way entrances, because:
    * A one-way going to an entrance doesn't prevent a two-way from also going there.
    * One-ways work differently from two-ways in that not every possible destination will be filled, so it doesn't make sense to “choose a one-way that will go here” (unless “none” is one of the options and/or retaining the concept of priority one-ways is desired) so only one dimension of the one-way table is considered for next row/column to pick.
    * The possible targets for blue warps in “own dungeon entrance” mode are algorithmically determined based on the configurations of other entrances.

All randomization tables would be considered at the same time for picking the next row/column.

# Search

Start with an empty inventory. Collect all reachable checks (including ones that still have multiple possible fillings) and events. Repeat until stuck or won. When verifying access to a check/event, test whether there is a combination of items from collected checks such that no two items come from the same check (since each check can only have 1 item) and no item is used twice (hence the separate tracking of copies) that fulfills the access requirements.

## Ensuring repeatable access to different global states

The global states are times of day (here counting midnight, noon, and Dampé time, under the untested assumption that each other time of day is logically equivalent to at least one of the three) and ages (child and adult). We always have to account for the possibility that one of these states is reached out of logic, so we can only rely on states that are reachable from all other states.

To verify that this is the case, for each of the possible concrete state combinations (child midnight, adult midnight, child noon, adult noon, child Dampé time, and adult Dampé time), start a search where spawn access exists in that state but no other states are accessible, then check which other states are reachable. Then for the actual search, consider spawn to be accessible as the states that were transitively reachable from all other states.

For the real search, we map regions to sets of state combinations in which they're accessible, with each state space having an additional “unknown” row. Notably, an “unknown time of day, unknown age” entry should be distinguished from an empty (or equivalently, missing) one: it means that the region is known to be accessible at some time of day as some age, but not what those states are. For events and checks in accessible regions, we check if any one of the marked state combinations of the region are enough to access the check, with “unknown” meaning that all possibilities of that state space have to yield access.

### Health state

It might be possible to put heart upgrades into logic by tracking the maximum available hearts & fairies as a parameter of global state. Spawn always starts at 3 hearts, heart refills max out health (depending on heart containers), fairy refills max out fairies (depending on bottles, need to ensure there's no conflict with other required bottle contents, such as bugs), reduce max available when taking damage.
