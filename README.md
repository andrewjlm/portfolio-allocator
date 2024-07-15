# portfolio-allocator

This tool is designed for periodic (eg annually or quarterly) reallocation of a retirement portfolio. You can input your current asset allocation (in dollars or other current) and your desired asset allocation (in percentages) and it will tell you how much to buy/sell of each asset class to align with your goals.

Sample usage:

```
> Command: Add Asset Class
Current Asset Classes:
[]
> New Asset Class: stocks
> Command: Add Asset Class
Current Asset Classes:
["stocks"]
> New Asset Class: bonds
> Command: Set Allocation
> Choose Asset Class: bonds
> New Allocation for bonds (Current: 0.00): 500
> Command: Set Allocation
> Choose Asset Class: stocks
> New Allocation for stocks (Current: 0.00): 1000
> Command: Set Target
> Choose Asset Class: bonds
> New Target for bonds (Current: Unspecified): 50
> Command: Set Target
> Choose Asset Class: stocks
> New Target for stocks (Current: Unspecified): 50
> Command: Check Portfolio
Asset Class     Allocation      Target
bonds   500     Some(50)
stocks  1000    Some(50)
> Command: Compute Exchange
Buy $250.00 of bonds
Sell $250.00 of stocks
> Command: Exit
```
