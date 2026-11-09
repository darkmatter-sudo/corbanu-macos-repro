| pattern | 1000 median ms | 2000 median ms | 4000 median ms | 8000 median ms | doubling ratios | verdict |
|---|---|---|---|---|---|---|
| nested_substitution | 69.836 | 124.979 | 191.090 | 384.303 | 1.79, 1.53, 2.01 | linear |
| alias_chain | 108.238 | 156.600 | 262.980 | 445.252 | 1.45, 1.68, 1.69 | linear |
| arithmetic | 33.027 | 41.015 | 29.077 | 40.699 | 1.24, 0.71, 1.40 | linear |
| glob_alternatives | 45.897 | 63.897 | 112.485 | 200.550 | 1.39, 1.76, 1.78 | linear |
| nested_quotes | 25.336 | 27.888 | 42.024 | 56.115 | 1.10, 1.51, 1.34 | linear |

Verdicts on each line (classifier result):
- nested_substitution: Some(UnseenCode)
- alias_chain: Some(UnseenCode)
- arithmetic: None
- glob_alternatives: None, Some(UnseenCode)
- nested_quotes: Some(UnseenCode)
