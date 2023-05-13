# Village

# Overview

## DATA

We divide up our game data into 2 major pieces, often touched+often changed, and rarely touched+rarely changed, and deal with them different depending on needs.  The 2 systems are called **ECS** and **COM**

Another data split is between static data, and changing data.  For example, *MaxSpeed* is often a static value, while *CurSpeed* changes quite a bit.  These 2 things are split into **DEF** and **INST** pieces.

These 2 ideas interact, so both ECS and COM have Def's and Insts.

### ECS

This piece is for often touched often changed data.  Movement data is a great example, position, rotation, etc.

We're going to double buffer this data.

### COM

This is the more traditional GetComponent style components.  I want to 
approach this via an immutable style of code.  

## CODE
