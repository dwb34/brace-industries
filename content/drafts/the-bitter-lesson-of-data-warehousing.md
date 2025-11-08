---
title: The Bitter Lesson of Data Warehousing
date: 2025-11-07
published: false
---

# The Bitter Lesson of Data Warehousing

On a late Friday afternoon a wise data modeler on my team said a line that I don’t think I’ll ever forget.

> Guys, you are bending over backwards to fix things in your warehouse, when you really should be concerned that you’re paying claims for people who aren’t in your membership system.

The whole call the client team was going on and on how they need to explore all these different convoluted ways to preserve the “referential integrity” and get the best possible match rate of tagging members to medical claims. There might have even been a mention of SCD type 3. I think the data modeler had enough and carried on saying:

> I could have some dummy plan in your system, and have been getting free healthcare for years. Don’t you think that’s the problem you should be solving.

Later I was chatting with the data modeler and he mentioned this was the classic case of smart junior people trying to solve for the problem right in front of them. Which is why this is such a bitter lesson. No matter how well you design, model, and build your data warehouse, if the source systems you are working with are poorly designed there’s nothing you can do about having bad data.

The whole team was optimizing for having the highest match rate where the true insight should have been “We're paying $XXM in claims for people we have no idea if they are our members“.  

