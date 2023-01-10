
# What is Hacker News?

HN (news.ycombinator.com) is a place where the registered **users** (identified by string nicknames) place links to external resources (make **posts**, each **post** has a unique **item_id**).

Other registered **users** may upvote a **post** thus increasing the **score** of that **post**.

*Somehow* based on a **post's** **score** and **age** the order is defined among all posts — **rank** (the lower the numeric value of a **rank** is — the more "relevant" a **post** is).

The **/news** section of the site shows the posts with the best (lowest numerically) **ranks** in **pages**, 30 **posts** each.

---

# The Database

## Snapshot

A record per each fetch of a page.

Props:

- url: `URL` — the URL of the page;
- fetched at: `Timestamp` — the time at which the page was fetched.

## Document

A record with the payload of a **Page Snapshot**.
Maybe be dropped during the archiving to save space.

Props:

- snapshot: `ID`;
- data: `Bytes`.

## Scrap

Such record reflects the fact of a **Page** being already processed.

Props:

- snapshot: `ID`.

## Scrap Error

A record for a **Scrap** failure.

Props:

- scrap: `ID`;
- error: `String`.

## Post

A post extracted by processing a **Page**.

Props:

- scrap `ID`;
- item_id: `i64`;
- link: `URL`;
- title: `String`;
- by user: `ID`;
- posted at: `Timestamp`.

*Note: There is some risk of sharing the same values of those properties among several snapshost, as they may be edited. For sake of simplicity, this corner will be cut this time.*

## Score

A record reflecting the score of a **Post** as seen in a specific snapshot.

Props:

- post: `ID`;
- scrap: `ID`;
- score: `i32`.

## Rank

A record reflecting the rank of a **Post** as seen in a specific snapshot.

Props:

- post: `ID`;
- scrap: `ID`;
- rank: `i32`.

---

# Grabbing the Data

In order to acquire that data from the site, the solution would have to:

- fetch the pages, 1 through 30 (28?);
- scrap the data out of those pages;
- store it in the database.

Note: The site seems not to like being crawled...

---

# Queries

## List of top posts

We are searching for the **Rank** records such that the result set would contain:

- one record per each `Rank.rank` value (i.e. one — first place, one — second place, etc);
- for each result set record, choose the `Rank->Scrap->Snapshot` with the most recent `Snapshot.fetched_at`.

The result set above is sorted by rank in the ascending order and paged.

Without too much trickery the approach above may be able to present such a list for some given time in past.

## List of posts sent by a user

Choose a **User** record, join it with the **Post** records.

## List of posts of a user that was on the first page at some point.

I can't say I am sure I have understood that piece in its entirety. :(

We could find the set of **Users** that have created some **Posts** that according to some snapshots have achieved some rank less or equal to 30.

But I'm confused by "**a** user" in the original wording of the task.


