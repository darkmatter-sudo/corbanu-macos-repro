| pattern | 1000 median ms | 2000 median ms | 4000 median ms | 8000 median ms | doubling ratios | verdict |
|---|---|---|---|---|---|---|
| nested_substitution | 25.971 | 52.511 | 105.967 | 211.280 | 2.02, 2.02, 1.99 | linear |
| alias_chain | 44.031 | 70.967 | 125.308 | 221.116 | 1.61, 1.77, 1.76 | linear |
| arithmetic | 1.117 | 2.189 | 4.407 | 8.989 | 1.96, 2.01, 2.04 | linear |
| glob_alternatives | 14.332 | 28.869 | 56.082 | 104.176 | 2.01, 1.94, 1.86 | linear |
| nested_quotes | 1.712 | 3.195 | 6.214 | 12.848 | 1.87, 1.95, 2.07 | linear |

Verdicts on each line (classifier result):
- nested_substitution: Some(UnseenCode)
- alias_chain: Some(UnseenCode)
- arithmetic: None
- glob_alternatives: None, Some(UnseenCode)
- nested_quotes: Some(UnseenCode)
