# Judgement Date

This program was created as part of an assignment for a second year Computer Science paper at Massey University. This assignment follows on from [the first assignment](https://github.com/Jamelas/Change-Date) that was written using Haskell.

## Requirements

1. Implement function from ymd and ymd, which convert between the year/month/day format and integer
format, which stores days since 0/1/1, i.e., Year 0, Jan 1st. Note that dates before the reference date
must be supported as well, represented as negative integer values.

2. Implement traits Debug, Display and Add for Date to enable the code in function main to run. Dates
in or before year 0 should be displayed using the BC notation of the Gregorian calendar. For Add, the
expression some date + 30 should return the date 30 days after some date.
Note: In the Gregorian calendar there is no year 0, and hence non-positive year values are offset by 1.
That means that 0/1/1 (in the astronomical format used by from ymd and ymd) should be displayed as
1/1/1 BC, while -1/12/31 should be displayed as 2/12/31 BC.

3. Do not use any external crates.
