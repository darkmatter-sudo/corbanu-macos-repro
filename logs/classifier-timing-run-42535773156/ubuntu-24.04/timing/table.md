| pattern | 1000 median ms | 2000 median ms | 4000 median ms | 8000 median ms | doubling ratios | verdict |
|---|---|---|---|---|---|---|
| nested_substitution | 18.481 | 35.592 | 71.938 | 151.179 | 1.93, 2.02, 2.10 | linear |
| alias_chain | 30.053 | 48.180 | 87.188 | 159.020 | 1.60, 1.81, 1.82 | linear |
| arithmetic | 0.790 | 1.541 | 3.154 | 6.156 | 1.95, 2.05, 1.95 | linear |
| glob_alternatives | 10.336 | 25.336 | 38.641 | 79.449 | 2.45, 1.53, 2.06 | linear |
| nested_quotes | 1.144 | 2.264 | 4.334 | 8.668 | 1.98, 1.91, 2.00 | linear |

Verdicts on each line (classifier result):
- nested_substitution: Some(UnseenCode)
- alias_chain: Some(UnseenCode)
- arithmetic: None
- glob_alternatives: None, Some(UnseenCode)
- nested_quotes: Some(UnseenCode)
