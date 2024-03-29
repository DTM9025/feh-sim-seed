## Note on Standard Banners

Currently, the standard banner, due to having no focus items, is done through a workaround where
all the 5* and 4* items that can appear are considered "focus items" and the rate of focus items
to nonfocus items is set to 100% vs 0% respectively, which should simulate the actual summoning
mechanic.

## Settings

### Goal

Choose a goal from the dropdown menu. The options are:

* **Custom goal** - details below.
* **Any 5\* focus item** - pulls until any 5* focus item appears.
* **Any 5\* focus charcter** - pulls until any 5* focus character appears.
* **Specific 5\* focus charcter** - pulls until a specified number of a specific 5* focus character appears.
* **Any 5\* focus weapon** - pulls until any 5* focus weapon appears.
* **Specific 5\* focus weapon** - pulls until a specified number of a specific 5* focus weapon appears.
* **Any 4\* focus item** - pulls until any 4* focus item appears.
* **Any 4\* focus charcter** - pulls until any 4* focus character appears.
* **Specific 4\* focus charcter** - pulls until a specified number of a specific 4* focus character appears.
* **Any 4\* focus weapon** - pulls until any 4* focus weapon appears.
* **Specific 4\* focus weapon** - pulls until a specified number of a specific 4* focus weapon appears.

#### Custom goals

For examples of how they work, you can choose a preset and then switch to a custom goal to see what that preset is actually doing. Each goal is a collection of individual unit targets. The simulator will continue until one of those targets appears or until all of those targets have appeared, depending on the all vs. any setting. When there are multiple targets for the same color, they each represent a different focus unit.

For example, if you want a C6 of the new character and also to pick up one copy of another unit , set the selector to "All of these" instead of "Any of these" and create two entries: 7 copies of a specific 5* character, and 1 copy of a specific other unit unit. The simulation will then pull until both that other unit has appeared and the new unit has appeared 7 times.

Be sure to choose the correct goal type when doing custom goals. If say you have a 5* Character goal on the Weapon Banner, the sim will infinitely loop as it is impossible to get a 5* focus character from the Weapon Banner. While the sim disables the ability to choose mismatching goals for the most part, there are cases (like having multiple goals) where it is not, so please be careful.

<!-- Note that when enabling the Epitomized Path, when using custom goals the sim assumes that the weapon selected for the path is the one with the highest number of copies needed. This is the optimal selection that results in the fewest pulls. Whenever a goal is summoned, if the selected Epitomized Path is no longer the one with the highest number of copies needed, the sim will change its selection to be the one with the current highest number of copies needed automatically. This again is the optimal procedure that results in the fewest pulls. -->

Note that when enabling the Epitomized Path, when using custom goals the sim assumes that the weapon selected for the path is represented by the 5* goal listed first. This will not change unless that goal is completed, in which case the weapon selected for the path is moved the the second listed 5* goal, and so on. Optimally, the strategy with the fewest pulls would be to have the path for the 5* goal with the most amount of copies needed, and once that is completed move to the second most amount, and so on. Thus, it is suggested to order the 5* goals from highest number of copies needed to lowest when using Epitomized Path. However, we leave the option to have alternative selection orders in case there are specific wants and such for your summoning scenario.

### Banner selection

Select the starting rates from the dropdown menu.

Enter the number of focus units that the banner has on each item type in the boxes.

## Results

The graph shows how many orbs you need to spend to get a certain percent chance of reaching your goal, with labels at a few milestones for hard numbers. Each label shows the number of orbs spent before the indicated percentage of simulated results reach the goal.

Clicking or tapping on the graph will place a label on the line at the chosen horizontal position.

Don't forget that there is no amount of spending that can guarantee that you reach the goal. The 99th percentile shows a really high cost, but one out of every hundred people who read this will spend more than that next time they go to summon.
