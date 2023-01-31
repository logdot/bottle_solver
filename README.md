# bottle_solver
Is an algorithm that tries to find the best possible solution to one of those liquid sorting puzzle games.
If you've never seen one of them before, they look kind of like this:
![image](https://user-images.githubusercontent.com/34782839/215686510-344e1886-6198-4107-9a6e-65259d5169f3.png)

The whole idea behind the game is that you want to take these bottles filled with a rainbow of colors, and make it so that each bottle contains only one colored liquid.
There was a time where I was pretty addicted to these games so I decided to make an algorithm in rust to solve them for me.

The algorithm is a kind of tree searching algorithm where we try to find the shortest path possible to any game that is considered to be solved.
It can do this pretty quickly, taking less than 20 seconds to solve a fairly difficult game with 10 bottles, 8 colors, and 20 steps for the shortest possible solution.
This is possible because the algorithm makes extensive use of pruning and deduplication.
Without making use of these tecnices the algorithm could easily take upwards of an hour to find the best possible solution, as it would need to traverse the entirety of a giganteous tree.
