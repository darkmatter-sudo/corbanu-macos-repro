| pattern | 64 median ms | 128 median ms | 256 median ms | 512 median ms | doubling ratios | verdict |
|---|---|---|---|---|---|---|
| nested_substitution | 2.699 | 5.095 | 9.784 | 18.567 | 1.89, 1.92, 1.90 | linear |
| alias_chain | 8.162 | 16.250 | 31.498 | 41.702 | 1.99, 1.94, 1.32 | linear |
| arithmetic | 0.418 | 0.605 | 1.028 | 1.535 | 1.45, 1.70, 1.49 | linear |
| glob_alternatives | 2.126 | 3.957 | 9.335 | 20.414 | 1.86, 2.36, 2.19 | linear |
| nested_quotes | 0.688 | 1.130 | 2.008 | 1.382 | 1.64, 1.78, 0.69 | linear |

Verdicts on each line (classifier result):
- nested_substitution: Some(UnseenCode)
- alias_chain: Some(UnseenCode)
- arithmetic: None
- glob_alternatives: None
- nested_quotes: Some(UnseenCode)
