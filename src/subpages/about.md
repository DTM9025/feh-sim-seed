## About

The code for this web application can be found in the repository below under the `genshin-sim-seed` branch:

* Source: [https://github.com/DTM9025/feh-sim-seed/tree/genshin-sim-seed](https://github.com/DTM9025/feh-sim-seed/tree/genshin-sim-seed)

This website is hosted on GitHub pages under the following repository:

* Host Repository: [https://github.com/DTM9025/DTM9025.github.io](https://github.com/DTM9025/DTM9025.github.io)


This web application is based on FEH Statistics by Minno726, from which it was forked from:

* Website: [https://fullyconcentrated.net/fehstatsim/](https://fullyconcentrated.net/fehstatsim/)
* Source: [https://github.com/minno726/feh-sim-seed](https://github.com/minno726/feh-sim-seed)

### Assumptions About Soft Pity Mechanics

Due to the fact that the exact mechanics of soft pity, and the gacha system in general, are unknown, there are
certain assumption that had to be made when building this statistic simulator.

The model used is based on the statistics given in this post:
* [https://www.hoyolab.com/genshin/article/497840](https://www.hoyolab.com/genshin/article/497840)

### Assumptions About Capturing Radiance

Due to the fact that the exact mechanics of Captruing Radiance are unknown, there are certain assumption that had to be made when building this statistic simulator.

The model used is based on the statistics given in this post:
* [https://www.reddit.com/r/Genshin_Impact/comments/1hd1sqa/understanding_genshin_impacts_capturing_radiance/](https://www.reddit.com/r/Genshin_Impact/comments/1hd1sqa/understanding_genshin_impacts_capturing_radiance/)

Note that at the time of that post, the exact probability of receieving a 5* Focus for when Counter = 2 is currently unknown. This simulator assumes that probability is 55%, which is the lowest whole number value that meets or exceed the consolidated probabililty promised by Mihoyo.

### Note on Epitomized Path

<!-- Note that when enabling the Epitomized Path, when using custom goals the sim assumes that the weapon selected by the path is the one with the highest number of copies needed. This is the optimal selection that results in the fewest pulls. Whenever a goal is summoned, if the selected Epitomized Path is no longer the one with the highest number of copies needed, the sim will change its selection to be the one with the current highest number of copies needed automatically. This again is the optimal procedure that results in the fewest pulls. -->

Note that when enabling the Epitomized Path, when using custom goals the sim assumes that the weapon selected for the path is represented by the 5* goal listed first. This will not change until that goal is completed, in which case the weapon selected for the path is moved the the second listed 5* goal, and so on. Optimally, the strategy with the fewest pulls would be to have the path for the 5* goal with the most amount of copies needed, and once that is completed move to the second most amount, and so on. Thus, it is suggested to order the 5* goals from highest number of copies needed to lowest when using Epitomized Path. However, we leave the option to have alternative selection orders in case there are specific wants and such for your summoning scenario.
