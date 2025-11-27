## About

This simulator compares the scenarios of either having each circle have all red stones (and focus units)\* or each circle having no red stone (except focus units)\*\* while aiming to summon for red focus units. Its purpose is to show that the ideal scenario when loading in the summoning circle is to see less of the color you are aiming for, contrary to popular opinion that you want to see more color that you are aiming for.

To facilitate this, the option Orb Behavior has been added which has three options:
* Normal: Normal summoning behavior in FEH
* All Reds: The summoning circle will be all red stones (and focus units) \*
* No Reds: The summoning circle will have no red stones (except focus units) \*\*

This is implemented by changing the behavior of summoning non-focus pools. If the RNG chooses say a 3\* pool unit, if one chooses All Reds as the behavior option, then only red 3\* units will be selected. Similarly if one chooses No Reds, then no red 3\* units will be selected. The same will happen respectively for 4\* pool units, 4\* special pool units, etc. This is implmented by changing the non focus pools to either only have red units or have no red units.

\* Due to how this is implemented, the focus pool behavior remains unchanged. Thus if a focus unit is summoned and is not red, it will show up in the circle. Thus it is techincally not "all red", but it is very very close as summoning a focus unit is relatively rare and it would almost always be 4 reds then. Regardless, it should be sufficient for this demonstration.

\*\* Techinically "One Red" is better than "No Red". Both no red and one red scenarios result in just pulling once and backing out of the circle and so are mostly equivalent, though there may be instances for "No Red" where you pull an off color focus 5\*, decreasing the rates. Therefore, true "One Red" would be better, but "No Red" is still better in rates compared to "All Reds" despite that small detriment. Note that the above note about focus units being unchanged remain true, so 5\* focus reds would still appear as normal despite being "No Red".

The code for this web application can be found in the repository below under the `orb-behavior` branch:

* Source: [https://github.com/DTM9025/feh-sim-seed/tree/orb-behavior](https://github.com/DTM9025/feh-sim-seed/tree/orb-behavior)
