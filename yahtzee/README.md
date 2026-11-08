# Yahtzee

A simple yet fun game where you have to combine luck and a bit of strategy to get more points than your opponent.

So basically the premise to any game ever ?

## How it works

2 players take turns in throwing 5 dice 3 times in a row, each time selecting between none and all of them to keep their values before the next throw.

The goal of those 3 throws is to finish with the dice matching one of the patterns listed below, to then add the associated score to their total scores :

| Pattern name | Description                                                   | Score |
| -            | -                                                             | - |
|*[Digit name]*| The same digit on any number of dice                          | `the digit * the number of times it appears` |
| Triple       | The same digit on 3 dice                                      | `the sum of all dice` |
| Square       | The same digit on 4 dice                                      | `the sum of all dice` |
| Full house   | The same digit on 3 dice and another digit on the last 2 dice | `25` |
| Small street | Any sequence of 4 digits                                      | `30` |
| Big street   | Any sequence of 5 digits                                      | `40` |
| Yahtzee      | The same digit on all 5 dice                                  | `50` |
| Chance       | No particular pattern                                         | `the sum of all dice` |

At the end of the game, when all patterns have a score for all players, the total score for each player is calculated by adding up all their scores.

A bonus of 35 points can be attributed to any player whose sum of all *[Digit name]* patterns is greater than (or equal to) 63.

## Run

```bash
cargo run
```

Make sure the terminal is big enough to properly display the scoreboard.

## Example

```text
(... ongoing game) 

 ==== Pattern name == Player 1 == Player 2 ====

1     One                                        
2     Two                                        
3     Three                                      
4     Four                           8           
5     Five                                       
6     Six                                        
7     Triple                                     
8     Square             22                      
9     Full house                                 
10    Small street       30                      
11    Big street                                 
12    Yahtzee                                    
13    Chance                                     

 === Player 2's turn ===
 |4|  |5|  |4|  |6|  |5| 
Enter the positions of the dice you want to keep ('all' to keep all) :
2 5
 |5|  |5|  |1|  |1|  |5| 
Enter the positions of the dice you want to keep ('all' to keep all) :
A   


 ==== Pattern name == Player 1 == Player 2 ====

1     One                                        
2     Two                                        
3     Three                                      
4     Four                           8           
5     Five                                       
6     Six                                        
7     Triple                                     
8     Square             22                      
9     Full house                                 
10    Small street       30                      
11    Big street                                 
12    Yahtzee                                    
13    Chance                                     

Final dice output :
 |5|  |5|  |1|  |1|  |5| 
Enter the corresponding number of the pattern for which you want to add your score (1-13) :
9

 ==== Pattern name == Player 1 == Player 2 ====

1     One                                        
2     Two                                        
3     Three                                      
4     Four                           8           
5     Five                                       
6     Six                                        
7     Triple                                     
8     Square             22                      
9     Full house                     25          
10    Small street       30                      
11    Big street                                 
12    Yahtzee                                    
13    Chance                                     
```
